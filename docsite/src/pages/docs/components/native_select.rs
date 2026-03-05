//! NativeSelect component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::native_select::*;
use dioxus::prelude::*;

/// NativeSelect documentation page.
#[component]
pub fn NativeSelectDoc() -> Element {
    let usage_source = r##"rsx! {
    NativeSelect {
        on_change: move |value| println!("Selected: {value}"),

        NativeSelectOption { value: "", "Select..." }
        NativeSelectOption { value: "a", "Option A" }
        NativeSelectOption { value: "b", "Option B" }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Native Select",
                description: "A styled native HTML select element with proper accessibility.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::native_select::{{NativeSelect, NativeSelectOption, NativeSelectOptGroup}};".to_string(),
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
                        NativeSelectBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "groups", class: "text-xl font-medium", "With Groups" }
                    ComponentPreview {
                        source: GROUPS_SOURCE.to_string(),
                        NativeSelectGroupsExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "NativeSelectSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant (Sm, Default)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Whether the select is disabled" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<EventHandler<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when value changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
