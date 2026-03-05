//! ButtonGroup component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::button_group::*;
use dioxus::prelude::*;

/// ButtonGroup documentation page.
#[component]
pub fn ButtonGroupDoc() -> Element {
    let usage_source = r##"rsx! {
    ButtonGroup {
        Button { "Left" }
        Button { "Center" }
        Button { "Right" }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Button Group",
                description: "A container for grouping multiple buttons together.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::button_group::{{ButtonGroup, ButtonGroupOrientation}};".to_string(),
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
                        ButtonGroupBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "vertical", class: "text-xl font-medium", "Vertical" }
                    ComponentPreview {
                        source: VERTICAL_SOURCE.to_string(),
                        ButtonGroupVerticalExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "ButtonGroupOrientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "Horizontal" }
                                td { class: "py-3 px-4 text-muted-foreground", "Layout orientation (Horizontal, Vertical)" }
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
