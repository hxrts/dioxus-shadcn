//! Checkbox component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::checkbox::*;
use dioxus::prelude::*;

/// Checkbox documentation page.
#[component]
pub fn CheckboxDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Checkbox",
                description: "A control that allows the user to toggle between checked and not checked.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::checkbox::{Checkbox, CheckboxSize};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::checkbox::Checkbox;
use lumen_blocks::components::label::Label;

rsx! {
    div { class: "flex items-center space-x-2",
        Checkbox { id: Some("terms".to_string()) }
        Label { for_id: "terms", "Accept terms" }
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
                        "A simple checkbox with a label."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("checkbox_basic.rs".to_string()),
                        CheckboxBasicExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Checkboxes come in three sizes: small, medium (default), and large."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("checkbox_sizes.rs".to_string()),
                        CheckboxSizesExample {}
                    }
                }

                // Controlled
                div { class: "space-y-4",
                    h3 { id: "controlled", class: "text-xl font-medium", "Controlled" }
                    p { class: "text-muted-foreground",
                        "Use the checked prop with on_checked_change for controlled state."
                    }
                    ComponentPreview {
                        source: CONTROLLED_SOURCE.to_string(),
                        filename: Some("checkbox_controlled.rs".to_string()),
                        CheckboxControlledExample {}
                    }
                }

                // Disabled
                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground",
                        "Disabled checkboxes cannot be interacted with."
                    }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("checkbox_disabled.rs".to_string()),
                        CheckboxDisabledExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "checked" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<bool>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled checked state" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_checked" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial checked state (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "on_checked_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "EventHandler<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when checked changes" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "CheckboxSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Medium" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size of the checkbox" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the checkbox" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "name" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Name for form submission" }
                            }
                        }
                    }
                }
            }
        }
    }
}
