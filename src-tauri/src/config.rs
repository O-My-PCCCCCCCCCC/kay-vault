use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub auto_lock_minutes: u32,
    pub categories: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_lock_minutes: 5,
            categories: vec![
                "社交账号".into(),
                "开发工具".into(),
                "金融支付".into(),
                "邮箱".into(),
                "娱乐游戏".into(),
                "其他".into(),
            ],
        }
    }
}

pub fn load_config(path: &str) -> AppConfig {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_config(path: &str, config: &AppConfig) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    let json =
        serde_json::to_string_pretty(config).map_err(|e| format!("序列化失败: {}", e))?;
    std::fs::write(path, json).map_err(|e| format!("写入失败: {}", e))
}
