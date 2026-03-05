//! Alert component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::alert::*;
use dioxus::prelude::*;

/// Alert documentation page.
#[component]
pub fn AlertDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Alert",
                description: "Displays a callout for user attention with contextual feedback.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::alert::{Alert, AlertTitle, AlertDescription, AlertVariant};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Alert {
        AlertTitle { "Heads up!" }
        AlertDescription { "You can add components to your app using the CLI." }
    }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Examples
            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                // Variants
                div { class: "space-y-4",
                    h3 { id: "variants", class: "text-xl font-medium", "Variants" }
                    p { class: "text-muted-foreground",
                        "Alerts come in four variants: default, destructive, success, and warning."
                    }
                    ComponentPreview {
                        source: VARIANTS_SOURCE.to_string(),
                        filename: Some("alert_variants.rs".to_string()),
                        AlertVariantsExample {}
                    }
                }

                // Without Icon
                div { class: "space-y-4",
                    h3 { id: "without-icon", class: "text-xl font-medium", "Without Icon" }
                    p { class: "text-muted-foreground",
                        "Set show_icon to false to hide the default variant icon."
                    }
                    ComponentPreview {
                        source: NO_ICON_SOURCE.to_string(),
                        filename: Some("alert_no_icon.rs".to_string()),
                        AlertNoIconExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "AlertVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual style variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "show_icon" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Show the variant icon" }
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
