//! Textarea component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::textarea::*;
use dioxus::prelude::*;

/// Textarea documentation page.
#[component]
pub fn TextareaDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Textarea",
                description: "A multi-line text input field.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::textarea::Textarea;"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Textarea { placeholder: "Type your message..." }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("textarea_basic.rs".to_string()),
                        TextareaBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "with-label", class: "text-xl font-medium", "With Label" }
                    ComponentPreview {
                        source: WITH_LABEL_SOURCE.to_string(),
                        filename: Some("textarea_label.rs".to_string()),
                        TextareaWithLabelExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "rows", class: "text-xl font-medium", "Custom Rows" }
                    ComponentPreview {
                        source: ROWS_SOURCE.to_string(),
                        filename: Some("textarea_rows.rs".to_string()),
                        TextareaRowsExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "states", class: "text-xl font-medium", "States" }
                    ComponentPreview {
                        source: STATES_SOURCE.to_string(),
                        filename: Some("textarea_states.rs".to_string()),
                        TextareaStatesExample {}
                    }
                }
            }

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
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Signal<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "placeholder" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Placeholder text" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "rows" }
                                td { class: "py-3 px-4 font-mono text-xs", "u32" }
                                td { class: "py-3 px-4 font-mono text-xs", "3" }
                                td { class: "py-3 px-4 text-muted-foreground", "Number of visible rows" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the textarea" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "error" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Error state styling" }
                            }
                        }
                    }
                }
            }
        }
    }
}
