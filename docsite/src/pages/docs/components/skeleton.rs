//! Skeleton component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::skeleton::*;
use dioxus::prelude::*;

/// Skeleton documentation page.
#[component]
pub fn SkeletonDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Skeleton",
                description: "A placeholder component for loading states.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::skeleton::Skeleton;"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Skeleton { class: "h-4 w-[250px]" }
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
                        "A simple skeleton line."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("skeleton_basic.rs".to_string()),
                        SkeletonBasicExample {}
                    }
                }

                // Card
                div { class: "space-y-4",
                    h3 { id: "card", class: "text-xl font-medium", "Card" }
                    p { class: "text-muted-foreground",
                        "Skeleton layout for a card with avatar and text."
                    }
                    ComponentPreview {
                        source: CARD_SOURCE.to_string(),
                        filename: Some("skeleton_card.rs".to_string()),
                        SkeletonCardExample {}
                    }
                }

                // Text Block
                div { class: "space-y-4",
                    h3 { id: "text-block", class: "text-xl font-medium", "Text Block" }
                    p { class: "text-muted-foreground",
                        "Skeleton layout for a paragraph of text."
                    }
                    ComponentPreview {
                        source: TEXT_BLOCK_SOURCE.to_string(),
                        filename: Some("skeleton_text.rs".to_string()),
                        SkeletonTextBlockExample {}
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
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes for sizing" }
                            }
                        }
                    }
                }
            }
        }
    }
}
