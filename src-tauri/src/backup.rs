use chrono::Local;
use std::path::PathBuf;

const BACKUP_ROOT: &str = "C:/LuSh-Password-Backup";

fn backup_root() -> PathBuf {
    PathBuf::from(BACKUP_ROOT)
}

/// 确保备份目录存在
pub fn ensure_backup_dir() -> Result<(), String> {
    let root = backup_root();
    std::fs::create_dir_all(&root).map_err(|e| format!("创建备份目录失败: {}", e))
}

/// 备份加密密码库到 C:/LuSh-Password-Backup
pub fn backup_vault(vault_path: &str) -> Result<String, String> {
    // 先确保目录存在
    ensure_backup_dir()?;

    if !crate::auth::is_authorized() {
        return Err("本机未认证，无法备份。请先在「设置 → 设备认证」中添加认证。".into());
    }
    if !std::path::Path::new(vault_path).exists() {
        return Err("密码库文件不存在".into());
    }

    let root = backup_root();
    let date_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_name = format!("vault-{}.enc", date_str);
    let backup_path = root.join(&backup_name);

    std::fs::copy(vault_path, &backup_path).map_err(|e| format!("备份失败: {}", e))?;

    Ok(backup_name)
}

/// 从 C:/LuSh-Password-Backup 还原密码库到本机
pub fn restore_vault(target_path: &str, backup_filename: Option<&str>) -> Result<(), String> {
    // 先确保目录存在
    ensure_backup_dir()?;

    if !crate::auth::is_authorized() {
        return Err("本机未认证，无法还原。请先在「设置 → 设备认证」中添加认证。".into());
    }

    let root = backup_root();
    let source = match backup_filename {
        Some(name) => {
            let p = root.join(name);
            if !p.exists() {
                return Err(format!("备份文件 '{}' 不存在", name));
            }
            p
        }
        None => {
            let mut entries: Vec<_> = std::fs::read_dir(&root)
                .map_err(|e| format!("读取备份目录失败: {}", e))?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name()
                        .to_str()
                        .map_or(false, |n| n.ends_with(".enc"))
                })
                .collect();
            entries.sort_by_key(|e| e.file_name());
            match entries.into_iter().last() {
                Some(e) => e.path(),
                None => return Err("没有找到备份文件".into()),
            }
        }
    };

    if let Some(parent) = std::path::Path::new(target_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::copy(&source, target_path).map_err(|e| format!("还原失败: {}", e))?;
    Ok(())
}

/// 列出所有备份文件
pub fn list_backups() -> Result<Vec<String>, String> {
    let root = backup_root();
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut backups = Vec::new();
    let entries = std::fs::read_dir(&root).map_err(|e| format!("读取备份目录失败: {}", e))?;
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".enc") {
                    backups.push(name.to_string());
                }
            }
        }
    }
    backups.sort();
    backups.reverse();
    Ok(backups)
}
