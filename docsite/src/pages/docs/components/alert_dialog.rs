//! AlertDialog component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::alert_dialog::*;
use dioxus::prelude::*;

/// AlertDialog documentation page.
#[component]
pub fn AlertDialogDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Alert Dialog",
                description: "A modal dialog that interrupts the user with important content and expects a response.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
    AlertDialogTrigger,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    AlertDialog {
        AlertDialogTrigger {
            Button { "Open" }
        }
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Are you sure?" }
                AlertDialogDescription { "This action cannot be undone." }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { "Continue" }
            }
        }
    }
}"#.to_string(),
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
                        "A basic alert dialog with a confirmation message."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("alert_dialog_basic.rs".to_string()),
                        AlertDialogBasicExample {}
                    }
                }

                // Destructive
                div { class: "space-y-4",
                    h3 { id: "destructive", class: "text-xl font-medium", "Destructive" }
                    p { class: "text-muted-foreground",
                        "An alert dialog for destructive actions like deleting data."
                    }
                    ComponentPreview {
                        source: DESTRUCTIVE_SOURCE.to_string(),
                        filename: Some("alert_dialog_destructive.rs".to_string()),
                        AlertDialogDestructiveExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

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
                                td { class: "py-3 px-4 font-mono text-xs", "open" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<bool>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled open state" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_open" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial open state (uncontrolled)" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_open_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when open state changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
