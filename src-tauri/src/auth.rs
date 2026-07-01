use rand::RngCore;
use std::path::{Path, PathBuf};

const AUTH_FILE: &str = ".vault_auth";

fn auth_file_path(backup_root: &str) -> PathBuf {
    PathBuf::from(backup_root).join(AUTH_FILE)
}

/// 检查本机是否已认证（密钥文件是否存在）
pub fn is_authorized_with_path(backup_root: &str) -> bool {
    auth_file_path(backup_root).exists()
}

/// 保留旧版接口兼容（默认路径）
pub fn is_authorized() -> bool {
    is_authorized_with_path("C:/LuSh-Password-Backup")
}

/// 生成本机认证密钥（自动创建文件夹和密钥文件）
pub fn generate_key_with_path(backup_root: &str) -> Result<(), String> {
    let root = Path::new(backup_root);
    std::fs::create_dir_all(root).map_err(|e| format!("创建目录失败: {}", e))?;

    // 生成随机 token 作为密钥
    let mut token = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut token);
    let token_hex = hex::encode(&token);

    std::fs::write(auth_file_path(backup_root), &token_hex)
        .map_err(|e| format!("写入密钥文件失败: {}", e))?;

    Ok(())
}

/// 保留旧版接口兼容
pub fn generate_key() -> Result<(), String> {
    generate_key_with_path("C:/LuSh-Password-Backup")
}

/// 删除本机认证
pub fn remove_auth_with_path(backup_root: &str) -> Result<(), String> {
    let path = auth_file_path(backup_root);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("删除认证失败: {}", e))?;
        Ok(())
    } else {
        Err("未找到认证密钥文件".into())
    }
}

/// 保留旧版接口兼容
pub fn remove_auth() -> Result<(), String> {
    remove_auth_with_path("C:/LuSh-Password-Backup")
}
