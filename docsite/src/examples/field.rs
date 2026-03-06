//! Field example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet,
};
use dioxus_shadcn::components::input::Input;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    FieldSet {
        FieldLegend { "Personal Information" }
        FieldGroup {
            Field {
                FieldLabel { "Name" }
                Input { placeholder: "Enter your name" }
            }
            Field {
                FieldLabel { "Email" }
                Input { placeholder: "Enter your email" }
                FieldDescription { "We'll never share your email." }
            }
        }
    }
}"##;

/// Basic field example.
#[component]
pub fn FieldBasicExample() -> Element {
    rsx! {
        FieldSet { class: "max-w-md",
            FieldLegend { "Personal Information" }
            FieldGroup {
                Field {
                    FieldLabel { "Name" }
                    Input { placeholder: "Enter your name" }
                }
                Field {
                    FieldLabel { "Email" }
                    Input { placeholder: "Enter your email" }
                    FieldDescription { "We'll never share your email." }
                }
            }
        }
    }
}
