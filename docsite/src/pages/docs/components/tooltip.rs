//! Tooltip component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::tooltip::*;
use dioxus::prelude::*;

/// Tooltip documentation page.
#[component]
pub fn TooltipDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Tooltip",
                description: "A popup that displays information when hovering over an element.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::tooltip::{Tooltip, TooltipProvider, TooltipSide};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Tooltip { content: "Helpful information",
        Button { "Hover me" }
    }
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
                        filename: Some("tooltip_basic.rs".to_string()),
                        TooltipBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "sides", class: "text-xl font-medium", "Sides" }
                    p { class: "text-muted-foreground", "Position the tooltip on different sides." }
                    ComponentPreview {
                        source: SIDES_SOURCE.to_string(),
                        filename: Some("tooltip_sides.rs".to_string()),
                        TooltipSidesExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "with-provider", class: "text-xl font-medium", "With Provider" }
                    p { class: "text-muted-foreground", "Use TooltipProvider to configure delay globally." }
                    ComponentPreview {
                        source: WITH_PROVIDER_SOURCE.to_string(),
                        filename: Some("tooltip_provider.rs".to_string()),
                        TooltipWithProviderExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "no-arrow", class: "text-xl font-medium", "Without Arrow" }
                    ComponentPreview {
                        source: NO_ARROW_SOURCE.to_string(),
                        filename: Some("tooltip_no_arrow.rs".to_string()),
                        TooltipNoArrowExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "content" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Text content" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "content_element" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Element>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Rich content" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "side" }
                                td { class: "py-3 px-4 font-mono text-xs", "TooltipSide" }
                                td { class: "py-3 px-4 font-mono text-xs", "Top" }
                                td { class: "py-3 px-4 text-muted-foreground", "Position side" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "delay_ms" }
                                td { class: "py-3 px-4 font-mono text-xs", "u32" }
                                td { class: "py-3 px-4 font-mono text-xs", "200" }
                                td { class: "py-3 px-4 text-muted-foreground", "Show delay in ms" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "show_arrow" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Show arrow element" }
                            }
                        }
                    }
                }
            }
        }
    }
}
