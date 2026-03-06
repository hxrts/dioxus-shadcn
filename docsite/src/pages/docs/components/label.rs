//! Label component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::label::*;
use dioxus::prelude::*;

/// Label documentation page.
#[component]
pub fn LabelDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Label",
                description: "A label element for form controls with accessibility support.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::label::{Label, LabelSize};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Label { "Username" }
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
                        "A label paired with an input field."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("label_basic.rs".to_string()),
                        LabelBasicExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Labels come in three sizes."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("label_sizes.rs".to_string()),
                        LabelSizesExample {}
                    }
                }

                // Required
                div { class: "space-y-4",
                    h3 { id: "required", class: "text-xl font-medium", "Required" }
                    p { class: "text-muted-foreground",
                        "Show a required indicator for mandatory fields."
                    }
                    ComponentPreview {
                        source: REQUIRED_SOURCE.to_string(),
                        filename: Some("label_required.rs".to_string()),
                        LabelRequiredExample {}
                    }
                }

                // With Checkbox
                div { class: "space-y-4",
                    h3 { id: "with-checkbox", class: "text-xl font-medium", "With Checkbox" }
                    p { class: "text-muted-foreground",
                        "Associate labels with checkboxes using for_id."
                    }
                    ComponentPreview {
                        source: WITH_CHECKBOX_SOURCE.to_string(),
                        filename: Some("label_checkbox.rs".to_string()),
                        LabelWithCheckboxExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "for_id" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<Option<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "ID of associated control" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<LabelSize>" }
                                td { class: "py-3 px-4 font-mono text-xs", "Medium" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size of the label" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "required" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Show required indicator" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Display as disabled" }
                            }
                        }
                    }
                }
            }
        }
    }
}
