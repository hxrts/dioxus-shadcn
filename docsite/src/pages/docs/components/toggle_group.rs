//! ToggleGroup component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::toggle_group::*;
use dioxus::prelude::*;

/// ToggleGroup documentation page.
#[component]
pub fn ToggleGroupDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Toggle Group",
                description: "A set of two-state buttons that can be toggled on or off.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::toggle_group::{
    ToggleGroup, ToggleGroupItem, ToggleGroupType,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let value = use_signal(|| vec!["center".to_string()]);

rsx! {
    ToggleGroup {
        toggle_type: ToggleGroupType::Single,
        value: value,
        on_value_change: move |v| value.set(v),

        ToggleGroupItem { value: "left", AlignLeft {} }
        ToggleGroupItem { value: "center", AlignCenter {} }
        ToggleGroupItem { value: "right", AlignRight {} }
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
                        "A single-selection toggle group for text alignment."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("toggle_group_basic.rs".to_string()),
                        ToggleGroupBasicExample {}
                    }
                }

                // Multiple Selection
                div { class: "space-y-4",
                    h3 { id: "multiple", class: "text-xl font-medium", "Multiple Selection" }
                    p { class: "text-muted-foreground",
                        "Allow multiple items to be selected at once."
                    }
                    ComponentPreview {
                        source: MULTIPLE_SOURCE.to_string(),
                        filename: Some("toggle_group_multiple.rs".to_string()),
                        ToggleGroupMultipleExample {}
                    }
                }

                // Outline Variant
                div { class: "space-y-4",
                    h3 { id: "outline", class: "text-xl font-medium", "Outline Variant" }
                    p { class: "text-muted-foreground",
                        "Use the outline variant for a bordered appearance."
                    }
                    ComponentPreview {
                        source: OUTLINE_SOURCE.to_string(),
                        filename: Some("toggle_group_outline.rs".to_string()),
                        ToggleGroupOutlineExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Toggle groups come in small, default, and large sizes."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("toggle_group_sizes.rs".to_string()),
                        ToggleGroupSizesExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "toggle_type" }
                                td { class: "py-3 px-4 font-mono text-xs", "ToggleGroupType" }
                                td { class: "py-3 px-4 font-mono text-xs", "Single" }
                                td { class: "py-3 px-4 text-muted-foreground", "Single or multiple selection" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<Vec<String>>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled selected values" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Vec<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial values (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "ToggleVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual variant for all items" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "ToggleSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size for all items" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disable all items" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_value_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<Vec<String>>" }
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
