//! Toggle component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::toggle::*;
use dioxus::prelude::*;

/// Toggle documentation page.
#[component]
pub fn ToggleDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Toggle",
                description: "A two-state button that can be either on or off.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::toggle::{Toggle, ToggleVariant, ToggleSize};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let pressed = use_signal(|| false);

rsx! {
    Toggle {
        pressed: pressed,
        on_pressed_change: move |v| pressed.set(v),
        Bold { class: "size-4" }
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
                        "A simple toggle button with an icon."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("toggle_basic.rs".to_string()),
                        ToggleBasicExample {}
                    }
                }

                // Variants
                div { class: "space-y-4",
                    h3 { id: "variants", class: "text-xl font-medium", "Variants" }
                    p { class: "text-muted-foreground",
                        "Toggle supports default and outline variants."
                    }
                    ComponentPreview {
                        source: VARIANTS_SOURCE.to_string(),
                        filename: Some("toggle_variants.rs".to_string()),
                        ToggleVariantsExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Toggle comes in small, default, and large sizes."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("toggle_sizes.rs".to_string()),
                        ToggleSizesExample {}
                    }
                }

                // With Text
                div { class: "space-y-4",
                    h3 { id: "with-text", class: "text-xl font-medium", "With Text" }
                    p { class: "text-muted-foreground",
                        "Include text alongside the icon."
                    }
                    ComponentPreview {
                        source: WITH_TEXT_SOURCE.to_string(),
                        filename: Some("toggle_with_text.rs".to_string()),
                        ToggleWithTextExample {}
                    }
                }

                // Disabled
                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground",
                        "A disabled toggle cannot be interacted with."
                    }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("toggle_disabled.rs".to_string()),
                        ToggleDisabledExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "pressed" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<bool>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled pressed state" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_pressed" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial state (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "ToggleVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "ToggleSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disable the toggle" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_pressed_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<bool>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when state changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
