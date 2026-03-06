//! Menubar component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::menubar::*;
use dioxus::prelude::*;

/// Menubar documentation page.
#[component]
pub fn MenubarDoc() -> Element {
    let usage_source = r##"rsx! {
    Menubar {
        MenubarMenu { index: ReadSignal::new(Signal::new(0)),
            MenubarTrigger { "File" }
            MenubarContent {
                MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "new",
                    "New File"
                    MenubarShortcut { "Cmd+N" }
                }
                MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "open",
                    "Open"
                }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Menubar",
                description: "A visually persistent menu common in desktop applications.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use dioxus_shadcn::components::menubar::{{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem}};".to_string(),
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
                        "A menubar with multiple menus and keyboard shortcuts."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("menubar_basic.rs".to_string()),
                        MenubarBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // MenubarMenu
                h3 { class: "text-lg font-medium mt-6", "MenubarMenu" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "index" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<usize>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Index of this menu in the menubar" }
                            }
                        }
                    }
                }

                // MenubarItem
                h3 { class: "text-lg font-medium mt-6", "MenubarItem" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "index" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<usize>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Index of this item in the menu" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "The value of this item" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "destructive" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Renders item in destructive style" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "inset" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Add inset padding" }
                            }
                        }
                    }
                }
            }
        }
    }
}
