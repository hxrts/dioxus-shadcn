//! Command component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::command::*;
use dioxus::prelude::*;

/// Command documentation page.
#[component]
pub fn CommandDoc() -> Element {
    let usage_source = r##"rsx! {
    Command {
        on_select: move |value| {
            // Handle selection
        },
        CommandInput { placeholder: "Type a command or search..." }
        CommandList {
            CommandEmpty { "No results found." }
            CommandGroup { heading: "Suggestions",
                CommandItem { value: "calendar", "Calendar" }
                CommandItem { value: "search", "Search" }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Command",
                description: "Fast, composable, unstyled command menu for Dioxus.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::command::{{Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Examples
            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                // Basic
                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    p { class: "text-muted-foreground",
                        "A command palette with search, groups, and shortcuts."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("command_basic.rs".to_string()),
                        CommandBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // Command
                h3 { class: "text-lg font-medium mt-6", "Command" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "on_select" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when an item is selected" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "show_input" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether to show the search input" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "placeholder" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Placeholder text for search" }
                            }
                        }
                    }
                }

                // CommandItem
                h3 { class: "text-lg font-medium mt-6", "CommandItem" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "The value of this item" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "keywords" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Vec<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional keywords for filtering" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the item is disabled" }
                            }
                        }
                    }
                }
            }
        }
    }
}
