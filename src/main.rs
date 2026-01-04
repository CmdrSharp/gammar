#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::{
    desktop::{tao, window, HotKeyState, LogicalSize},
    prelude::*,
};
use gammar::{
    components::header::{Header, Tab},
    tabs::{
        keybinds::KeybindsTab,
        profiles::ProfilesTab,
        settings::{find_monitor, SettingsTab},
    },
    windows::{
        display::{apply_display_settings_to_monitor, enumerate_monitors, DisplaySettings},
        hotkeys::HotkeyAction,
    },
    AppConfig,
};
use global_hotkey::hotkey::HotKey;
use std::str::FromStr;

const MAIN_CSS: &str = include_str!("../assets/main.css");
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

fn main() {
    let icon = match image::load_from_memory(ICON_BYTES) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            tao::window::Icon::from_rgba(rgba.into_raw(), width, height).ok()
        }
        Err(_) => None,
    };

    let mut window_builder = tao::window::WindowBuilder::new()
        .with_title("Gammar")
        .with_inner_size(LogicalSize::new(1300.0, 900.0))
        .with_min_inner_size(LogicalSize::new(1300.0, 900.0))
        .with_resizable(false);

    if let Some(icon) = icon {
        window_builder = window_builder.with_window_icon(Some(icon));
    }

    let window = window_builder;

    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_menu(None),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    // Load configuration
    let mut config = use_signal(AppConfig::load);

    // Track keybind version for re-registration when keybinds change
    let keybind_version = use_signal(|| 0);

    // Enumerate monitors
    let monitors = use_signal(enumerate_monitors);

    // Current tab
    let mut active_tab = use_signal(|| Tab::Settings);

    // Initialize selected monitor to primary if not set
    use_effect(move || {
        let monitors_list = monitors();
        let selected_id = config.read().selected_monitor_id.clone();

        if selected_id.is_empty() || !monitors_list.iter().any(|m| m.id == selected_id) {
            if let Some(primary_monitor) = find_monitor(&monitors_list, None) {
                config.write().selected_monitor_id = primary_monitor.id;
                let _ = config.read().save();
            }
        }
    });

    // Apply initial settings
    use_effect(move || {
        let monitors_list = monitors();
        let settings = config.read().current_settings;
        let selected_id = config.read().selected_monitor_id.clone();

        if let Some(monitor) = find_monitor(&monitors_list, Some(selected_id.as_str())) {
            let _ = apply_display_settings_to_monitor(settings, &monitor);
        }
    });

    // Register all keybinds - re-register when keybind_version changes
    use_effect(move || {
        let version = keybind_version();
        let keybinds = config.peek().keybinds.clone();

        println!("Registering keybinds (version {})", version);

        // Remove all existing shortcuts
        window().remove_all_shortcuts();

        // Register all current keybinds
        for (action, keybind) in keybinds.iter() {
            let action = *action;
            let shortcut = keybind.to_shortcut_string();

            // Parse the shortcut string into a HotKey
            let hotkey = match HotKey::from_str(&shortcut) {
                Ok(hk) => hk,
                Err(e) => {
                    println!("Failed to parse shortcut '{}': {:?}", shortcut, e);
                    continue;
                }
            };

            let result = window().create_shortcut(hotkey, move |state| {
                if state != HotKeyState::Pressed {
                    return;
                }

                let mut cfg = config.write();
                let step = cfg.step_size.clone();
                let mut settings = cfg.current_settings;

                match action {
                    HotkeyAction::IncreaseGamma => {
                        settings.gamma = (settings.gamma + step.gamma).min(3.0);
                    }
                    HotkeyAction::DecreaseGamma => {
                        settings.gamma = (settings.gamma - step.gamma).max(0.1);
                    }
                    HotkeyAction::IncreaseBrightness => {
                        settings.brightness = (settings.brightness + step.brightness).min(1.0);
                    }
                    HotkeyAction::DecreaseBrightness => {
                        settings.brightness = (settings.brightness - step.brightness).max(-1.0);
                    }
                    HotkeyAction::IncreaseContrast => {
                        settings.contrast = (settings.contrast + step.contrast).min(3.0);
                    }
                    HotkeyAction::DecreaseContrast => {
                        settings.contrast = (settings.contrast - step.contrast).max(0.1);
                    }
                    HotkeyAction::Reset => {
                        settings = DisplaySettings::default();
                    }
                    HotkeyAction::LoadProfile(index) => {
                        if let Some(profile) = cfg.profile_manager.get_profile(index) {
                            settings = profile.settings;
                        }
                    }
                }

                cfg.current_settings = settings;

                let monitors_list = monitors();
                let selected_id = cfg.selected_monitor_id.clone();

                if let Some(monitor) = find_monitor(&monitors_list, Some(selected_id.as_str())) {
                    let _ = apply_display_settings_to_monitor(settings, &monitor);
                }

                let _ = cfg.save();
            });

            match result {
                Ok(_) => println!("Registered shortcut: {} for {:?}", shortcut, action),
                Err(e) => println!("Failed to register shortcut {}: {:?}", shortcut, e),
            }
        }
    });

    rsx! {
        document::Style { {MAIN_CSS} }
        div {
            class: "app-container",

            Header { active_tab, on_tab_change: move |tab| active_tab.set(tab) }

            div { class: "content",
                match active_tab() {
                    Tab::Settings => rsx! { SettingsTab { config, monitors } },
                    Tab::Keybinds => rsx! { KeybindsTab { config, keybind_version } },
                    Tab::Profiles => rsx! { ProfilesTab { config, monitors } },
                }
            }
        }
    }
}
