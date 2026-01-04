use crate::{
    components::slider::Slider,
    windows::display::{apply_display_settings_to_monitor, DisplaySettings, MonitorInfo},
    AppConfig,
};
use dioxus::prelude::*;

pub fn find_monitor(monitors: &[MonitorInfo], id: Option<&str>) -> Option<MonitorInfo> {
    if let Some(id) = id {
        return monitors.iter().find(|m| m.id == id).cloned();
    }

    monitors
        .iter()
        .find(|m| m.is_primary)
        .cloned()
        .or_else(|| monitors.first().cloned())
}

/// Apply settings and handle errors
fn apply_settings_update(
    settings: DisplaySettings,
    monitors: &[MonitorInfo],
    selected_id: &str,
    mut config: Signal<AppConfig>,
    mut error_msg: Signal<Option<String>>,
) {
    config.write().current_settings = settings;

    if let Some(monitor) = find_monitor(monitors, Some(selected_id)) {
        match apply_display_settings_to_monitor(settings, &monitor) {
            Ok(_) => {
                error_msg.set(None);
                let _ = config.read().save();
            }
            Err(e) => error_msg.set(Some(e.to_string())),
        }
    }
}

/// Update a display setting using a closure
fn update_display_setting<F>(
    config: Signal<AppConfig>,
    monitors: Signal<Vec<MonitorInfo>>,
    error_msg: Signal<Option<String>>,
    update_fn: F,
) where
    F: FnOnce(&mut DisplaySettings),
{
    let mut settings = config.read().current_settings;

    update_fn(&mut settings);

    let monitors_list = monitors();
    let selected_id = config.read().selected_monitor_id.clone();

    apply_settings_update(settings, &monitors_list, &selected_id, config, error_msg);
}

/// Update a step size setting and save
fn update_step_size<F>(mut config: Signal<AppConfig>, update_fn: F)
where
    F: FnOnce(&mut crate::StepSize),
{
    let mut step = config.read().step_size.clone();

    update_fn(&mut step);
    config.write().step_size = step;

    let _ = config.read().save();
}

#[component]
pub fn SettingsTab(mut config: Signal<AppConfig>, monitors: Signal<Vec<MonitorInfo>>) -> Element {
    let error_msg = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "settings-tab",
            div {
                class: "settings-card",
                div {
                    class: "card-header",
                    h2 { "Monitor selection" }
                    p {
                        class: "card-description",
                        "Choose which display to control"
                    }
                }
                select {
                    class: "monitor-select",
                    value: config.read().selected_monitor_id.clone(),
                    onchange: move |evt| {
                        let value = evt.value();
                        config.write().selected_monitor_id = value;
                        let _ = config.read().save();
                    },

                    for monitor in monitors().iter() {
                        option {
                            key: "{monitor.id}",
                            value: "{monitor.id}",
                            "{monitor.name}"
                        }
                    }
                }
            }

            if let Some(err) = error_msg() {
                div {
                    class: "error-message",
                    "⚠️ Error: {err}"
                }
            }

            // Display controls card
            div {
                class: "settings-card",
                div {
                    class: "card-header",
                    h2 { "Display controls" }
                    p {
                        class: "card-description",
                        "Adjust gamma, brightness, and contrast for your display"
                    }
                }

                div {
                    class: "sliders-grid",

                    Slider {
                        label: "Gamma",
                        value: config.read().current_settings.gamma,
                        min: 0.1,
                        max: 3.0,
                        step: 0.01,
                        on_change: move |value| {
                            update_display_setting(config, monitors, error_msg, |s| s.gamma = value);
                        }
                    }

                    Slider {
                        label: "Brightness",
                        value: config.read().current_settings.brightness,
                        min: -1.0,
                        max: 1.0,
                        step: 0.01,
                        on_change: move |value| {
                            update_display_setting(config, monitors, error_msg, |s| s.brightness = value);
                        }
                    }

                    Slider {
                        label: "Contrast",
                        value: config.read().current_settings.contrast,
                        min: 0.1,
                        max: 3.0,
                        step: 0.01,
                        on_change: move |value| {
                            update_display_setting(config, monitors, error_msg, |s| s.contrast = value);
                        }
                    }
                }

                button {
                    class: "reset-button",
                    onclick: move |_| {
                        let settings = DisplaySettings::default();
                        let monitors_list = monitors();
                        let selected_id = config.read().selected_monitor_id.clone();
                        apply_settings_update(settings, &monitors_list, &selected_id, config, error_msg);
                    },
                    "Reset to Default"
                }
            }

            // Hotkey step size card
            div {
                class: "settings-card",
                div {
                    class: "card-header",
                    h2 { "Step size" }
                    p {
                        class: "card-description",
                        "Configure how much each hotkey press adjusts the values"
                    }
                }

                div {
                    class: "sliders-grid",

                    Slider {
                        label: "Gamma step",
                        value: config.read().step_size.gamma,
                        min: 0.01,
                        max: 0.5,
                        step: 0.01,
                        on_change: move |value| {
                            update_step_size(config, |s| s.gamma = value);
                        }
                    }

                    Slider {
                        label: "Brightness step",
                        value: config.read().step_size.brightness,
                        min: 0.01,
                        max: 0.5,
                        step: 0.01,
                        on_change: move |value| {
                            update_step_size(config, |s| s.brightness = value);
                        }
                    }

                    Slider {
                        label: "Contrast step",
                        value: config.read().step_size.contrast,
                        min: 0.01,
                        max: 0.5,
                        step: 0.01,
                        on_change: move |value| {
                            update_step_size(config, |s| s.contrast = value);
                        }
                    }
                }
            }
        }
    }
}
