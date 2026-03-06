//! Textarea component examples and source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::label::Label;
use dioxus_shadcn::components::textarea::Textarea;

pub const BASIC_SOURCE: &str = r#"rsx! {
    Textarea { placeholder: "Type your message here..." }
}"#;

#[component]
pub fn TextareaBasicExample() -> Element {
    rsx! {
        Textarea { placeholder: "Type your message here..." }
    }
}

pub const WITH_LABEL_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-2",
        Label { "Your message" }
        Textarea { placeholder: "Tell us what you think..." }
    }
}"#;

#[component]
pub fn TextareaWithLabelExample() -> Element {
    rsx! {
        div { class: "grid gap-2",
            Label { "Your message" }
            Textarea { placeholder: "Tell us what you think..." }
        }
    }
}

pub const ROWS_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Textarea { rows: 2, placeholder: "2 rows" }
        Textarea { rows: 4, placeholder: "4 rows" }
        Textarea { rows: 6, placeholder: "6 rows" }
    }
}"#;

#[component]
pub fn TextareaRowsExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Textarea { rows: 2, placeholder: "2 rows" }
            Textarea { rows: 4, placeholder: "4 rows" }
            Textarea { rows: 6, placeholder: "6 rows" }
        }
    }
}

pub const STATES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Textarea { placeholder: "Normal textarea" }
        Textarea { disabled: true, placeholder: "Disabled textarea" }
        Textarea { error: true, placeholder: "Error state" }
        Textarea { readonly: true, default_value: "Read-only content" }
    }
}"#;

#[component]
pub fn TextareaStatesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Textarea { placeholder: "Normal textarea" }
            Textarea { disabled: true, placeholder: "Disabled textarea" }
            Textarea { error: true, placeholder: "Error state" }
            Textarea { readonly: true, default_value: "Read-only content" }
        }
    }
}
