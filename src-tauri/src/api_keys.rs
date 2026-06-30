use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
    pub provider: String,
    pub base_url: String,
    pub created_at: String,
}

fn api_keys_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".key-vault").join("apikeys.enc")
}

pub fn load_keys(password: &str) -> Result<Vec<ApiKey>, String> {
    let path = api_keys_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    if data.len() < 32 {
        return Err("API密钥文件损坏".into());
    }
    let (salt, encrypted) = data.split_at(32);
    let key = crate::crypto::derive_key(password, salt);
    let plaintext = crate::crypto::decrypt(encrypted, &key)?;
    serde_json::from_slice(&plaintext).map_err(|e| format!("解析失败: {}", e))
}

pub fn save_keys(keys: &[ApiKey], password: &str) -> Result<(), String> {
    let plaintext = serde_json::to_vec(keys).map_err(|e| format!("序列化失败: {}", e))?;
    let salt = crate::crypto::generate_salt();
    let key = crate::crypto::derive_key(password, &salt);
    let encrypted = crate::crypto::encrypt(&plaintext, &key);

    let mut data = Vec::with_capacity(32 + encrypted.len());
    data.extend_from_slice(&salt);
    data.extend_from_slice(&encrypted);

    let path = api_keys_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::write(&path, &data).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
