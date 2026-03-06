//! Collapsible example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    Collapsible {
        CollapsibleTrigger { "Click to expand" }
        CollapsibleContent {
            p { "This content is hidden by default and revealed when the trigger is clicked." }
        }
    }
}"#;

/// Basic collapsible example.
#[component]
pub fn CollapsibleBasicExample() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger { "Click to expand" }
            CollapsibleContent {
                p { "This content is hidden by default and revealed when the trigger is clicked." }
            }
        }
    }
}

/// Source code for the repository example.
pub const REPOSITORY_SOURCE: &str = r#"rsx! {
    Collapsible {
        CollapsibleTrigger { "@peduarte/radix-ui" }
        CollapsibleContent {
            div { class: "space-y-2",
                div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                    "@radix-ui/primitives"
                }
                div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                    "@radix-ui/colors"
                }
                div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                    "@stitches/react"
                }
            }
        }
    }
}"#;

/// Repository collapsible example.
#[component]
pub fn CollapsibleRepositoryExample() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger { "@peduarte/radix-ui" }
            CollapsibleContent {
                div { class: "space-y-2",
                    div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                        "@radix-ui/primitives"
                    }
                    div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                        "@radix-ui/colors"
                    }
                    div { class: "rounded-md border px-4 py-2 font-mono text-sm shadow-sm",
                        "@stitches/react"
                    }
                }
            }
        }
    }
}
