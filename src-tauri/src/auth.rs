use rand::RngCore;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const USB_KEYS_DIR: &str = ".key-vault/keys";

/// 获取本机唯一标识
fn get_machine_id() -> String {
    let hostname = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".into());
    let mut hasher = Sha256::new();
    hasher.update(hostname.as_bytes());
    format!("PC-{}", hex::encode(hasher.finalize())[..8].to_uppercase())
}

/// 生成本机设备密钥写入 U 盘
pub fn generate_device_key(usb_path: &str) -> Result<String, String> {
    let device_name = get_machine_id();
    let keys_dir = PathBuf::from(usb_path).join(USB_KEYS_DIR);
    std::fs::create_dir_all(&keys_dir).map_err(|e| format!("创建密钥目录失败: {}", e))?;

    // 生成随机 token 作为密钥
    let mut token = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut token);
    let token_hex = hex::encode(&token);

    let key_path = keys_dir.join(format!("{}.key", &device_name));
    std::fs::write(&key_path, &token_hex).map_err(|e| format!("写入密钥失败: {}", e))?;

    Ok(device_name)
}

/// 列出 U 盘上所有已认证设备
pub fn list_authorized_devices(usb_path: &str) -> Result<Vec<String>, String> {
    let keys_dir = PathBuf::from(usb_path).join(USB_KEYS_DIR);
    if !keys_dir.exists() {
        return Ok(Vec::new());
    }
    let mut devices = Vec::new();
    let entries =
        std::fs::read_dir(&keys_dir).map_err(|e| format!("读取密钥目录失败: {}", e))?;
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".key") {
                    devices.push(name.trim_end_matches(".key").to_string());
                }
            }
        }
    }
    Ok(devices)
}

/// 检查本机是否在 U 盘上有认证
pub fn is_device_authorized(usb_path: &str) -> Result<bool, String> {
    let device_name = get_machine_id();
    let key_path = PathBuf::from(usb_path)
        .join(USB_KEYS_DIR)
        .join(format!("{}.key", &device_name));
    Ok(key_path.exists())
}

/// 删除指定设备认证
pub fn remove_device_auth(usb_path: &str, device_name: &str) -> Result<(), String> {
    if device_name.contains("..") || device_name.contains('/') || device_name.contains('\\') {
        return Err("无效设备名".into());
    }
    let key_path = PathBuf::from(usb_path)
        .join(USB_KEYS_DIR)
        .join(format!("{}.key", device_name));
    if key_path.exists() {
        std::fs::remove_file(&key_path).map_err(|e| format!("删除失败: {}", e))?;
        Ok(())
    } else {
        Err("未找到该设备的认证密钥".into())
    }
}
