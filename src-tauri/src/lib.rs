mod crypto;
mod vault;
mod auth;
mod backup;
mod config;

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
fn greet() -> String {
    "凯伊密码管家已就绪".into()
}

#[tauri::command]
fn vault_load(password: String) -> Result<Vec<vault::VaultEntry>, String> {
    let path = vault_path();
    if !path.exists() {
        return Ok(Vec::new()); // 首次使用，返回空库
    }
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
fn auth_generate_key(usb_path: String) -> Result<String, String> {
    auth::generate_device_key(&usb_path)
}

#[tauri::command]
fn auth_list_devices(usb_path: String) -> Result<Vec<String>, String> {
    auth::list_authorized_devices(&usb_path)
}

#[tauri::command]
fn auth_check(usb_path: String) -> Result<bool, String> {
    auth::is_device_authorized(&usb_path)
}

#[tauri::command]
fn auth_remove(usb_path: String, device_name: String) -> Result<(), String> {
    auth::remove_device_auth(&usb_path, &device_name)
}

#[tauri::command]
fn backup_now(usb_path: String) -> Result<String, String> {
    let vault_path = vault_path();
    let vault_str = vault_path.to_str().unwrap();
    backup::backup_vault(vault_str, &usb_path)
}

#[tauri::command]
fn restore_from_usb(usb_path: String, filename: Option<String>) -> Result<(), String> {
    let vault_path = vault_path();
    let vault_str = vault_path.to_str().unwrap();
    backup::restore_vault(&usb_path, vault_str, filename.as_deref())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            vault_load,
            vault_save,
            config_load,
            config_save,
            auth_generate_key,
            auth_list_devices,
            auth_check,
            auth_remove,
            backup_now,
            restore_from_usb,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
