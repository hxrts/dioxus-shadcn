//! Separator component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::separator::*;
use dioxus::prelude::*;

/// Separator documentation page.
#[component]
pub fn SeparatorDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Separator",
                description: "A visual divider between content sections.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::separator::{Separator, SeparatorOrientation};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Separator {}
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Examples
            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                // Horizontal
                div { class: "space-y-4",
                    h3 { id: "horizontal", class: "text-xl font-medium", "Horizontal" }
                    p { class: "text-muted-foreground",
                        "The default orientation divides content vertically."
                    }
                    ComponentPreview {
                        source: HORIZONTAL_SOURCE.to_string(),
                        filename: Some("separator_horizontal.rs".to_string()),
                        SeparatorHorizontalExample {}
                    }
                }

                // Vertical
                div { class: "space-y-4",
                    h3 { id: "vertical", class: "text-xl font-medium", "Vertical" }
                    p { class: "text-muted-foreground",
                        "Use vertical orientation to separate inline content."
                    }
                    ComponentPreview {
                        source: VERTICAL_SOURCE.to_string(),
                        filename: Some("separator_vertical.rs".to_string()),
                        SeparatorVerticalExample {}
                    }
                }

                // In Card
                div { class: "space-y-4",
                    h3 { id: "in-card", class: "text-xl font-medium", "In Card" }
                    p { class: "text-muted-foreground",
                        "Separators work well within cards to divide sections."
                    }
                    ComponentPreview {
                        source: IN_CARD_SOURCE.to_string(),
                        filename: Some("separator_card.rs".to_string()),
                        SeparatorInCardExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "SeparatorOrientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "Horizontal" }
                                td { class: "py-3 px-4 text-muted-foreground", "Direction of the separator" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "decorative" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Purely decorative (no ARIA)" }
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
        }
    }
}
