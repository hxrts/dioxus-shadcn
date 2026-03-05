//! Pagination component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::pagination::*;
use dioxus::prelude::*;

/// Pagination documentation page.
#[component]
pub fn PaginationDoc() -> Element {
    let usage_source = r##"rsx! {
    Pagination {
        PaginationContent {
            PaginationItem {
                PaginationPrevious { href: "#" }
            }
            PaginationItem {
                PaginationLink { href: "#", is_active: true, "1" }
            }
            PaginationItem {
                PaginationLink { href: "#", "2" }
            }
            PaginationItem {
                PaginationNext { href: "#" }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Pagination",
                description: "Pagination with page navigation, next and previous links.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::pagination::{{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis}};".to_string(),
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
                        "A pagination component with previous/next links and page numbers."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("pagination_basic.rs".to_string()),
                        PaginationBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // PaginationLink
                h3 { class: "text-lg font-medium mt-6", "PaginationLink" }
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
                                td { class: "py-3 px-4 text-muted-foreground", "The href for the link" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "is_active" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether this is the current page" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "PaginationLinkSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant (Default, Icon)" }
                            }
                        }
                    }
                }

                // PaginationPrevious / PaginationNext
                h3 { class: "text-lg font-medium mt-6", "PaginationPrevious / PaginationNext" }
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
                                td { class: "py-3 px-4 text-muted-foreground", "The href for the link" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "show_label" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether to show the label text" }
                            }
                        }
                    }
                }
            }
        }
    }
}
