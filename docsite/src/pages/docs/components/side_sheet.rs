//! SideSheet component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::side_sheet::*;
use dioxus::prelude::*;

/// SideSheet documentation page.
#[component]
pub fn SideSheetDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Side Sheet",
                description: "A panel that slides in from the edge of the screen.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::side_sheet::{
    SideSheet, SideSheetContent, SideSheetDescription, SideSheetHeader,
    SideSheetTitle, SideSheetTrigger, SideSheetFooter, SideSheetSide,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    SideSheet {
        SideSheetTrigger {
            Button { "Open" }
        }
        SideSheetContent {
            SideSheetHeader {
                SideSheetTitle { "Title" }
                SideSheetDescription { "Description" }
            }
            // Content here
            SideSheetFooter {
                Button { "Save" }
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
                        "A side sheet with a form for editing user profile."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("side_sheet_basic.rs".to_string()),
                        SideSheetBasicExample {}
                    }
                }

                // Sides
                div { class: "space-y-4",
                    h3 { id: "sides", class: "text-xl font-medium", "Sides" }
                    p { class: "text-muted-foreground",
                        "The sheet can slide in from any edge of the screen."
                    }
                    ComponentPreview {
                        source: SIDES_SOURCE.to_string(),
                        filename: Some("side_sheet_sides.rs".to_string()),
                        SideSheetSidesExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "side" }
                                td { class: "py-3 px-4 font-mono text-xs", "SideSheetSide" }
                                td { class: "py-3 px-4 font-mono text-xs", "Right" }
                                td { class: "py-3 px-4 text-muted-foreground", "Edge to slide from" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_open" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial open state" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "\"\"" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                        }
                    }
                }

                // Sub-components
                h3 { class: "text-xl font-medium mt-6", "Sub-components" }
                ul { class: "list-disc list-inside space-y-1 text-muted-foreground",
                    li { code { class: "text-sm", "SideSheetTrigger" } " - Button to open the sheet" }
                    li { code { class: "text-sm", "SideSheetContent" } " - Container for sheet content" }
                    li { code { class: "text-sm", "SideSheetHeader" } " - Header section" }
                    li { code { class: "text-sm", "SideSheetTitle" } " - Title text" }
                    li { code { class: "text-sm", "SideSheetDescription" } " - Description text" }
                    li { code { class: "text-sm", "SideSheetBody" } " - Main content area" }
                    li { code { class: "text-sm", "SideSheetFooter" } " - Footer with actions" }
                    li { code { class: "text-sm", "SideSheetClose" } " - Custom close trigger" }
                    li { code { class: "text-sm", "SideSheetCloseButton" } " - Default close button" }
                }
            }
        }
    }
}
