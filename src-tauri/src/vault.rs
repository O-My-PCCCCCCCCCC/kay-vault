use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub id: String,
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub notes: String,
    pub group: String,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultFile {
    pub entries: Vec<VaultEntry>,
}

impl VaultFile {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

/// 从磁盘加载并解密密码库（使用预派生的密钥）
pub fn load_vault(path: &str, key: &[u8]) -> Result<VaultFile, String> {
    let data = std::fs::read(path).map_err(|e| format!("读取文件失败: {}", e))?;
    if data.len() < 32 {
        return Err("密码库文件损坏".into());
    }
    let (_salt, encrypted) = data.split_at(32);
    let plaintext = crate::crypto::decrypt(encrypted, key)?;
    serde_json::from_slice(&plaintext).map_err(|e| format!("解析密码库失败: {}", e))
}

/// 加密并保存密码库到磁盘（使用预派生的密钥）
pub fn save_vault(path: &str, vault: &VaultFile, key: &[u8]) -> Result<(), String> {
    let plaintext = serde_json::to_vec(vault).map_err(|e| format!("序列化失败: {}", e))?;
    let salt = crate::crypto::generate_salt(); // 保持文件格式兼容
    let encrypted = crate::crypto::encrypt(&plaintext, key);

    let mut data = Vec::with_capacity(32 + encrypted.len());
    data.extend_from_slice(&salt);
    data.extend_from_slice(&encrypted);

    // 确保目录存在
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::write(path, &data).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
