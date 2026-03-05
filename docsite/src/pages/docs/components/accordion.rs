//! Accordion component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::accordion::*;
use dioxus::prelude::*;

/// Accordion documentation page.
#[component]
pub fn AccordionDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Accordion",
                description: "A vertically stacked set of interactive headings that each reveal a section of content.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Accordion {
        AccordionItem { index: 0,
            AccordionTrigger { "Section Title" }
            AccordionContent { "Section content here." }
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
                        "A basic accordion with single item expansion."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("accordion_basic.rs".to_string()),
                        AccordionBasicExample {}
                    }
                }

                // Multiple Open
                div { class: "space-y-4",
                    h3 { id: "multiple", class: "text-xl font-medium", "Multiple Open" }
                    p { class: "text-muted-foreground",
                        "Allow multiple items to be open simultaneously using the allow_multiple_open prop."
                    }
                    ComponentPreview {
                        source: MULTIPLE_SOURCE.to_string(),
                        filename: Some("accordion_multiple.rs".to_string()),
                        AccordionMultipleExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                div { class: "space-y-6",
                    // Accordion props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "Accordion" }
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
                                        td { class: "py-3 px-4 font-mono text-xs", "allow_multiple_open" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 font-mono text-xs", "false" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Allow multiple items open" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "horizontal" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 font-mono text-xs", "false" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Horizontal orientation" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "class" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                        td { class: "py-3 px-4 font-mono text-xs", "None" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                                    }
                                }
                            }
                        }
                    }

                    // AccordionItem props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "AccordionItem" }
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
                                        td { class: "py-3 px-4 font-mono text-xs", "index" }
                                        td { class: "py-3 px-4 font-mono text-xs", "usize" }
                                        td { class: "py-3 px-4 text-muted-foreground", "The index of the item (required)" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "on_change" }
                                        td { class: "py-3 px-4 font-mono text-xs", "EventHandler<bool>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Called when open state changes" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "id" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Optional element ID" }
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
