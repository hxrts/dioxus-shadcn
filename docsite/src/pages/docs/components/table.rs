//! Table component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::table::*;
use dioxus::prelude::*;

/// Table documentation page.
#[component]
pub fn TableDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Table",
                description: "A responsive table component for displaying tabular data.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter,
    TableHead, TableHeader, TableRow,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Table {
        TableHeader {
            TableRow {
                TableHead { "Name" }
                TableHead { "Email" }
            }
        }
        TableBody {
            TableRow {
                TableCell { "John Doe" }
                TableCell { "john@example.com" }
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
                        "A table with header, body, footer, and caption."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("table_basic.rs".to_string()),
                        TableBasicExample {}
                    }
                }

                // Selected Rows
                div { class: "space-y-4",
                    h3 { id: "selected", class: "text-xl font-medium", "Selected Rows" }
                    p { class: "text-muted-foreground",
                        "Use the selected prop on TableRow to highlight selected rows."
                    }
                    ComponentPreview {
                        source: SELECTED_SOURCE.to_string(),
                        filename: Some("table_selected.rs".to_string()),
                        TableSelectedExample {}
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
                                th { class: "py-3 px-4 text-left font-medium", "Component" }
                                th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                th { class: "py-3 px-4 text-left font-medium", "Type" }
                                th { class: "py-3 px-4 text-left font-medium", "Description" }
                            }
                        }
                        tbody {
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "Table" }
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "TableRow" }
                                td { class: "py-3 px-4 font-mono text-xs", "selected" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 text-muted-foreground", "Highlight as selected" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "TableHead" }
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "TableCell" }
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
