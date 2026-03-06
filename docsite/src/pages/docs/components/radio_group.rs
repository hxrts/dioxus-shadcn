//! RadioGroup component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::radio_group::*;
use dioxus::prelude::*;

/// RadioGroup documentation page.
#[component]
pub fn RadioGroupDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Radio Group",
                description: "A set of checkable buttons where only one can be checked at a time.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::radio_group::{
    RadioGroup, RadioGroupItem, RadioGroupOrientation,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let selected = use_signal(|| "option1".to_string());

rsx! {
    RadioGroup {
        value: selected,
        on_value_change: move |v| selected.set(v),

        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "option1", id: "r1" }
            Label { r#for: "r1", "Option 1" }
        }
        div { class: "flex items-center space-x-2",
            RadioGroupItem { value: "option2", id: "r2" }
            Label { r#for: "r2", "Option 2" }
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
                        "A vertical radio group for selecting a single option."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("radio_group_basic.rs".to_string()),
                        RadioGroupBasicExample {}
                    }
                }

                // Horizontal
                div { class: "space-y-4",
                    h3 { id: "horizontal", class: "text-xl font-medium", "Horizontal" }
                    p { class: "text-muted-foreground",
                        "Use horizontal orientation for inline layout."
                    }
                    ComponentPreview {
                        source: HORIZONTAL_SOURCE.to_string(),
                        filename: Some("radio_group_horizontal.rs".to_string()),
                        RadioGroupHorizontalExample {}
                    }
                }

                // Disabled
                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground",
                        "Disable the entire radio group."
                    }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("radio_group_disabled.rs".to_string()),
                        RadioGroupDisabledExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled selected value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial value (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "name" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "auto" }
                                td { class: "py-3 px-4 text-muted-foreground", "Form field name" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "RadioGroupOrientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "Vertical" }
                                td { class: "py-3 px-4 text-muted-foreground", "Layout direction" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disable all items" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_value_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when selection changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
