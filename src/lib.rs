use crate::{
    profiles::ProfileManager,
    windows::{
        display::DisplaySettings,
        hotkeys::{HotkeyAction, KeybindConfig},
    },
};
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fs, path::PathBuf};

pub mod components;
pub mod profiles;
pub mod tabs;
pub mod windows;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub current_settings: DisplaySettings,
    pub step_size: StepSize,
    #[serde(
        serialize_with = "serialize_keybinds",
        deserialize_with = "deserialize_keybinds"
    )]
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

/// Serialize keybinds HashMap with HotkeyAction keys as string keys in JSON
fn serialize_keybinds<S>(
    keybinds: &HashMap<HotkeyAction, KeybindConfig>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(keybinds.len()))?;

    for (action, config) in keybinds {
        let key = action_to_string(action);
        map.serialize_entry(&key, config)?;
    }

    map.end()
}

/// Deserialize keybinds from JSON with string keys back to HotkeyAction keys
fn deserialize_keybinds<'de, D>(
    deserializer: D,
) -> Result<HashMap<HotkeyAction, KeybindConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_map: HashMap<String, KeybindConfig> = HashMap::deserialize(deserializer)?;
    let mut keybinds = HashMap::new();

    for (key_str, config) in string_map {
        if let Some(action) = string_to_action(&key_str) {
            keybinds.insert(action, config);
        }
    }

    Ok(keybinds)
}

/// Convert HotkeyAction to a string representation
fn action_to_string(action: &HotkeyAction) -> String {
    match action {
        HotkeyAction::IncreaseGamma => "IncreaseGamma".to_string(),
        HotkeyAction::DecreaseGamma => "DecreaseGamma".to_string(),
        HotkeyAction::IncreaseBrightness => "IncreaseBrightness".to_string(),
        HotkeyAction::DecreaseBrightness => "DecreaseBrightness".to_string(),
        HotkeyAction::IncreaseContrast => "IncreaseContrast".to_string(),
        HotkeyAction::DecreaseContrast => "DecreaseContrast".to_string(),
        HotkeyAction::Reset => "Reset".to_string(),
        HotkeyAction::LoadProfile(index) => format!("LoadProfile({})", index),
    }
}

/// Convert string back to HotkeyAction
fn string_to_action(s: &str) -> Option<HotkeyAction> {
    match s {
        "IncreaseGamma" => Some(HotkeyAction::IncreaseGamma),
        "DecreaseGamma" => Some(HotkeyAction::DecreaseGamma),
        "IncreaseBrightness" => Some(HotkeyAction::IncreaseBrightness),
        "DecreaseBrightness" => Some(HotkeyAction::DecreaseBrightness),
        "IncreaseContrast" => Some(HotkeyAction::IncreaseContrast),
        "DecreaseContrast" => Some(HotkeyAction::DecreaseContrast),
        "Reset" => Some(HotkeyAction::Reset),
        s if s.starts_with("LoadProfile(") && s.ends_with(')') => {
            let index_str = &s[12..s.len() - 1];
            index_str
                .parse::<usize>()
                .ok()
                .map(HotkeyAction::LoadProfile)
        }
        _ => None,
    }
}
