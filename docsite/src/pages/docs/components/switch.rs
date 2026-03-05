//! Switch component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::switch::*;
use dioxus::prelude::*;

/// Switch documentation page.
#[component]
pub fn SwitchDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Switch",
                description: "A toggle switch for on/off states.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::switch::{Switch, SwitchSize};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let enabled = use_signal(|| false);
rsx! {
    Switch { checked: enabled, on_checked_change: move |v| enabled.set(v) }
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
                        filename: Some("switch_basic.rs".to_string()),
                        SwitchBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "with-label", class: "text-xl font-medium", "With Label" }
                    ComponentPreview {
                        source: WITH_LABEL_SOURCE.to_string(),
                        filename: Some("switch_label.rs".to_string()),
                        SwitchWithLabelExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("switch_sizes.rs".to_string()),
                        SwitchSizesExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("switch_disabled.rs".to_string()),
                        SwitchDisabledExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "checked" }
                                td { class: "py-3 px-4 font-mono text-xs", "Signal<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled checked state" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "SwitchSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "ReadSignal<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the switch" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_checked_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "EventHandler<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when toggled" }
                            }
                        }
                    }
                }
            }
        }
    }
}
