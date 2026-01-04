use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Settings,
    Keybinds,
    Profiles,
}

#[component]
pub fn Header(active_tab: Signal<Tab>, on_tab_change: EventHandler<Tab>) -> Element {
    rsx! {
        header {
            class: "header",
            h1 { "Gammar" }
            nav {
                class: "tabs",
                button {
                    class: if active_tab() == Tab::Settings { "tab active" } else { "tab" },
                    onclick: move |_| on_tab_change.call(Tab::Settings),
                    "Settings"
                }
                button {
                    class: if active_tab() == Tab::Keybinds { "tab active" } else { "tab" },
                    onclick: move |_| on_tab_change.call(Tab::Keybinds),
                    "Keybinds"
                }
                button {
                    class: if active_tab() == Tab::Profiles { "tab active" } else { "tab" },
                    onclick: move |_| on_tab_change.call(Tab::Profiles),
                    "Profiles"
                }
            }
        }
    }
}
