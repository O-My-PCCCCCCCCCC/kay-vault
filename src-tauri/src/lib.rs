mod crypto;
mod vault;
mod auth;
mod backup;
mod config;

#[tauri::command]
fn greet() -> String {
    "凯伊密码管家已就绪".into()
}

#[tauri::command]
fn crypto_encrypt(plaintext: Vec<u8>, password: String) -> Vec<u8> {
    let salt = crypto::generate_salt();
    let key = crypto::derive_key(&password, &salt);
    let encrypted = crypto::encrypt(&plaintext, &key);
    // 返回 salt(32B) + encrypted
    let mut result = Vec::with_capacity(32 + encrypted.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&encrypted);
    result
}

#[tauri::command]
fn crypto_decrypt(data: Vec<u8>, password: String) -> Result<Vec<u8>, String> {
    if data.len() < 32 {
        return Err("数据损坏".into());
    }
    let (salt, encrypted) = data.split_at(32);
    let key = crypto::derive_key(&password, salt);
    crypto::decrypt(encrypted, &key)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, crypto_encrypt, crypto_decrypt])
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
