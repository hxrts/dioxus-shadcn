//! Progress component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::progress::*;
use dioxus::prelude::*;

/// Progress documentation page.
#[component]
pub fn ProgressDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Progress",
                description: "A progress bar indicating completion status.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::progress::{Progress, ProgressSize, ProgressVariant};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let progress = use_signal(|| 50.0);
rsx! {
    Progress { value: progress }
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
                        filename: Some("progress_basic.rs".to_string()),
                        ProgressBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("progress_sizes.rs".to_string()),
                        ProgressSizesExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "variants", class: "text-xl font-medium", "Variants" }
                    ComponentPreview {
                        source: VARIANTS_SOURCE.to_string(),
                        filename: Some("progress_variants.rs".to_string()),
                        ProgressVariantsExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "with-percentage", class: "text-xl font-medium", "With Percentage" }
                    ComponentPreview {
                        source: WITH_PERCENTAGE_SOURCE.to_string(),
                        filename: Some("progress_percentage.rs".to_string()),
                        ProgressWithPercentageExample {}
                    }
                }
            }

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
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<f64>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Current progress value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "max" }
                                td { class: "py-3 px-4 font-mono text-xs", "f64" }
                                td { class: "py-3 px-4 font-mono text-xs", "100.0" }
                                td { class: "py-3 px-4 text-muted-foreground", "Maximum value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "ProgressSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Medium" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "ProgressVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Color variant" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "show_percentage" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Show percentage text" }
                            }
                        }
                    }
                }
            }
        }
    }
}
