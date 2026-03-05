//! InputGroup component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::input_group::*;
use dioxus::prelude::*;

/// InputGroup documentation page.
#[component]
pub fn InputGroupDoc() -> Element {
    let usage_source = r##"rsx! {
    InputGroup {
        InputGroupText { "$" }
        InputGroupInput { placeholder: "Amount" }
        InputGroupText { ".00" }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Input Group",
                description: "A composable component system for creating inputs with prefixes, suffixes, and addons.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::input_group::{{InputGroup, InputGroupInput, InputGroupText, InputGroupAddon, InputGroupButton}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        InputGroupBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "InputGroup" }
                div { class: "overflow-x-auto",
                    table { class: "w-full text-sm",
                        thead {
                            tr { class: "border-b border-border",
                                th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                th { class: "py-3 px-4 text-left font-medium", "Type" }
                                th { class: "py-3 px-4 text-left font-medium", "Default" }
                                th { class: "py-3 px-4 text-left font-medium", "Description" }
                            }
                        }
                        tbody {
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "invalid" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the group has an error" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the group is disabled" }
                            }
                        }
                    }
                }
            }
        }
    }
}
