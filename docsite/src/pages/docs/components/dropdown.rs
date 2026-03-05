//! Dropdown component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::dropdown::*;
use dioxus::prelude::*;

/// Dropdown documentation page.
#[component]
pub fn DropdownDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Dropdown Menu",
                description: "Displays a menu to the user—such as a set of actions or functions—triggered by a button.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::dropdown::{
    Dropdown, DropdownContent, DropdownItem, DropdownLabel,
    DropdownSeparator, DropdownTrigger,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Dropdown {
        DropdownTrigger {
            Button { "Open Menu" }
        }
        DropdownContent {
            DropdownItem { value: ReadSignal::new(Signal::new("item1")), "Item 1" }
            DropdownItem { value: ReadSignal::new(Signal::new("item2")), "Item 2" }
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
                        "A simple dropdown menu with items."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("dropdown_basic.rs".to_string()),
                        DropdownBasicExample {}
                    }
                }

                // With Icons
                div { class: "space-y-4",
                    h3 { id: "with-icons", class: "text-xl font-medium", "With Icons" }
                    p { class: "text-muted-foreground",
                        "Dropdown items can include icons using the icon prop."
                    }
                    ComponentPreview {
                        source: WITH_ICONS_SOURCE.to_string(),
                        filename: Some("dropdown_icons.rs".to_string()),
                        DropdownWithIconsExample {}
                    }
                }

                // Destructive Items
                div { class: "space-y-4",
                    h3 { id: "destructive", class: "text-xl font-medium", "Destructive Items" }
                    p { class: "text-muted-foreground",
                        "Use the destructive prop for dangerous actions like delete."
                    }
                    ComponentPreview {
                        source: r#"DropdownItem {
    destructive: true,
    "Delete"
}"#.to_string(),
                        filename: Some("dropdown_destructive.rs".to_string()),
                        DropdownDestructiveExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                div { class: "space-y-6",
                    // Dropdown props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "Dropdown" }
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
                                        td { class: "py-3 px-4 font-mono text-xs", "default_open" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 font-mono text-xs", "false" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Open by default" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 font-mono text-xs", "false" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Disable the dropdown" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "aria_label" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                        td { class: "py-3 px-4 font-mono text-xs", "None" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Accessible label" }
                                    }
                                }
                            }
                        }
                    }

                    // DropdownItem props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "DropdownItem" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                        th { class: "py-3 px-4 text-left font-medium", "Type" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "value" }
                                        td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<T>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Value of the item" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "on_select" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Callback<T>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Called when selected" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Disable the item" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "destructive" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Red destructive style" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "icon" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<Element>" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Icon element" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
