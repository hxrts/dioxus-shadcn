//! HoverCard component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::hover_card::*;
use dioxus::prelude::*;

/// HoverCard documentation page.
#[component]
pub fn HoverCardDoc() -> Element {
    let usage_source = r##"rsx! {
    HoverCard {
        HoverCardTrigger {
            a { href: "#", "Trigger" }
        }
        HoverCardContent {
            "Content"
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Hover Card",
                description: "For sighted users to preview content available behind a link.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use dioxus_shadcn::components::hover_card::{{HoverCard, HoverCardContent, HoverCardTrigger}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
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
                        "A hover card showing user profile information."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("hover_card_basic.rs".to_string()),
                        HoverCardBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // HoverCardContent
                h3 { class: "text-lg font-medium mt-6", "HoverCardContent" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "side" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<HoverCardSide>" }
                                td { class: "py-3 px-4 font-mono text-xs", "Top" }
                                td { class: "py-3 px-4 text-muted-foreground", "Side to show content" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "align" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<HoverCardAlign>" }
                                td { class: "py-3 px-4 font-mono text-xs", "Center" }
                                td { class: "py-3 px-4 text-muted-foreground", "Alignment of content" }
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
