//! Label component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::checkbox::Checkbox;
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::{Label, LabelSize};

pub const BASIC_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-2",
        Label { "Username" }
        Input { placeholder: "Enter username" }
    }
}"#;

#[component]
pub fn LabelBasicExample() -> Element {
    rsx! {
        div { class: "grid gap-2",
            Label { "Username" }
            Input { placeholder: "Enter username" }
        }
    }
}

pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Label { size: LabelSize::Small, "Small label" }
        Label { size: LabelSize::Medium, "Medium label (default)" }
        Label { size: LabelSize::Large, "Large label" }
    }
}"#;

#[component]
pub fn LabelSizesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Label { size: LabelSize::Small, "Small label" }
            Label { size: LabelSize::Medium, "Medium label (default)" }
            Label { size: LabelSize::Large, "Large label" }
        }
    }
}

pub const REQUIRED_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-2",
        Label { required: true, "Email address" }
        Input { input_type: "email", required: true, placeholder: "you@example.com" }
    }
}"#;

#[component]
pub fn LabelRequiredExample() -> Element {
    rsx! {
        div { class: "grid gap-2",
            Label { required: true, "Email address" }
            Input { input_type: "email", required: true, placeholder: "you@example.com" }
        }
    }
}

pub const WITH_CHECKBOX_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-2",
        Checkbox { id: "terms" }
        Label { for_id: "terms", "Accept terms and conditions" }
    }
}"#;

#[component]
pub fn LabelWithCheckboxExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox { id: "terms" }
            Label { for_id: "terms", "Accept terms and conditions" }
        }
    }
}
