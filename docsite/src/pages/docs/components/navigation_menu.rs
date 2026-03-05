//! NavigationMenu component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::navigation_menu::*;
use dioxus::prelude::*;

/// NavigationMenu documentation page.
#[component]
pub fn NavigationMenuDoc() -> Element {
    let usage_source = r##"rsx! {
    NavigationMenu {
        NavigationMenuList {
            NavigationMenuItem { value: "getting-started",
                NavigationMenuTrigger { "Getting started" }
                NavigationMenuContent {
                    NavigationMenuLink { href: "/docs",
                        "Introduction"
                    }
                }
            }
            NavigationMenuItem {
                NavigationMenuLink { href: "/docs",
                    "Documentation"
                }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Navigation Menu",
                description: "A collection of links for navigating websites.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::navigation_menu::{{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink}};".to_string(),
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
                        "A navigation menu with dropdown content panels."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("navigation_menu_basic.rs".to_string()),
                        NavigationMenuBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // NavigationMenu
                h3 { class: "text-lg font-medium mt-6", "NavigationMenu" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<Option<String>>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled active value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Default value for uncontrolled mode" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "use_viewport" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "true" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether to use viewport mode" }
                            }
                        }
                    }
                }

                // NavigationMenuLink
                h3 { class: "text-lg font-medium mt-6", "NavigationMenuLink" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "active" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether this link is active" }
                            }
                        }
                    }
                }
            }
        }
    }
}
