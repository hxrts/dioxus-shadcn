//! Slider component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::slider::*;
use dioxus::prelude::*;

/// Slider documentation page.
#[component]
pub fn SliderDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Slider",
                description: "An input component for selecting a value from a range.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::slider::Slider;"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let value = use_signal(|| 50.0);

rsx! {
    Slider {
        value: value,
        on_value_change: move |v| value.set(v),
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
                        "A basic slider with a label showing the current value."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("slider_basic.rs".to_string()),
                        SliderBasicExample {}
                    }
                }

                // Custom Range
                div { class: "space-y-4",
                    h3 { id: "range", class: "text-xl font-medium", "Custom Range" }
                    p { class: "text-muted-foreground",
                        "Configure the min, max, and step values."
                    }
                    ComponentPreview {
                        source: RANGE_SOURCE.to_string(),
                        filename: Some("slider_range.rs".to_string()),
                        SliderRangeExample {}
                    }
                }

                // Disabled
                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground",
                        "A disabled slider cannot be interacted with."
                    }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("slider_disabled.rs".to_string()),
                        SliderDisabledExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<f64>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "50.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial value (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "min" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "0.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Minimum value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "max" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "100.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Maximum value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "step" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "1.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Step increment" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disable the slider" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_value_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<f64>" }
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
