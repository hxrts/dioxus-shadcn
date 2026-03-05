//! Select component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::select::*;
use dioxus::prelude::*;

/// Select documentation page.
#[component]
pub fn SelectDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Select",
                description: "A dropdown select component for choosing from a list of options.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::select::{
    Select, SelectTrigger, SelectValue, SelectContent,
    SelectItem, SelectGroup, SelectLabel
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Select {
        SelectTrigger {
            SelectValue { placeholder: "Select..." }
        }
        SelectContent {
            SelectItem { value: "1", "Option 1" }
            SelectItem { value: "2", "Option 2" }
        }
    }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("select_basic.rs".to_string()),
                        SelectBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "with-groups", class: "text-xl font-medium", "With Groups" }
                    p { class: "text-muted-foreground", "Organize options into labeled groups." }
                    ComponentPreview {
                        source: WITH_GROUPS_SOURCE.to_string(),
                        filename: Some("select_groups.rs".to_string()),
                        SelectWithGroupsExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground", "Disable the entire select or individual items." }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("select_disabled.rs".to_string()),
                        SelectDisabledExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "Select" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "Signal<Option<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the select" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_value_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when value changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
