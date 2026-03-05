//! Combobox component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::combobox::*;
use dioxus::prelude::*;

/// Combobox documentation page.
#[component]
pub fn ComboboxDoc() -> Element {
    let usage_source = r##"rsx! {
    Combobox {
        ComboboxInput { placeholder: "Search..." }
        ComboboxContent {
            ComboboxList {
                ComboboxEmpty { "No results." }
                ComboboxItem { value: "1", label: "Option 1", "Option 1" }
                ComboboxItem { value: "2", label: "Option 2", "Option 2" }
            }
        }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Combobox",
                description: "A searchable select component combining an input with a dropdown list.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use lumen_blocks::components::combobox::{{Combobox, ComboboxInput, ComboboxContent, ComboboxList, ComboboxItem, ComboboxEmpty}};".to_string(),
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
                        ComboboxBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "Combobox" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "multiple" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Enable multi-select mode" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<Vec<String>>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled selected values" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<Vec<String>>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when selection changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
