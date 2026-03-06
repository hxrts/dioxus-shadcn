//! InputGroup example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::input_group::{InputGroup, InputGroupInput, InputGroupText};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    InputGroup {
        InputGroupText { "$" }
        InputGroupInput { placeholder: "Amount" }
        InputGroupText { ".00" }
    }
}"##;

/// Basic input group example.
#[component]
pub fn InputGroupBasicExample() -> Element {
    rsx! {
        div { class: "max-w-xs",
            InputGroup {
                InputGroupText { "$" }
                InputGroupInput { placeholder: "Amount" }
                InputGroupText { ".00" }
            }
        }
    }
}
