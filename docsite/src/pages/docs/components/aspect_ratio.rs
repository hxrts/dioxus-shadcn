//! AspectRatio component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::aspect_ratio::*;
use dioxus::prelude::*;

/// AspectRatio documentation page.
#[component]
pub fn AspectRatioDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Aspect Ratio",
                description: "Displays content within a desired ratio.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::aspect_ratio::AspectRatio;"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    AspectRatio { ratio: 16.0 / 9.0,
        // Content here
    }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Examples
            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                // Basic (16:9)
                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "16:9 Ratio" }
                    p { class: "text-muted-foreground",
                        "The most common aspect ratio for videos and wide images."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("aspect_ratio_basic.rs".to_string()),
                        AspectRatioBasicExample {}
                    }
                }

                // Square (1:1)
                div { class: "space-y-4",
                    h3 { id: "square", class: "text-xl font-medium", "Square" }
                    p { class: "text-muted-foreground",
                        "A 1:1 ratio for square content."
                    }
                    ComponentPreview {
                        source: SQUARE_SOURCE.to_string(),
                        filename: Some("aspect_ratio_square.rs".to_string()),
                        AspectRatioSquareExample {}
                    }
                }

                // Portrait (3:4)
                div { class: "space-y-4",
                    h3 { id: "portrait", class: "text-xl font-medium", "Portrait" }
                    p { class: "text-muted-foreground",
                        "A 3:4 ratio for portrait-oriented content."
                    }
                    ComponentPreview {
                        source: PORTRAIT_SOURCE.to_string(),
                        filename: Some("aspect_ratio_portrait.rs".to_string()),
                        AspectRatioPortraitExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "ratio" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "required" }
                                td { class: "py-3 px-4 text-muted-foreground", "The aspect ratio (width / height)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "id" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Optional element ID" }
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
