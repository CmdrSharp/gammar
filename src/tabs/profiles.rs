use crate::{
    profiles::Profile,
    tabs::settings::find_monitor,
    windows::display::{apply_display_settings_to_monitor, MonitorInfo},
    AppConfig,
};
use dioxus::prelude::*;

#[component]
pub fn ProfilesTab(mut config: Signal<AppConfig>, monitors: Signal<Vec<MonitorInfo>>) -> Element {
    let mut new_profile_name = use_signal(String::new);

    rsx! {
        div {
            class: "profiles-tab",
            h2 { "Display profiles" }

            div {
                class: "new-profile",
                h3 { "Create new profile" }
                input {
                    r#type: "text",
                    placeholder: "Profile name",
                    value: "{new_profile_name}",
                    oninput: move |evt| new_profile_name.set(evt.value())
                }
                button {
                    onclick: move |_| {
                        let name = new_profile_name();
                        if !name.is_empty() {
                            let current_settings = config.read().current_settings;
                            let profile = Profile::new(name, current_settings);
                            config.write().profile_manager.add_profile(profile);
                            let _ = config.read().save();
                            new_profile_name.set(String::new());
                        }
                    },
                    "Save current settings as profile"
                }
            }

            h3 { "Saved profiles" }
            div {
                class: "profiles-list",
                {
                    let profiles = config.read().profile_manager.get_profiles().to_vec();

                    if profiles.is_empty() {
                        rsx! { p { class: "empty", "No profiles yet. Create one above!" } }
                    } else {
                        rsx! {
                            for (index , profile) in profiles.iter().enumerate() {
                                {
                                    let profile_settings = profile.settings;
                                    rsx! {
                                        div {
                                            key: "{index}",
                                            class: "profile-item",
                                            div {
                                                class: "profile-info",
                                                h4 { "{profile.name}" }
                                                p { "Gamma: {profile.settings.gamma:.2}, Brightness: {profile.settings.brightness:.2}, Contrast: {profile.settings.contrast:.2}" }
                                            }
                                            div {
                                                class: "profile-actions",
                                                button {
                                                    onclick: move |_| {
                                                        config.write().current_settings = profile_settings;

                                                        let monitors_list = monitors();
                                                        let selected_id = config.read().selected_monitor_id.clone();

                                                        if let Some(monitor) = find_monitor(&monitors_list, Some(selected_id.as_str())) {
                                                            let _ = apply_display_settings_to_monitor(profile_settings, &monitor);
                                                        }

                                                        let _ = config.read().save();
                                                    },
                                                    "Apply"
                                                }
                                                button {
                                                    class: "delete",
                                                    onclick: move |_| {
                                                        config.write().profile_manager.remove_profile(index);
                                                        let _ = config.read().save();
                                                    },
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
