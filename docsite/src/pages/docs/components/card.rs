//! Card component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::card::*;
use dioxus::prelude::*;

/// Card documentation page.
#[component]
pub fn CardDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Card",
                description: "A container for grouping related content and actions.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::card::{
    Card, CardHeader, CardTitle, CardDescription,
    CardContent, CardFooter, CardAction
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Card {
        CardHeader {
            CardTitle { "Title" }
            CardDescription { "Description" }
        }
        CardContent {
            p { "Content" }
        }
        CardFooter {
            Button { "Action" }
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
                        "A simple card with header, content, and footer."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("card_basic.rs".to_string()),
                        CardBasicExample {}
                    }
                }

                // With Form
                div { class: "space-y-4",
                    h3 { id: "with-form", class: "text-xl font-medium", "With Form" }
                    p { class: "text-muted-foreground",
                        "Cards work well as containers for forms."
                    }
                    ComponentPreview {
                        source: WITH_FORM_SOURCE.to_string(),
                        filename: Some("card_form.rs".to_string()),
                        CardWithFormExample {}
                    }
                }

                // With Action
                div { class: "space-y-4",
                    h3 { id: "with-action", class: "text-xl font-medium", "With Action" }
                    p { class: "text-muted-foreground",
                        "Use CardAction to add an action button in the header."
                    }
                    ComponentPreview {
                        source: WITH_ACTION_SOURCE.to_string(),
                        filename: Some("card_action.rs".to_string()),
                        CardWithActionExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "Card" }
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
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                        }
                    }
                }

                h3 { class: "text-lg font-medium mt-6", "Sub-components" }
                p { class: "text-muted-foreground",
                    "CardHeader, CardTitle, CardDescription, CardContent, CardFooter, and CardAction all accept a class prop for additional styling."
                }
            }
        }
    }
}
