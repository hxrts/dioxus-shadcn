//! Resizable component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::resizable::*;
use dioxus::prelude::*;

/// Resizable documentation page.
#[component]
pub fn ResizableDoc() -> Element {
    let usage_source = r##"rsx! {
    ResizablePanelGroup { direction: ResizableDirection::Horizontal,
        ResizablePanel { default_size: 50.0,
            "Panel One"
        }
        ResizableHandle { with_handle: true }
        ResizablePanel { default_size: 50.0,
            "Panel Two"
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Resizable",
                description: "A set of components for building resizable split panel layouts.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::resizable::{{ResizablePanelGroup, ResizablePanel, ResizableHandle, ResizableDirection}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        ResizableBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "ResizablePanel" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "default_size" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "50.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Default size as percentage (0-100)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "min_size" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<f64>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Minimum size percentage" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "collapsible" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the panel is collapsible" }
                            }
                        }
                    }
                }
            }
        }
    }
}
