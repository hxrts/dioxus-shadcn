//! Checkbox component examples with embedded source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::checkbox::{Checkbox, CheckboxSize};
use dioxus_shadcn::components::label::Label;

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use dioxus_shadcn::components::checkbox::Checkbox;
use dioxus_shadcn::components::label::Label;

rsx! {
    div { class: "flex items-center space-x-2",
        Checkbox { id: Some("terms".to_string()) }
        Label { for_id: "terms", "Accept terms and conditions" }
    }
}"#;

pub const SIZES_SOURCE: &str = r#"use dioxus_shadcn::components::checkbox::{Checkbox, CheckboxSize};

rsx! {
    div { class: "flex items-center gap-4",
        Checkbox { size: CheckboxSize::Small }
        Checkbox { size: CheckboxSize::Medium }
        Checkbox { size: CheckboxSize::Large }
    }
}"#;

pub const CONTROLLED_SOURCE: &str = r#"use dioxus_shadcn::components::checkbox::Checkbox;

let checked = use_signal(|| false);

rsx! {
    Checkbox {
        checked: checked,
        on_checked_change: move |new_value| checked.set(new_value),
    }
}"#;

pub const DISABLED_SOURCE: &str = r#"use dioxus_shadcn::components::checkbox::Checkbox;

rsx! {
    div { class: "flex items-center gap-4",
        Checkbox { disabled: true }
        Checkbox { disabled: true, default_checked: true }
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn CheckboxBasicExample() -> Element {
    rsx! {
        div { class: "flex items-center space-x-2",
            Checkbox { id: Some("terms".to_string()) }
            Label { for_id: "terms", "Accept terms and conditions" }
        }
    }
}

#[component]
pub fn CheckboxSizesExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "flex items-center space-x-2",
                Checkbox { id: Some("small".to_string()), size: CheckboxSize::Small }
                Label { for_id: "small", "Small" }
            }
            div { class: "flex items-center space-x-2",
                Checkbox { id: Some("medium".to_string()), size: CheckboxSize::Medium }
                Label { for_id: "medium", "Medium" }
            }
            div { class: "flex items-center space-x-2",
                Checkbox { id: Some("large".to_string()), size: CheckboxSize::Large }
                Label { for_id: "large", "Large" }
            }
        }
    }
}

#[component]
pub fn CheckboxControlledExample() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "flex items-center space-x-2",
            Checkbox {
                id: Some("controlled".to_string()),
                checked: checked,
                on_checked_change: move |new_value| checked.set(new_value),
            }
            Label { for_id: "controlled",
                if *checked.read() { "Checked" } else { "Unchecked" }
            }
        }
    }
}

#[component]
pub fn CheckboxDisabledExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-6",
            div { class: "flex items-center space-x-2",
                Checkbox { id: Some("disabled1".to_string()), disabled: true }
                Label { for_id: "disabled1", class: "opacity-50", "Disabled" }
            }
            div { class: "flex items-center space-x-2",
                Checkbox { id: Some("disabled2".to_string()), disabled: true, default_checked: true }
                Label { for_id: "disabled2", class: "opacity-50", "Disabled checked" }
            }
        }
    }
}
