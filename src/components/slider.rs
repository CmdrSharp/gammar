use dioxus::prelude::*;

#[component]
pub fn Slider(
    label: String,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    on_change: EventHandler<f32>,
) -> Element {
    rsx! {
        div {
            class: "slider-container",
            label { "{label}: {value:.2}" }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<f32>() {
                        on_change.call(val);
                    }
                }
            }
        }
    }
}
