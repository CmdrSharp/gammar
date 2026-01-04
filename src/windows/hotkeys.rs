use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HotkeyAction {
    IncreaseGamma,
    DecreaseGamma,
    IncreaseBrightness,
    DecreaseBrightness,
    IncreaseContrast,
    DecreaseContrast,
    Reset,
    LoadProfile(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindConfig {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl KeybindConfig {
    pub fn new(modifiers: Vec<String>, key: String) -> Self {
        Self { modifiers, key }
    }

    /// Convert to Dioxus shortcut format: "Ctrl+Shift+F1"
    pub fn to_shortcut_string(&self) -> String {
        let mut parts = self.modifiers.clone();

        parts.push(self.key.clone());
        parts.join("+")
    }
}
