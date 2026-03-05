//! Dialog component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::dialog::*;
use dioxus::prelude::*;
use lumen_blocks::components::toast::ToastProvider;

/// Dialog documentation page.
#[component]
pub fn DialogDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Dialog",
                description: "A modal dialog that interrupts the user with important content and expects a response.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::dialog::{
    Dialog, DialogContent, DialogDescription, DialogFooter,
    DialogHeader, DialogOverlay, DialogTitle, DialogClose,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let mut open = use_signal(|| false);

rsx! {
    Button { on_click: move |_| open.set(true), "Open Dialog" }

    Dialog {
        open: ReadSignal::new(Signal::new(Some(*open.read()))),
        on_open_change: move |new_open| open.set(new_open),

        DialogOverlay {}
        DialogContent {
            DialogHeader {
                DialogTitle { "Dialog Title" }
                DialogDescription { "Dialog description here." }
            }
            // Content...
            DialogFooter {
                Button { "Confirm" }
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
                        "A basic dialog with title, description, and action buttons."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: BASIC_SOURCE.to_string(),
                            filename: Some("dialog_basic.rs".to_string()),
                            DialogBasicExample {}
                        }
                    }
                }

                // With Form
                div { class: "space-y-4",
                    h3 { id: "with-form", class: "text-xl font-medium", "With Form" }
                    p { class: "text-muted-foreground",
                        "A dialog containing a form for user input."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: WITH_FORM_SOURCE.to_string(),
                            filename: Some("dialog_form.rs".to_string()),
                            DialogWithFormExample {}
                        }
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                div { class: "space-y-6",
                    // Dialog props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "Dialog" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                        th { class: "py-3 px-4 text-left font-medium", "Type" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "open" }
                                        td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<Option<bool>>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Controlled open state" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "default_open" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Default open state" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "on_open_change" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Callback<bool>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Called when open state changes" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "is_modal" }
                                        td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<bool>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Whether dialog is modal (traps focus)" }
                                    }
                                }
                            }
                        }
                    }

                    // DialogContent props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "DialogContent" }
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
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "show_close_button" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 font-mono text-xs", "true" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Show close button in corner" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
