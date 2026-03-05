//! Form example components.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::form::{Form, FormControl, FormDescription, FormField, FormLabel, FormMessage};
use lumen_blocks::components::input::Input;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Form {
        on_submit: move |_| {
            // Handle form submission
        },
        FormField { name: "username",
            FormLabel { "Username" }
            FormControl {
                Input { placeholder: "Enter your username" }
            }
            FormDescription { "This is your public display name." }
            FormMessage {}
        }
        Button { r#type: "submit", "Submit" }
    }
}"##;

/// Basic form example.
#[component]
pub fn FormBasicExample() -> Element {
    rsx! {
        Form {
            class: "w-2/3 space-y-6",
            on_submit: move |_| {
                // Handle form submission
            },
            FormField { name: "username",
                FormLabel { "Username" }
                FormControl {
                    Input { placeholder: "Enter your username" }
                }
                FormDescription { "This is your public display name." }
                FormMessage {}
            }
            Button { r#type: "submit", "Submit" }
        }
    }
}
