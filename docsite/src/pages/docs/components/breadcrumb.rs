//! Breadcrumb component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::breadcrumb::*;
use dioxus::prelude::*;

/// Breadcrumb documentation page.
#[component]
pub fn BreadcrumbDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Breadcrumb",
                description: "Displays the path to the current resource using a hierarchy of links.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList,
    BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem {
                BreadcrumbLink { href: "/", "Home" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbPage { "Current" }
            }
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
                        "A simple breadcrumb navigation."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("breadcrumb_basic.rs".to_string()),
                        BreadcrumbBasicExample {}
                    }
                }

                // With Ellipsis
                div { class: "space-y-4",
                    h3 { id: "ellipsis", class: "text-xl font-medium", "With Ellipsis" }
                    p { class: "text-muted-foreground",
                        "Use ellipsis to indicate collapsed items in deep paths."
                    }
                    ComponentPreview {
                        source: ELLIPSIS_SOURCE.to_string(),
                        filename: Some("breadcrumb_ellipsis.rs".to_string()),
                        BreadcrumbEllipsisExample {}
                    }
                }

                // Custom Separator
                div { class: "space-y-4",
                    h3 { id: "custom-separator", class: "text-xl font-medium", "Custom Separator" }
                    p { class: "text-muted-foreground",
                        "Use a custom separator character instead of the default chevron."
                    }
                    ComponentPreview {
                        source: CUSTOM_SEPARATOR_SOURCE.to_string(),
                        filename: Some("breadcrumb_separator.rs".to_string()),
                        BreadcrumbCustomSeparatorExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // BreadcrumbLink
                h3 { class: "text-lg font-medium mt-6", "BreadcrumbLink" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "href" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Link destination" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "on_click" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<()>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Click handler" }
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

                // Other components
                h3 { class: "text-lg font-medium mt-6", "Other Components" }
                p { class: "text-muted-foreground text-sm mb-2", "All breadcrumb components accept optional " code { "class" } " prop." }
                ul { class: "list-disc pl-6 text-sm text-muted-foreground space-y-1",
                    li { code { "Breadcrumb" } " - Root navigation container" }
                    li { code { "BreadcrumbList" } " - Ordered list wrapper" }
                    li { code { "BreadcrumbItem" } " - Individual item wrapper" }
                    li { code { "BreadcrumbPage" } " - Current page (non-clickable)" }
                    li { code { "BreadcrumbSeparator" } " - Separator between items" }
                    li { code { "BreadcrumbEllipsis" } " - Collapsed items indicator" }
                }
            }
        }
    }
}
