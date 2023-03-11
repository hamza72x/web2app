use serde::{Deserialize, Serialize};

// $HOME/.config/nativefier_tauri_apps/app_name/config.json
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub zoom_factor: f64,

    #[serde(default)]
    pub dark_reader_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            zoom_factor: 1.0,
            dark_reader_enabled: false,
        };
    }
}

impl AppConfig {
    fn config_dir() -> std::path::PathBuf {
        return home::home_dir()
            .unwrap()
            .join(".config")
            .join("nativefier_tauri_apps")
            .join("app_name");
    }

    fn config_path() -> std::path::PathBuf {
        return Self::config_dir().join("config.json");
    }

    fn create_config_dir() {
        let config_dir = Self::config_dir();

        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }
    }

    pub fn load() -> Option<Self> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            Self::create_config_dir();
            return None;
        }

        let config_file = std::fs::File::open(config_path).unwrap();
        let config: Self = serde_json::from_reader(config_file).unwrap();

        return Some(config);
    }

    pub fn save(&self) {
        let config_path = Self::config_path();

        let config_file = std::fs::File::create(config_path).unwrap();
        serde_json::to_writer_pretty(config_file, self).unwrap();
    }

    pub fn dark_reader_text(&self) -> &'static str {
        if self.dark_reader_enabled {
            return "Disable DarkReader";
        } else {
            return "Enable DarkReader";
        }
    }
}