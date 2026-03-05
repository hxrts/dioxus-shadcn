//! Slider example components.

use dioxus::prelude::*;
use lumen_blocks::components::slider::Slider;
use lumen_blocks::components::label::Label;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let value = use_signal(|| 50.0);

rsx! {
    div { class: "space-y-2",
        Label { "Volume: {value}" }
        Slider {
            value: value,
            on_value_change: move |v| value.set(v),
        }
    }
}"#;

/// Basic slider example.
#[component]
pub fn SliderBasicExample() -> Element {
    let mut value = use_signal(|| 50.0);

    rsx! {
        div { class: "space-y-2 w-full max-w-sm",
            Label { "Volume: {value}" }
            Slider {
                value: value,
                on_value_change: move |v| value.set(v),
            }
        }
    }
}

/// Source code for the range example.
pub const RANGE_SOURCE: &str = r#"let temperature = use_signal(|| 20.0);

rsx! {
    div { class: "space-y-2",
        Label { "Temperature: {temperature}°C" }
        Slider {
            value: temperature,
            min: -10.0,
            max: 40.0,
            step: 0.5,
            on_value_change: move |v| temperature.set(v),
        }
    }
}"#;

/// Slider with custom range example.
#[component]
pub fn SliderRangeExample() -> Element {
    let mut temperature = use_signal(|| 20.0);

    rsx! {
        div { class: "space-y-2 w-full max-w-sm",
            Label { "Temperature: {temperature}°C" }
            Slider {
                value: temperature,
                min: -10.0,
                max: 40.0,
                step: 0.5,
                on_value_change: move |v| temperature.set(v),
            }
        }
    }
}

/// Source code for the disabled example.
pub const DISABLED_SOURCE: &str = r#"rsx! {
    Slider {
        default_value: 30.0,
        disabled: true,
    }
}"#;

/// Disabled slider example.
#[component]
pub fn SliderDisabledExample() -> Element {
    rsx! {
        div { class: "w-full max-w-sm",
            Slider {
                default_value: 30.0,
                disabled: true,
            }
        }
    }
}
