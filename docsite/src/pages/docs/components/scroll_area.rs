//! ScrollArea component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::scroll_area::*;
use dioxus::prelude::*;

/// ScrollArea documentation page.
#[component]
pub fn ScrollAreaDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Scroll Area",
                description: "A scrollable area with styled scrollbars.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::scroll_area::{
    ScrollArea, ScrollAreaViewport, ScrollBar, ScrollbarOrientation,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    ScrollArea {
        height: "200px",

        ScrollAreaViewport {
            // Content here
        }
        ScrollBar { orientation: ScrollbarOrientation::Vertical }
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
                    h3 { id: "basic", class: "text-xl font-medium", "Vertical" }
                    p { class: "text-muted-foreground",
                        "A vertically scrollable area with a list of tags."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("scroll_area_basic.rs".to_string()),
                        ScrollAreaBasicExample {}
                    }
                }

                // Horizontal
                div { class: "space-y-4",
                    h3 { id: "horizontal", class: "text-xl font-medium", "Horizontal" }
                    p { class: "text-muted-foreground",
                        "A horizontally scrollable area for image galleries."
                    }
                    ComponentPreview {
                        source: HORIZONTAL_SOURCE.to_string(),
                        filename: Some("scroll_area_horizontal.rs".to_string()),
                        ScrollAreaHorizontalExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // ScrollArea
                h3 { class: "text-lg font-medium mt-6", "ScrollArea" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "height" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "CSS height value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "max_height" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "CSS max-height value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "width" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "CSS width value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "scrollbar" }
                                td { class: "py-3 px-4 font-mono text-xs", "ScrollbarVisibility" }
                                td { class: "py-3 px-4 font-mono text-xs", "Auto" }
                                td { class: "py-3 px-4 text-muted-foreground", "When to show scrollbar" }
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

                // ScrollBar
                h3 { class: "text-lg font-medium mt-6", "ScrollBar" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "ScrollbarOrientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "Vertical" }
                                td { class: "py-3 px-4 text-muted-foreground", "Scrollbar direction" }
                            }
                        }
                    }
                }
            }
        }
    }
}
