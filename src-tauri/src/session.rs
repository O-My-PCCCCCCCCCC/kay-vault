use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const VERIFY_FILE: &str = "master.verify";
const SESSION_TTL: Duration = Duration::from_secs(300); // 5 分钟无人操作自动过期

fn verify_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".key-vault").join(VERIFY_FILE)
}

fn compute_verify_tag(key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.finalize().to_vec()
}

/// 获取 C 盘卷序列号作为设备指纹
pub fn get_machine_id() -> String {
    #[cfg(windows)]
    {
        if let Ok(out) = std::process::Command::new("powershell")
            .args([
                "-Command",
                "(Get-CimInstance -ClassName Win32_LogicalDisk -Filter 'DeviceID=\"C:\"').VolumeSerialNumber",
            ])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }
    #[cfg(not(windows))]
    {
        if let Ok(out) = std::process::Command::new("lsblk")
            .args(["-o", "SERIAL", "-n", "/dev/sda"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }
    // 保底：如果获取不到序列号，用 hostname
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".into())
}

pub struct SessionManager {
    sessions: Mutex<HashMap<String, SessionData>>,
}

struct SessionData {
    key: Vec<u8>,
    last_active: Instant,
    machine_id: String,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// 登录：校验密码，创建会话，返回 session_id
    pub fn login(&self, password: &str) -> Result<String, String> {
        let vp = verify_path();
        let (_salt, key) = if vp.exists() {
            // 已有校验文件 → 验证密码
            let data = std::fs::read(&vp).map_err(|e| format!("读取校验文件失败: {}", e))?;
            if data.len() < 64 {
                return Err("校验文件损坏".into());
            }
            let (stored_salt, stored_hmac) = data.split_at(32);
            let key = crate::crypto::derive_key(password, stored_salt);
            let computed = compute_verify_tag(&key);
            if computed.as_slice() != stored_hmac {
                return Err("主密码错误".into());
            }
            (stored_salt.to_vec(), key)
        } else {
            // 首次使用 → 创建校验标签
            let salt = crate::crypto::generate_salt();
            let key = crate::crypto::derive_key(password, &salt);
            let hmac = compute_verify_tag(&key);

            let mut data = Vec::with_capacity(64);
            data.extend_from_slice(&salt);
            data.extend_from_slice(&hmac);
            if let Some(parent) = vp.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("创建目录失败: {}", e))?;
            }
            std::fs::write(&vp, &data).map_err(|e| format!("写入校验文件失败: {}", e))?;
            (salt, key)
        };

        let session_id = uuid::Uuid::new_v4().to_string();
        let machine_id = get_machine_id();
        let now = Instant::now();
        self.sessions.lock().unwrap().insert(
            session_id.clone(),
            SessionData {
                key,
                last_active: now,
                machine_id,
            },
        );
        Ok(session_id)
    }

    /// 锁定：删除会话，密钥从内存消失
    pub fn lock(&self, session_id: &str) {
        self.sessions.lock().unwrap().remove(session_id);
    }

    /// 取派生密钥（只检查 TTL，设备指纹由心跳校验）
    pub fn get_key(&self, session_id: &str) -> Option<Vec<u8>> {
        let mut map = self.sessions.lock().unwrap();
        let data = map.get_mut(session_id)?;

        // 检查 TTL
        if Instant::now().duration_since(data.last_active) > SESSION_TTL {
            map.remove(session_id);
            return None;
        }

        // 更新活动时间
        data.last_active = Instant::now();
        Some(data.key.clone())
    }

    /// 心跳：刷新 TTL，重新校验设备指纹
    pub fn heartbeat(&self, session_id: &str) -> bool {
        let mut map = self.sessions.lock().unwrap();
        let data = match map.get_mut(session_id) {
            Some(d) => d,
            None => return false,
        };

        // 检查 TTL
        if Instant::now().duration_since(data.last_active) > SESSION_TTL {
            map.remove(session_id);
            return false;
        }

        // 重新校验设备指纹
        let current_machine = get_machine_id();
        if current_machine != data.machine_id {
            map.remove(session_id);
            return false;
        }

        data.last_active = Instant::now();
        true
    }

    /// 检查会话是否存活
    pub fn is_active(&self, session_id: &str) -> bool {
        let mut map = self.sessions.lock().unwrap();
        let data = match map.get_mut(session_id) {
            Some(d) => d,
            None => return false,
        };

        if Instant::now().duration_since(data.last_active) > SESSION_TTL {
            map.remove(session_id);
            return false;
        }
        true
    }

    /// 换密码：校验旧密码 → 用新密码重新加密所有数据
    pub fn change_password(
        &self,
        _session_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<String, String> {
        // 1. 校验旧密码并拿到旧密钥
        let vp = verify_path();
        let data = std::fs::read(&vp).map_err(|e| format!("读取校验文件失败: {}", e))?;
        if data.len() < 64 {
            return Err("校验文件损坏".into());
        }
        let (old_salt, stored_hmac) = data.split_at(32);
        let old_key = crate::crypto::derive_key(old_password, old_salt);
        let computed = compute_verify_tag(&old_key);
        if computed.as_slice() != stored_hmac {
            return Err("旧密码错误".into());
        }

        // 2. 读旧数据
        let vp_vault = crate::vault_path();
        let vault_data = if vp_vault.exists() {
            let raw = std::fs::read(&vp_vault).map_err(|e| format!("读取密码库失败: {}", e))?;
            if raw.len() >= 32 {
                let (_salt, encrypted) = raw.split_at(32);
                let plaintext = crate::crypto::decrypt(encrypted, &old_key)
                    .map_err(|_| "解密密码库失败")?;
                let vault: crate::vault::VaultFile =
                    serde_json::from_slice(&plaintext).map_err(|_| "解析密码库失败")?;
                Some((raw[..32].to_vec(), vault))
            } else {
                None
            }
        } else {
            None
        };

        let vp_apikeys = {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".key-vault").join("apikeys.enc")
        };
        let apikeys_data = if vp_apikeys.exists() {
            let raw =
                std::fs::read(&vp_apikeys).map_err(|e| format!("读取 API 密钥文件失败: {}", e))?;
            if raw.len() >= 32 {
                let (_salt, encrypted) = raw.split_at(32);
                let plaintext = crate::crypto::decrypt(encrypted, &old_key)
                    .map_err(|_| "解密 API 密钥失败")?;
                let keys: Vec<crate::api_keys::ApiKey> = serde_json::from_slice(&plaintext)
                    .map_err(|_| "解析 API 密钥失败")?;
                Some((raw[..32].to_vec(), keys))
            } else {
                None
            }
        } else {
            None
        };

        // 3. 新密码派生
        let new_salt = crate::crypto::generate_salt();
        let new_key = crate::crypto::derive_key(new_password, &new_salt);

        // 4. 写入新校验文件
        let new_hmac = compute_verify_tag(&new_key);
        let mut verify_data = Vec::with_capacity(64);
        verify_data.extend_from_slice(&new_salt);
        verify_data.extend_from_slice(&new_hmac);
        std::fs::write(&vp, &verify_data).map_err(|e| format!("写入校验文件失败: {}", e))?;

        // 5. 重新加密 vault
        if let Some((old_vault_salt, vault)) = vault_data {
            let vault_json =
                serde_json::to_vec(&vault).map_err(|e| format!("序列化密码库失败: {}", e))?;
            let encrypted = crate::crypto::encrypt(&vault_json, &new_key);
            let mut out = Vec::with_capacity(32 + encrypted.len());
            out.extend_from_slice(&old_vault_salt);
            out.extend_from_slice(&encrypted);
            std::fs::write(&vp_vault, &out).map_err(|e| format!("写入密码库失败: {}", e))?;
        }

        // 6. 重新加密 api_keys
        if let Some((old_apikeys_salt, keys)) = apikeys_data {
            let keys_json =
                serde_json::to_vec(&keys).map_err(|e| format!("序列化 API 密钥失败: {}", e))?;
            let encrypted = crate::crypto::encrypt(&keys_json, &new_key);
            let mut out = Vec::with_capacity(32 + encrypted.len());
            out.extend_from_slice(&old_apikeys_salt);
            out.extend_from_slice(&encrypted);
            std::fs::write(&vp_apikeys, &out).map_err(|e| format!("写入 API 密钥失败: {}", e))?;
        }

        // 7. 创建新会话
        let new_session_id = uuid::Uuid::new_v4().to_string();
        let now = Instant::now();
        let machine_id = get_machine_id();
        self.sessions.lock().unwrap().insert(
            new_session_id.clone(),
            SessionData {
                key: new_key,
                last_active: now,
                machine_id,
            },
        );

        Ok(new_session_id)
    }
}
