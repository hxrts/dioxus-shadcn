//! Badge component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::badge::*;
use dioxus::prelude::*;

/// Badge documentation page.
#[component]
pub fn BadgeDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Badge",
                description: "Displays a small status indicator or label.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::badge::{Badge, BadgeVariant};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Badge { "New" }
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
                        "Badges come in six variants matching the button variants."
                    }
                    ComponentPreview {
                        source: VARIANTS_SOURCE.to_string(),
                        filename: Some("badge_variants.rs".to_string()),
                        BadgeVariantsExample {}
                    }
                }

                // With Icons
                div { class: "space-y-4",
                    h3 { id: "with-icons", class: "text-xl font-medium", "With Icons" }
                    p { class: "text-muted-foreground",
                        "Add icons to provide additional visual context."
                    }
                    ComponentPreview {
                        source: WITH_ICON_SOURCE.to_string(),
                        filename: Some("badge_icons.rs".to_string()),
                        BadgeWithIconExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "BadgeVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual style variant" }
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
