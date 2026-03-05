//! Spinner component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::spinner::*;
use dioxus::prelude::*;

/// Spinner documentation page.
#[component]
pub fn SpinnerDoc() -> Element {
    let usage_source = r##"rsx! {
    Spinner {}

    // With size
    Spinner { size: SpinnerSize::Lg }

    // With custom class
    Spinner { class: "text-primary" }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Spinner",
                description: "A loading spinner for indicating progress or loading states.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::spinner::{{Spinner, SpinnerSize}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        SpinnerBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        SpinnerSizesExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "SpinnerSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Sm" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size of the spinner (Xs, Sm, Md, Lg, Xl)" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
