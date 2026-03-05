//! Direction component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::direction::*;
use dioxus::prelude::*;

/// Direction documentation page.
#[component]
pub fn DirectionDoc() -> Element {
    let usage_source = r##"rsx! {
    DirectionProvider { direction: Direction::Rtl,
        // All children will be RTL
        div { "Right-to-left content" }
    }
}

// In a child component
fn MyComponent() -> Element {
    let direction = use_direction();
    // Use direction for conditional styling
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Direction",
                description: "A context provider for managing RTL/LTR text direction.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::direction::{{Direction, DirectionProvider, use_direction}};".to_string(),
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
                        DirectionBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "DirectionProvider" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "direction" }
                                td { class: "py-3 px-4 font-mono text-xs", "Direction" }
                                td { class: "py-3 px-4 font-mono text-xs", "Ltr" }
                                td { class: "py-3 px-4 text-muted-foreground", "Text direction (Ltr, Rtl)" }
                            }
                        }
                    }
                }

                h3 { class: "text-lg font-medium mt-6", "Hooks" }
                p { class: "text-muted-foreground",
                    code { class: "px-1.5 py-0.5 rounded bg-muted font-mono text-xs", "use_direction()" }
                    " - Returns the current direction from context."
                }
            }
        }
    }
}
