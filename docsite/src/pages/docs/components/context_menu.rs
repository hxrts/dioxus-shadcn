//! ContextMenu component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::context_menu::*;
use dioxus::prelude::*;

/// ContextMenu documentation page.
#[component]
pub fn ContextMenuDoc() -> Element {
    let usage_source = r##"rsx! {
    ContextMenu {
        ContextMenuTrigger {
            div { "Right click here" }
        }
        ContextMenuContent {
            ContextMenuItem { value: "copy", "Copy" }
            ContextMenuItem { value: "paste", "Paste" }
            ContextMenuSeparator {}
            ContextMenuItem { value: "delete", destructive: true, "Delete" }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Context Menu",
                description: "Displays a menu at the pointer location, triggered by a right-click.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use dioxus_shadcn::components::context_menu::{{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem}};".to_string(),
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
                        "A context menu with items, labels, and shortcuts."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("context_menu_basic.rs".to_string()),
                        ContextMenuBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // ContextMenuItem
                h3 { class: "text-lg font-medium mt-6", "ContextMenuItem" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "\"\"" }
                                td { class: "py-3 px-4 text-muted-foreground", "The value of the item" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the item is disabled" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "destructive" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Renders item in destructive style" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_select" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<EventHandler<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when item is selected" }
                            }
                        }
                    }
                }
            }
        }
    }
}
