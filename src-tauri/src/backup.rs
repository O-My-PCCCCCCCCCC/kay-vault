use chrono::Local;
use std::path::PathBuf;

const USB_BACKUPS_DIR: &str = ".key-vault/backups";

/// 备份加密密码库到 U 盘
pub fn backup_vault(vault_path: &str, usb_path: &str) -> Result<String, String> {
    if !crate::auth::is_device_authorized(usb_path)? {
        return Err("本设备未认证，无法备份".into());
    }
    if !std::path::Path::new(vault_path).exists() {
        return Err("密码库文件不存在".into());
    }

    let backup_dir = PathBuf::from(usb_path).join(USB_BACKUPS_DIR);
    std::fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败: {}", e))?;

    let date_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_name = format!("vault-{}.enc", date_str);
    let backup_path = backup_dir.join(&backup_name);

    std::fs::copy(vault_path, &backup_path).map_err(|e| format!("备份失败: {}", e))?;

    Ok(backup_name)
}

/// 从 U 盘还原密码库到本机
pub fn restore_vault(
    usb_path: &str,
    target_path: &str,
    backup_filename: Option<&str>,
) -> Result<(), String> {
    if !crate::auth::is_device_authorized(usb_path)? {
        return Err("本设备未认证，无法还原".into());
    }

    let backup_dir = PathBuf::from(usb_path).join(USB_BACKUPS_DIR);
    if !backup_dir.exists() {
        return Err("U 盘上没有备份".into());
    }

    let source = match backup_filename {
        Some(name) => {
            let p = backup_dir.join(name);
            if !p.exists() {
                return Err(format!("备份文件 '{}' 不存在", name));
            }
            p
        }
        None => {
            let mut entries: Vec<_> = std::fs::read_dir(&backup_dir)
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
