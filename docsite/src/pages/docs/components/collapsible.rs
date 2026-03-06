//! Collapsible component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::collapsible::*;
use dioxus::prelude::*;

/// Collapsible documentation page.
#[component]
pub fn CollapsibleDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Collapsible",
                description: "An interactive component that expands/collapses content.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleTrigger,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Collapsible {
        CollapsibleTrigger { "Toggle" }
        CollapsibleContent {
            "Hidden content"
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
                        "A simple collapsible section."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("collapsible_basic.rs".to_string()),
                        CollapsibleBasicExample {}
                    }
                }

                // Repository
                div { class: "space-y-4",
                    h3 { id: "repository", class: "text-xl font-medium", "Repository" }
                    p { class: "text-muted-foreground",
                        "A collapsible showing a list of packages."
                    }
                    ComponentPreview {
                        source: REPOSITORY_SOURCE.to_string(),
                        filename: Some("collapsible_repository.rs".to_string()),
                        CollapsibleRepositoryExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // Collapsible
                h3 { class: "text-lg font-medium mt-6", "Collapsible" }
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

                // CollapsibleTrigger
                h3 { class: "text-lg font-medium mt-6", "CollapsibleTrigger" }
                p { class: "text-muted-foreground text-sm", "The button that toggles the content." }

                // CollapsibleContent
                h3 { class: "text-lg font-medium mt-6", "CollapsibleContent" }
                p { class: "text-muted-foreground text-sm", "The content area that is shown/hidden." }
            }
        }
    }
}
