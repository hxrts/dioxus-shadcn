//! Form example components.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::form::{
    Form, FormControl, FormDescription, FormField, FormLabel, FormMessage,
};
use lumen_blocks::components::input::Input;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Form {
        on_submit: move |_| {
            // Handle form submission
        },
        class: "space-y-6",

        FormField { name: "username",
            FormLabel { "Username" }
            FormControl {
                Input { placeholder: "Enter your username" }
            }
            FormDescription { "This is your public display name." }
            FormMessage {}
        }

        FormField { name: "email",
            FormLabel { "Email" }
            FormControl {
                Input { r#type: "email", placeholder: "Enter your email" }
            }
            FormDescription { "We'll never share your email." }
            FormMessage {}
        }

        Button { button_type: "submit", "Submit" }
    }
}"##;

/// Basic form example.
#[component]
pub fn FormBasicExample() -> Element {
    rsx! {
        Form {
            on_submit: move |_| {
                // Handle form submission
            },
            class: "space-y-6 w-full max-w-sm",

            FormField { name: "username",
                FormLabel { "Username" }
                FormControl {
                    Input { placeholder: "Enter your username" }
                }
                FormDescription { "This is your public display name." }
                FormMessage {}
            }

            FormField { name: "email",
                FormLabel { "Email" }
                FormControl {
                    Input { r#type: "email", placeholder: "Enter your email" }
                }
                FormDescription { "We'll never share your email." }
                FormMessage {}
            }

            Button { button_type: "submit", "Submit" }
        }
    }
}
