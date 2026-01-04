use crate::{
    windows::hotkeys::{HotkeyAction, KeybindConfig},
    AppConfig,
};
use dioxus::prelude::*;

/// Normalize a single key string to standard representation
fn normalize_key(key: &str) -> String {
    match key {
        // Function keys
        "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12" => {
            key.to_uppercase()
        }
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
fn normalize_key_with_code(key: &str, code: &str) -> String {
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
    normalize_key(key)
}

/// Format HotkeyAction to a user-friendly string
pub fn format_action(action: HotkeyAction) -> String {
    match action {
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

/// Get action display name with profile context
fn get_action_name(action: HotkeyAction, config: &AppConfig) -> String {
    match action {
        HotkeyAction::LoadProfile(i) => {
            if let Some(profile) = config.profile_manager.get_profile(i) {
                format!("Load profile: {}", profile.name)
            } else {
                format_action(action)
            }
        }
        _ => format_action(action),
    }
}

/// Format KeybindConfig to a user-friendly string
pub fn format_keybind(keybind: &KeybindConfig) -> String {
    let mods = keybind.modifiers.join(" + ");

    if mods.is_empty() {
        return keybind.key.clone();
    }

    format!("{} + {}", mods, keybind.key)
}

/// Handler for key capture events
#[allow(clippy::too_many_arguments)]
fn handle_key_capture(
    key: String,
    code: String,
    mut captured_modifiers: Signal<Vec<String>>,
    mut captured_key: Signal<Option<String>>,
    mut editing_action: Signal<Option<HotkeyAction>>,
    mut recording_keys: Signal<bool>,
    mut config: Signal<AppConfig>,
    mut keybind_version: Signal<usize>,
) {
    // Handle ESC to cancel
    if key == "Escape" {
        editing_action.set(None);
        recording_keys.set(false);
        captured_modifiers.set(Vec::new());
        captured_key.set(None);
        return;
    }

    // Capture modifiers
    let mut mods = captured_modifiers();
    match key.as_str() {
        "Control" => {
            if !mods.contains(&"Ctrl".to_string()) {
                mods.push("Ctrl".to_string());
            }
        }
        "Shift" => {
            if !mods.contains(&"Shift".to_string()) {
                mods.push("Shift".to_string());
            }
        }
        "Alt" => {
            if !mods.contains(&"Alt".to_string()) {
                mods.push("Alt".to_string());
            }
        }
        "Meta" => {
            if !mods.contains(&"Win".to_string()) {
                mods.push("Win".to_string());
            }
        }
        _ => {
            // Non-modifier key - this is the main key
            let normalized_key = normalize_key_with_code(&key, &code);

            if !normalized_key.is_empty() {
                captured_key.set(Some(normalized_key.clone()));

                // Save the keybind
                if let Some(action) = editing_action() {
                    let new_keybind = KeybindConfig::new(mods.clone(), normalized_key);
                    config.write().keybinds.insert(action, new_keybind);
                    let _ = config.read().save();

                    keybind_version.set(keybind_version() + 1);
                }

                // Reset state
                editing_action.set(None);
                recording_keys.set(false);
                captured_modifiers.set(Vec::new());
                captured_key.set(None);

                return;
            }
        }
    }
    captured_modifiers.set(mods);
}

/// Component for rendering a single keybind row
#[component]
fn KeybindRow(
    action: HotkeyAction,
    mut config: Signal<AppConfig>,
    mut editing_action: Signal<Option<HotkeyAction>>,
    mut recording_keys: Signal<bool>,
    mut captured_modifiers: Signal<Vec<String>>,
    mut captured_key: Signal<Option<String>>,
    mut keybind_version: Signal<usize>,
) -> Element {
    let cfg = config.read();
    let keybind = cfg.keybinds.get(&action).cloned();
    let action_name = get_action_name(action, &cfg);
    let keybind_str = keybind
        .as_ref()
        .map(format_keybind)
        .unwrap_or_else(|| "Not set".to_string());
    let is_editing = editing_action() == Some(action);

    drop(cfg);

    rsx! {
        tr {
            key: "{action_name}",
            td { "{action_name}" }
            td {
                if is_editing && recording_keys() {
                    span { class: "recording", "Press keys... (ESC to cancel)" }
                } else {
                    code { "{keybind_str}" }
                }
            }
            td {
                if is_editing {
                    button {
                        class: "cancel-btn",
                        onclick: move |_| {
                            editing_action.set(None);
                            recording_keys.set(false);
                            captured_modifiers.set(Vec::new());
                            captured_key.set(None);
                        },
                        "Cancel"
                    }
                } else {
                    button {
                        class: "edit-btn",
                        onclick: move |_| {
                            editing_action.set(Some(action));
                            recording_keys.set(true);
                            captured_modifiers.set(Vec::new());
                            captured_key.set(None);
                        },
                        "Edit"
                    }
                    if keybind.is_some() {
                        button {
                            class: "delete-btn",
                            style: "margin-left: 5px; background: #e74c3c;",
                            onclick: move |_| {
                                config.write().keybinds.remove(&action);
                                let _ = config.read().save();
                                keybind_version.set(keybind_version() + 1);
                            },
                            "Clear"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn KeybindsTab(mut config: Signal<AppConfig>, mut keybind_version: Signal<usize>) -> Element {
    let editing_action = use_signal(|| Option::<HotkeyAction>::None);
    let recording_keys = use_signal(|| false);
    let captured_modifiers = use_signal(Vec::<String>::new);
    let captured_key = use_signal(|| Option::<String>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "keybinds-tab",
            h2 { "Keyboard shortcuts" }
            p { class: "info", "Click 'Edit' to change a keybind." }

            if let Some(err) = error_msg() {
                div {
                    class: "error-message",
                    style: "background: #e74c3c; color: white; padding: 10px; border-radius: 5px; margin-bottom: 15px;",
                    "Error: {err}"
                    button {
                        style: "margin-left: 10px; background: transparent; border: 1px solid white; color: white; padding: 5px 10px; cursor: pointer;",
                        onclick: move |_| error_msg.set(None),
                        "✕"
                    }
                }
            }

            table {
                class: "keybinds-table",
                thead {
                    tr {
                        th { "Action" }
                        th { "Current keybind" }
                        th { "Actions" }
                    }
                }
                tbody {
                    for action in {
                        use HotkeyAction::*;
                        vec![
                            IncreaseGamma,
                            DecreaseGamma,
                            IncreaseBrightness,
                            DecreaseBrightness,
                            IncreaseContrast,
                            DecreaseContrast,
                            Reset,
                        ]
                    } {
                        KeybindRow {
                            action,
                            config,
                            editing_action,
                            recording_keys,
                            captured_modifiers,
                            captured_key,
                            keybind_version,
                        }
                    }
                }
            }

            // Profile keybinds section
            if config.read().profile_manager.profile_count() > 0 {
                h3 { style: "margin-top: 30px;", "Profile Shortcuts" }
                table {
                    class: "keybinds-table",
                    thead {
                        tr {
                            th { "Profile" }
                            th { "Current keybind" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for i in 0..config.read().profile_manager.profile_count() {
                            KeybindRow {
                                action: HotkeyAction::LoadProfile(i),
                                config,
                                editing_action,
                                recording_keys,
                                captured_modifiers,
                                captured_key,
                                keybind_version,
                            }
                        }
                    }
                }
            }

            if recording_keys() {
                div {
                    class: "key-capture-overlay",
                    id: "key-capture-overlay",
                    tabindex: 0,
                    onmounted: move |_| {
                        spawn(async move {
                            document::eval(
                                r#"document.getElementById('key-capture-overlay')?.focus();"#
                            );
                        });
                    },
                    onclick: move |evt| {
                        evt.stop_propagation();
                    },
                    onkeydown: move |evt| {
                        evt.prevent_default();
                        let key = evt.key().to_string();
                        let code = evt.code().to_string();
                        handle_key_capture(
                            key,
                            code,
                            captured_modifiers,
                            captured_key,
                            editing_action,
                            recording_keys,
                            config,
                            keybind_version,
                        );
                    },

                    div { class: "key-capture-box",
                        h3 { "Press your key combination" }
                        p {
                            if !captured_modifiers().is_empty() {
                                "{captured_modifiers().join(\" + \")} + ..."
                            } else {
                                "Waiting for keys..."
                            }
                        }
                        p { class: "hint-text", "Press ESC to cancel" }
                    }
                }
            }

            p { class: "hint", "Note: Keybinds are captured globally and not propagated to other applications. Avoid using shortcuts that may interfere with other software." }
        }
    }
}
