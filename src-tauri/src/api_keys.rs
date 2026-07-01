use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
    pub provider: String,
    pub base_url: String,
    pub created_at: String,
}

pub fn load_keys(path: &str, key: &[u8]) -> Result<Vec<ApiKey>, String> {
    if !std::path::Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read(path).map_err(|e| format!("读取文件失败: {}", e))?;
    if data.len() < 32 {
        return Err("API密钥文件损坏".into());
    }
    let (_salt, encrypted) = data.split_at(32);
    let plaintext = crate::crypto::decrypt(encrypted, key)?;
    serde_json::from_slice(&plaintext).map_err(|e| format!("解析失败: {}", e))
}

pub fn save_keys(keys: &[ApiKey], path: &str, key: &[u8]) -> Result<(), String> {
    let plaintext = serde_json::to_vec(keys).map_err(|e| format!("序列化失败: {}", e))?;
    let salt = crate::crypto::generate_salt(); // 保持文件格式兼容
    let encrypted = crate::crypto::encrypt(&plaintext, key);

    let mut data = Vec::with_capacity(32 + encrypted.len());
    data.extend_from_slice(&salt);
    data.extend_from_slice(&encrypted);

    let file_path = std::path::Path::new(path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::write(path, &data).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}
