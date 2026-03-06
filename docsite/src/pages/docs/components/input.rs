//! Input component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::input::*;
use dioxus::prelude::*;

/// Input documentation page.
#[component]
pub fn InputDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Input",
                description: "A form input field for capturing user text input.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::input::{Input, InputSize, InputVariant};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Input { placeholder: "Enter text..." }
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
                        "A simple text input with placeholder."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("input_basic.rs".to_string()),
                        InputBasicExample {}
                    }
                }

                // With Label
                div { class: "space-y-4",
                    h3 { id: "with-label", class: "text-xl font-medium", "With Label" }
                    p { class: "text-muted-foreground",
                        "Pair inputs with labels for accessibility."
                    }
                    ComponentPreview {
                        source: WITH_LABEL_SOURCE.to_string(),
                        filename: Some("input_label.rs".to_string()),
                        InputWithLabelExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Inputs come in three sizes: small, medium, and large."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("input_sizes.rs".to_string()),
                        InputSizesExample {}
                    }
                }

                // States
                div { class: "space-y-4",
                    h3 { id: "states", class: "text-xl font-medium", "States" }
                    p { class: "text-muted-foreground",
                        "Inputs support disabled, error, and read-only states."
                    }
                    ComponentPreview {
                        source: STATES_SOURCE.to_string(),
                        filename: Some("input_states.rs".to_string()),
                        InputStatesExample {}
                    }
                }

                // With Icons
                div { class: "space-y-4",
                    h3 { id: "with-icons", class: "text-xl font-medium", "With Icons" }
                    p { class: "text-muted-foreground",
                        "Add icons to the left or right of the input."
                    }
                    ComponentPreview {
                        source: WITH_ICONS_SOURCE.to_string(),
                        filename: Some("input_icons.rs".to_string()),
                        InputWithIconsExample {}
                    }
                }

                // Input Types
                div { class: "space-y-4",
                    h3 { id: "types", class: "text-xl font-medium", "Input Types" }
                    p { class: "text-muted-foreground",
                        "Different input types for various data."
                    }
                    ComponentPreview {
                        source: TYPES_SOURCE.to_string(),
                        filename: Some("input_types.rs".to_string()),
                        InputTypesExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "input_type" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "\"text\"" }
                                td { class: "py-3 px-4 text-muted-foreground", "HTML input type" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "InputSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Medium" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size of the input" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "InputVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the input" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "placeholder" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "\"\"" }
                                td { class: "py-3 px-4 text-muted-foreground", "Placeholder text" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "\"\"" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "icon_left" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Element>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Left icon element" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "icon_right" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Element>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Right icon element" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<FormEvent>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Change event handler" }
                            }
                        }
                    }
                }
            }
        }
    }
}
