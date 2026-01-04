use crate::{
    profiles::ProfileManager,
    windows::{
        display::DisplaySettings,
        hotkeys::{HotkeyAction, KeybindConfig},
    },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

pub mod components;
pub mod profiles;
pub mod tabs;
pub mod windows;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub current_settings: DisplaySettings,
    pub step_size: StepSize,
    pub keybinds: HashMap<HotkeyAction, KeybindConfig>,
    pub profile_manager: ProfileManager,
    pub selected_monitor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepSize {
    pub gamma: f32,
    pub brightness: f32,
    pub contrast: f32,
}

impl Default for StepSize {
    fn default() -> Self {
        Self {
            gamma: 0.1,
            brightness: 0.05,
            contrast: 0.1,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));

        path.push("gammar");
        fs::create_dir_all(&path).ok();
        path.push("config.json");

        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();

        if !path.exists() {
            return Self::default();
        }

        let Ok(contents) = fs::read_to_string(&path) else {
            return Self::default();
        };

        let Ok(config) = serde_json::from_str(&contents) else {
            return Self::default();
        };

        config
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        let json = serde_json::to_string_pretty(self)?;

        fs::write(path, json)
    }
}
