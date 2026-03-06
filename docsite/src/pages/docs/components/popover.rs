//! Popover component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::popover::*;
use dioxus::prelude::*;

/// Popover documentation page.
#[component]
pub fn PopoverDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Popover",
                description: "Displays rich content in a floating panel positioned relative to a trigger.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::popover::{
    Popover, PopoverContent, PopoverDescription, PopoverHeader,
    PopoverTitle, PopoverTrigger,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Popover {
        PopoverTrigger {
            Button { "Open" }
        }
        PopoverContent {
            p { "Content goes here" }
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
                        "A popover with a form for editing settings."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("popover_basic.rs".to_string()),
                        PopoverBasicExample {}
                    }
                }

                // Positioning
                div { class: "space-y-4",
                    h3 { id: "positioning", class: "text-xl font-medium", "Positioning" }
                    p { class: "text-muted-foreground",
                        "Control which side the popover appears relative to the trigger."
                    }
                    ComponentPreview {
                        source: POSITIONING_SOURCE.to_string(),
                        filename: Some("popover_positioning.rs".to_string()),
                        PopoverPositioningExample {}
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
                                td { class: "py-3 px-4 text-muted-foreground", "Initial open state" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "side" }
                                td { class: "py-3 px-4 font-mono text-xs", "PopoverSide" }
                                td { class: "py-3 px-4 font-mono text-xs", "Bottom" }
                                td { class: "py-3 px-4 text-muted-foreground", "Side to position on" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "align" }
                                td { class: "py-3 px-4 font-mono text-xs", "PopoverAlign" }
                                td { class: "py-3 px-4 font-mono text-xs", "Center" }
                                td { class: "py-3 px-4 text-muted-foreground", "Alignment along side" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "side_offset" }
                                td { class: "py-3 px-4 font-mono text-xs", "i32" }
                                td { class: "py-3 px-4 font-mono text-xs", "4" }
                                td { class: "py-3 px-4 text-muted-foreground", "Offset from trigger in pixels" }
                            }
                        }
                    }
                }
            }
        }
    }
}
