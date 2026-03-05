//! Input component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::input::{Input, InputSize, InputVariant};
use lumen_blocks::components::label::Label;

pub const BASIC_SOURCE: &str = r#"rsx! {
    Input { placeholder: "Enter your email" }
}"#;

#[component]
pub fn InputBasicExample() -> Element {
    rsx! {
        Input { placeholder: "Enter your email" }
    }
}

pub const WITH_LABEL_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-2",
        Label { "Email" }
        Input { input_type: "email", placeholder: "john@example.com" }
    }
}"#;

#[component]
pub fn InputWithLabelExample() -> Element {
    rsx! {
        div { class: "grid gap-2",
            Label { "Email" }
            Input { input_type: "email", placeholder: "john@example.com" }
        }
    }
}

pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Input { size: InputSize::Small, placeholder: "Small input" }
        Input { size: InputSize::Medium, placeholder: "Medium input (default)" }
        Input { size: InputSize::Large, placeholder: "Large input" }
    }
}"#;

#[component]
pub fn InputSizesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Input { size: InputSize::Small, placeholder: "Small input" }
            Input { size: InputSize::Medium, placeholder: "Medium input (default)" }
            Input { size: InputSize::Large, placeholder: "Large input" }
        }
    }
}

pub const STATES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Input { placeholder: "Normal input" }
        Input { disabled: true, placeholder: "Disabled input" }
        Input { variant: InputVariant::Error, placeholder: "Error state" }
        Input { readonly: true, value: "Read-only value" }
    }
}"#;

#[component]
pub fn InputStatesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Input { placeholder: "Normal input" }
            Input { disabled: true, placeholder: "Disabled input" }
            Input { variant: InputVariant::Error, placeholder: "Error state" }
            Input { readonly: true, value: "Read-only value" }
        }
    }
}

pub const WITH_ICONS_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Input {
            icon_left: rsx! { lucide_dioxus::Search { class: "size-4" } },
            placeholder: "Search..."
        }
        Input {
            icon_right: rsx! { lucide_dioxus::Eye { class: "size-4" } },
            input_type: "password",
            placeholder: "Password"
        }
    }
}"#;

#[component]
pub fn InputWithIconsExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Input {
                icon_left: rsx! { lucide_dioxus::Search { class: "size-4" } },
                placeholder: "Search..."
            }
            Input {
                icon_right: rsx! { lucide_dioxus::Eye { class: "size-4" } },
                input_type: "password",
                placeholder: "Password"
            }
        }
    }
}

pub const TYPES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Input { input_type: "text", placeholder: "Text input" }
        Input { input_type: "email", placeholder: "Email input" }
        Input { input_type: "password", placeholder: "Password input" }
        Input { input_type: "number", placeholder: "Number input" }
        Input { input_type: "date" }
    }
}"#;

#[component]
pub fn InputTypesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Input { input_type: "text", placeholder: "Text input" }
            Input { input_type: "email", placeholder: "Email input" }
            Input { input_type: "password", placeholder: "Password input" }
            Input { input_type: "number", placeholder: "Number input" }
            Input { input_type: "date" }
        }
    }
}
