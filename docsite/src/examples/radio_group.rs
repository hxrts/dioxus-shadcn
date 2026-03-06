//! RadioGroup example components.

use dioxus::prelude::*;
use lumen_blocks::components::label::Label;
use lumen_blocks::components::radio_group::{RadioGroup, RadioGroupItem, RadioGroupOrientation};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let selected = use_signal(|| "comfortable".to_string());

rsx! {
    RadioGroup {
        value: selected,
        on_value_change: move |v| selected.set(v),

        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "default", id: "r1" }
            Label { for_id: "r1", "Default" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "comfortable", id: "r2" }
            Label { for_id: "r2", "Comfortable" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "compact", id: "r3" }
            Label { for_id: "r3", "Compact" }
        }
    }
}"#;

/// Basic radio group example.
#[component]
pub fn RadioGroupBasicExample() -> Element {
    let mut selected = use_signal(|| "comfortable".to_string());

    rsx! {
        RadioGroup {
            value: selected,
            on_value_change: move |v| selected.set(v),

            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "default", id: "r1" }
                Label { for_id: "r1", "Default" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "comfortable", id: "r2" }
                Label { for_id: "r2", "Comfortable" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "compact", id: "r3" }
                Label { for_id: "r3", "Compact" }
            }
        }
    }
}

/// Source code for the horizontal example.
pub const HORIZONTAL_SOURCE: &str = r#"let plan = use_signal(|| "startup".to_string());

rsx! {
    RadioGroup {
        value: plan,
        orientation: RadioGroupOrientation::Horizontal,
        on_value_change: move |v| plan.set(v),

        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "startup", id: "h1" }
            Label { for_id: "h1", "Startup" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "business", id: "h2" }
            Label { for_id: "h2", "Business" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "enterprise", id: "h3" }
            Label { for_id: "h3", "Enterprise" }
        }
    }
}"#;

/// Horizontal radio group example.
#[component]
pub fn RadioGroupHorizontalExample() -> Element {
    let mut plan = use_signal(|| "startup".to_string());

    rsx! {
        RadioGroup {
            value: plan,
            orientation: RadioGroupOrientation::Horizontal,
            on_value_change: move |v| plan.set(v),

            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "startup", id: "h1" }
                Label { for_id: "h1", "Startup" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "business", id: "h2" }
                Label { for_id: "h2", "Business" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "enterprise", id: "h3" }
                Label { for_id: "h3", "Enterprise" }
            }
        }
    }
}

/// Source code for the disabled example.
pub const DISABLED_SOURCE: &str = r#"rsx! {
    RadioGroup {
        default_value: Some("option1".to_string()),
        disabled: true,

        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "option1", id: "d1" }
            Label { for_id: "d1", "Option 1" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "option2", id: "d2" }
            Label { for_id: "d2", "Option 2" }
        }
    }
}"#;

/// Disabled radio group example.
#[component]
pub fn RadioGroupDisabledExample() -> Element {
    rsx! {
        RadioGroup {
            default_value: Some("option1".to_string()),
            disabled: true,

            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "option1", id: "d1" }
                Label { for_id: "d1", "Option 1" }
            }
            div { class: "flex items-center space-x-2",
                RadioGroupItem { value: "option2", id: "d2" }
                Label { for_id: "d2", "Option 2" }
            }
        }
    }
}
