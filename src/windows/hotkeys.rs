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

impl HotkeyAction {
    /// Format HotkeyAction to a user-friendly string
    pub fn format(&self) -> String {
        match self {
            HotkeyAction::IncreaseGamma => "Increase gamma".to_string(),
            HotkeyAction::DecreaseGamma => "Decrease gamma".to_string(),
            HotkeyAction::IncreaseBrightness => "Increase brightness".to_string(),
            HotkeyAction::DecreaseBrightness => "Decrease brightness".to_string(),
            HotkeyAction::IncreaseContrast => "Increase contrast".to_string(),
            HotkeyAction::DecreaseContrast => "Decrease contrast".to_string(),
            HotkeyAction::Reset => "Reset to default".to_string(),
            HotkeyAction::LoadProfile(index) => format!("Load profile {}", index + 1),
        }
    }
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

    /// Format KeybindConfig to a user-friendly string
    pub fn format(&self) -> String {
        let mods = self.modifiers.join(" + ");

        if mods.is_empty() {
            return self.key.clone();
        }

        format!("{} + {}", mods, self.key)
    }

    /// Normalize a single key string to standard representation
    pub fn normalize_key(key: &str) -> String {
        match key {
            // Function keys
            "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11"
            | "F12" => key.to_uppercase(),
            // Arrow keys
            "ArrowUp" => "UP".to_string(),
            "ArrowDown" => "DOWN".to_string(),
            "ArrowLeft" => "LEFT".to_string(),
            "ArrowRight" => "RIGHT".to_string(),
            // Special keys
            "PageUp" => "PAGEUP".to_string(),
            "PageDown" => "PAGEDOWN".to_string(),
            "Home" => "HOME".to_string(),
            "End" => "END".to_string(),
            "Insert" => "INSERT".to_string(),
            "Delete" => "DELETE".to_string(),
            "Backspace" => "BACKSPACE".to_string(),
            "Enter" => "RETURN".to_string(),
            "Tab" => "TAB".to_string(),
            "Space" => "SPACE".to_string(),
            // Alphanumeric keys
            k if k.len() == 1 && k.chars().next().unwrap().is_alphanumeric() => k.to_uppercase(),
            "+" => "PLUS".to_string(),
            "-" => "MINUS".to_string(),
            _ => String::new(),
        }
    }

    /// Normalize key using both key and code to handle numpad and digit keys correctly
    pub fn normalize_key_with_code(key: &str, code: &str) -> String {
        // Check for numpad keys first using code
        if code.starts_with("Numpad") {
            let numpad_key = code.strip_prefix("Numpad").unwrap();
            match numpad_key {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    return format!("Numpad{}", numpad_key);
                }
                "Add" => return "NumpadPLUS".to_string(),
                "Subtract" => return "NumpadMINUS".to_string(),
                "Multiply" => return "NumpadMULTIPLY".to_string(),
                "Divide" => return "NumpadDIVIDE".to_string(),
                "Decimal" => return "NumpadDECIMAL".to_string(),
                _ => {}
            }
        }

        // For digit keys, use the code to get the actual digit
        // This handles Shift+Number combinations where key returns "!" instead of "1"
        if code.starts_with("Digit") {
            if let Some(digit) = code.strip_prefix("Digit") {
                return digit.to_string();
            }
        }

        // For key codes like "KeyA", extract the letter
        if code.starts_with("Key") && code.len() == 4 {
            if let Some(letter) = code.strip_prefix("Key") {
                return letter.to_uppercase();
            }
        }

        // Fall back to regular key normalization
        Self::normalize_key(key)
    }
}
