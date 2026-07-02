mod crypto;
mod vault;
mod auth;
mod backup;
mod config;
mod api_keys;
pub mod sha_pin;
mod session;

use tauri::Manager;
use std::path::PathBuf;
#[cfg(windows)] use std::os::windows::process::CommandExt;

pub fn vault_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".key-vault").join("vault.enc")
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".key-vault").join("config.json")
}

#[tauri::command]
fn greet() -> String { "凯伊密码管家已就绪".into() }

#[tauri::command]
fn vault_load(session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<Vec<vault::VaultEntry>, String> {
    let key = state.get_key(&session_id).ok_or("未登录或会话已过期")?;
    let path = vault_path();
    if !path.exists() { return Ok(Vec::new()); }
    let vault_file = vault::load_vault(path.to_str().unwrap(), &key)?;
    Ok(vault_file.entries)
}

#[tauri::command]
fn vault_save(entries: Vec<vault::VaultEntry>, session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<(), String> {
    let key = state.get_key(&session_id).ok_or("未登录或会话已过期")?;
    let path = vault_path();
    let vault_file = vault::VaultFile { entries };
    vault::save_vault(path.to_str().unwrap(), &vault_file, &key)
}

#[tauri::command]
fn config_load() -> config::AppConfig {
    let path = config_path();
    config::load_config(path.to_str().unwrap())
}

#[tauri::command]
fn config_save(cfg: config::AppConfig) -> Result<(), String> {
    let path = config_path();
    config::save_config(path.to_str().unwrap(), &cfg)
}

fn get_backup_path() -> String {
    let cp = config_path();
    if cp.exists() {
        config::load_config(cp.to_str().unwrap()).backup_path
    } else {
        "C:/LuSh-Password-Backup".into()
    }
}

#[tauri::command]
fn auth_generate_key() -> Result<(), String> {
    let bp = get_backup_path();
    auth::generate_key_with_path(&bp)
}
#[tauri::command]
fn auth_check() -> bool {
    let bp = get_backup_path();
    auth::is_authorized_with_path(&bp)
}
#[tauri::command]
fn auth_remove() -> Result<(), String> {
    let bp = get_backup_path();
    auth::remove_auth_with_path(&bp)
}

#[tauri::command]
fn backup_now() -> Result<String, String> {
    let bp = get_backup_path();
    backup::backup_vault(vault_path().to_str().unwrap(), &bp)
}

#[tauri::command]
fn restore_from_usb(filename: Option<String>) -> Result<(), String> {
    let bp = get_backup_path();
    backup::restore_vault(vault_path().to_str().unwrap(), &bp, filename.as_deref())
}

#[tauri::command]
fn list_backups() -> Result<Vec<String>, String> {
    let bp = get_backup_path();
    backup::list_backups(&bp)
}

/// 系统统计
#[tauri::command]
fn get_stats(session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<serde_json::Value, String> {
    let key = state.get_key(&session_id).ok_or("未登录或会话已过期")?;
    let vp = vault_path();
    let password_count = if vp.exists() {
        match vault::load_vault(vp.to_str().unwrap(), &key) {
            Ok(v) => v.entries.len(), Err(_) => 0,
        }
    } else { 0 };

    let ak_path = {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".key-vault").join("apikeys.enc")
    };
    let api_count = match api_keys::load_keys(ak_path.to_str().unwrap(), &key) {
        Ok(keys) => keys.len(), Err(_) => 0,
    };

    // 计算磁盘用量细分
    let (disk_total, disk_avail) = get_disk_info();
    let disk_used = disk_total.saturating_sub(disk_avail);

    // 密码数据 = vault.enc + apikeys.enc
    let vault_file = vault_path();
    let apikeys_file = {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".key-vault").join("apikeys.enc")
    };
    let mut password_data_bytes = 0u64;
    if vault_file.exists() {
        if let Ok(m) = std::fs::metadata(&vault_file) { password_data_bytes += m.len(); }
    }
    if apikeys_file.exists() {
        if let Ok(m) = std::fs::metadata(&apikeys_file) { password_data_bytes += m.len(); }
    }

    // 软件数据 = ~/.key-vault/ 目录总大小 - 密码文件
    let kvd = key_vault_dir();
    let vault_dir_total = if kvd.exists() { dir_size(&kvd) } else { 0 };
    let app_data_bytes = vault_dir_total.saturating_sub(password_data_bytes);

    // 其他数据 = 磁盘已用 - 软件数据 - 密码数据
    let other_bytes = disk_used.saturating_sub(app_data_bytes + password_data_bytes);

    let disk_percent = if disk_total > 0 { (disk_used as f64 / disk_total as f64 * 100.0).round() as u64 } else { 0 };

    Ok(serde_json::json!({
        "password_count": password_count,
        "api_count": api_count,
        "disk_total": disk_total,
        "disk_avail": disk_avail,
        "disk_used": disk_used,
        "disk_percent": disk_percent,
        "password_data_bytes": password_data_bytes,
        "app_data_bytes": app_data_bytes,
        "other_bytes": other_bytes,
    }))
}

fn key_vault_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".key-vault")
}

fn dir_size(path: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    total += dir_size(&entry.path());
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}

fn get_disk_info() -> (u64, u64) {
    #[cfg(windows)] {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let out = std::process::Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", "Get-PSDrive C | Select-Object Used,Free | ConvertTo-Json"])
            .output().ok();
        if let Some(o) = out {
            let s = String::from_utf8_lossy(&o.stdout);
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                let used = v["Used"].as_u64().unwrap_or(0);
                let free = v["Free"].as_u64().unwrap_or(0);
                let total = used + free;
                if total > 0 { return (total, free); }
            }
        }
        (0, 0)
    }
    #[cfg(not(windows))] {
        let out = std::process::Command::new("df").args(["-B1", "/"]).output().ok();
        if let Some(o) = out {
            for line in String::from_utf8_lossy(&o.stdout).lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let total = parts[1].parse::<u64>().unwrap_or(0);
                    let avail = parts[3].parse::<u64>().unwrap_or(0);
                    if total > 0 { return (total, avail); }
                }
            }
        }
        (0, 0)
    }
}

/// API Keys
#[tauri::command]
fn api_keys_load(session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<Vec<api_keys::ApiKey>, String> {
    let key = state.get_key(&session_id).ok_or("未登录或会话已过期")?;
    let ak_path = {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".key-vault").join("apikeys.enc")
    };
    api_keys::load_keys(ak_path.to_str().unwrap(), &key)
}

#[tauri::command]
fn api_keys_save(keys: Vec<api_keys::ApiKey>, session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<(), String> {
    let key = state.get_key(&session_id).ok_or("未登录或会话已过期")?;
    let ak_path = {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".key-vault").join("apikeys.enc")
    };
    api_keys::save_keys(&keys, ak_path.to_str().unwrap(), &key)
}

/// 打开 URL
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    let u = url.trim();
    if !u.starts_with("http://") && !u.starts_with("https://") {
        return Err("URL 必须以 http:// 或 https:// 开头".into());
    }
    #[cfg(target_os = "windows")] {
        std::process::Command::new("cmd").args(["/c", "start", u]).spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }
    #[cfg(target_os = "linux")] {
        std::process::Command::new("xdg-open").arg(u).spawn()
            .map_err(|e| format!("打开失败: {}", e))?;
    }
    Ok(())
}

/// 会话管理
#[tauri::command]
fn session_login(password: String, state: tauri::State<'_, session::SessionManager>) -> Result<String, String> {
    state.login(&password)
}

#[tauri::command]
fn session_lock(session_id: String, state: tauri::State<'_, session::SessionManager>) -> Result<(), String> {
    state.lock(&session_id);
    Ok(())
}

#[tauri::command]
fn session_status(session_id: String, state: tauri::State<'_, session::SessionManager>) -> bool {
    state.is_active(&session_id)
}

#[tauri::command]
fn session_heartbeat(session_id: String, state: tauri::State<'_, session::SessionManager>) -> bool {
    state.heartbeat(&session_id)
}

#[tauri::command]
fn session_change_password(session_id: String, old_password: String, new_password: String, state: tauri::State<'_, session::SessionManager>) -> Result<String, String> {
    state.change_password(&session_id, &old_password, &new_password)
}

/// 导入加密备份文件
#[tauri::command]
fn import_from_file(file_path: String, password: String) -> Result<(), String> {
    // 用 master.verify 的真实盐值派生密钥（文件中的盐值可能不准确）
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    let verify_path = PathBuf::from(home).join(".key-vault").join("master.verify");
    let verify_data = std::fs::read(&verify_path)
        .map_err(|_| String::from("未找到主密码校验文件，请先登录一次"))?;
    if verify_data.len() < 32 {
        return Err("校验文件损坏".into());
    }
    let real_salt = &verify_data[..32];

    // 用正确的盐派生密钥
    let key = crate::crypto::derive_key(&password, real_salt);

    // 读取备份文件，跳过盐值（不管文件里存的是什么盐）
    let data = std::fs::read(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    if data.len() < 32 {
        return Err("无效的备份文件".into());
    }
    let encrypted = &data[32..];

    // 解密验证
    let plaintext = crate::crypto::decrypt(encrypted, &key)
        .map_err(|_| String::from("密码错误。请输入创建该备份时所用的主密码。"))?;

    // 验证 JSON 格式
    let _: serde_json::Value = serde_json::from_slice(&plaintext)
        .map_err(|_| String::from("备份文件格式错误"))?;

    // 复制到 vault 路径
    let vp = vault_path();
    if let Some(parent) = vp.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::copy(&file_path, &vp).map_err(|e| format!("导入失败: {}", e))?;
    Ok(())
}

/// SHA-PIN 密码生成
#[tauri::command]
fn sha_pin_run(input1: String, input2: String, password_len: usize) -> Result<serde_json::Value, String> {
    let len = match password_len { 4 | 6 | 8 => password_len, _ => 6 };
    let result = sha_pin::compute_with_len(&input1, &input2, len)?;
    Ok(serde_json::json!({
        "forward_result": result.0, "reverse_result": result.1, "final_password": result.2,
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, vault_load, vault_save, config_load, config_save,
            auth_generate_key, auth_check, auth_remove,
            backup_now, restore_from_usb, list_backups,
            api_keys_load, api_keys_save, get_stats, sha_pin_run, open_url,
            session_login, session_lock, session_status, session_heartbeat, session_change_password, import_from_file,
        ])
        .setup(|app| {
            app.manage(session::SessionManager::new());
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build(),
                )?;
            }
            app.handle().plugin(
                tauri_plugin_single_instance::init(|app, _argv, _cwd| {
                    let _ = app.get_webview_window("main").map(|w| w.set_focus());
                }),
            )?;
            app.handle().plugin(tauri_plugin_dialog::init())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
