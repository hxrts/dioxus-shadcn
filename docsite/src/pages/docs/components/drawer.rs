//! Drawer component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::drawer::*;
use dioxus::prelude::*;

/// Drawer documentation page.
#[component]
pub fn DrawerDoc() -> Element {
    let usage_source = r##"rsx! {
    Drawer {
        DrawerTrigger {
            Button { "Open Drawer" }
        }
        DrawerContent {
            DrawerHeader {
                DrawerTitle { "Title" }
                DrawerDescription { "Description" }
            }
            // Content here
            DrawerFooter {
                DrawerClose {
                    Button { "Close" }
                }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Drawer",
                description: "A slide-out panel component that appears from the edge of the screen.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::drawer::{{Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose}};".to_string(),
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
                        DrawerBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "Drawer" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "direction" }
                                td { class: "py-3 px-4 font-mono text-xs", "DrawerDirection" }
                                td { class: "py-3 px-4 font-mono text-xs", "Bottom" }
                                td { class: "py-3 px-4 text-muted-foreground", "Direction (Top, Bottom, Left, Right)" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_open_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<bool>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when state changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
