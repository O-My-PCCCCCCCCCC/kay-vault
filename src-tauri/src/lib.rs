mod crypto;
mod vault;
mod auth;
mod backup;
mod config;
mod api_keys;
pub mod sha_pin;

use tauri::Manager;
use std::path::PathBuf;

fn vault_path() -> PathBuf {
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
fn vault_load(password: String) -> Result<Vec<vault::VaultEntry>, String> {
    let path = vault_path();
    if !path.exists() { return Ok(Vec::new()); }
    let vault_file = vault::load_vault(path.to_str().unwrap(), &password)?;
    Ok(vault_file.entries)
}

#[tauri::command]
fn vault_save(entries: Vec<vault::VaultEntry>, password: String) -> Result<(), String> {
    let path = vault_path();
    let vault_file = vault::VaultFile { entries };
    vault::save_vault(path.to_str().unwrap(), &vault_file, &password)
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

#[tauri::command]
fn auth_generate_key() -> Result<(), String> { auth::generate_key() }
#[tauri::command]
fn auth_check() -> bool { auth::is_authorized() }
#[tauri::command]
fn auth_remove() -> Result<(), String> { auth::remove_auth() }

#[tauri::command]
fn backup_now() -> Result<String, String> {
    backup::backup_vault(vault_path().to_str().unwrap())
}

#[tauri::command]
fn restore_from_usb(filename: Option<String>) -> Result<(), String> {
    backup::restore_vault(vault_path().to_str().unwrap(), filename.as_deref())
}

#[tauri::command]
fn list_backups() -> Result<Vec<String>, String> { backup::list_backups() }

/// 系统统计
#[tauri::command]
fn get_stats(password: String) -> Result<serde_json::Value, String> {
    let vp = vault_path();
    let password_count = if vp.exists() {
        match vault::load_vault(vp.to_str().unwrap(), &password) {
            Ok(v) => v.entries.len(), Err(_) => 0,
        }
    } else { 0 };

    let api_count = match api_keys::load_keys(&password) {
        Ok(keys) => keys.len(), Err(_) => 0,
    };

    let (disk_total, disk_avail) = get_disk_info();
    let disk_used = disk_total.saturating_sub(disk_avail);
    let disk_percent = if disk_total > 0 { (disk_used as f64 / disk_total as f64 * 100.0).round() as u64 } else { 0 };

    Ok(serde_json::json!({
        "password_count": password_count,
        "api_count": api_count,
        "disk_total": disk_total,
        "disk_avail": disk_avail,
        "disk_used": disk_used,
        "disk_percent": disk_percent,
    }))
}

fn get_disk_info() -> (u64, u64) {
    #[cfg(windows)] {
        let out = std::process::Command::new("powershell")
            .args(["-Command", "Get-PSDrive C | Select-Object Used,Free | ConvertTo-Json"])
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
fn api_keys_load(password: String) -> Result<Vec<api_keys::ApiKey>, String> {
    api_keys::load_keys(&password)
}

#[tauri::command]
fn api_keys_save(keys: Vec<api_keys::ApiKey>, password: String) -> Result<(), String> {
    api_keys::save_keys(&keys, &password)
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
        ])
        .setup(|app| {
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
