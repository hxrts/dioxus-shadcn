//! Toast component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::toast::*;
use dioxus::prelude::*;
use lumen_blocks::components::toast::ToastProvider;

/// Toast documentation page.
#[component]
pub fn ToastDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Toast",
                description: "A succinct message that is displayed temporarily to communicate feedback to the user.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::toast::{
    ToastProvider, use_toast, ToastOptions, ToastPosition,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                p { class: "text-muted-foreground",
                    "First, wrap your app with ToastProvider. Then use the use_toast hook to show toasts."
                }
                CodeBlock {
                    source: r#"// In your root component
rsx! {
    ToastProvider {
        // Your app content
        MyApp {}
    }
}

// In any child component
fn MyComponent() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            on_click: move |_| {
                toast.success("Saved!", None);
            },
            "Save"
        }
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
                        "A simple toast notification."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: BASIC_SOURCE.to_string(),
                            filename: Some("toast_basic.rs".to_string()),
                            ToastBasicExample {}
                        }
                    }
                }

                // Variants
                div { class: "space-y-4",
                    h3 { id: "variants", class: "text-xl font-medium", "Variants" }
                    p { class: "text-muted-foreground",
                        "Toasts come in four variants: success, error, warning, and info."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: VARIANTS_SOURCE.to_string(),
                            filename: Some("toast_variants.rs".to_string()),
                            ToastVariantsExample {}
                        }
                    }
                }

                // With Description
                div { class: "space-y-4",
                    h3 { id: "with-description", class: "text-xl font-medium", "With Description" }
                    p { class: "text-muted-foreground",
                        "Add a description for more context."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: WITH_DESCRIPTION_SOURCE.to_string(),
                            filename: Some("toast_description.rs".to_string()),
                            ToastWithDescriptionExample {}
                        }
                    }
                }

                // Custom Duration
                div { class: "space-y-4",
                    h3 { id: "duration", class: "text-xl font-medium", "Custom Duration" }
                    p { class: "text-muted-foreground",
                        "Control how long toasts stay visible, or make them permanent."
                    }
                    ToastProvider {
                        ComponentPreview {
                            source: CUSTOM_DURATION_SOURCE.to_string(),
                            filename: Some("toast_duration.rs".to_string()),
                            ToastDurationExample {}
                        }
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                div { class: "space-y-6",
                    // ToastProvider props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "ToastProvider" }
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
                                        td { class: "py-3 px-4 font-mono text-xs", "default_duration" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Duration" }
                                        td { class: "py-3 px-4 font-mono text-xs", "5s" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Default toast duration" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "max_toasts" }
                                        td { class: "py-3 px-4 font-mono text-xs", "usize" }
                                        td { class: "py-3 px-4 font-mono text-xs", "10" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Max visible toasts" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "position" }
                                        td { class: "py-3 px-4 font-mono text-xs", "ToastPosition" }
                                        td { class: "py-3 px-4 font-mono text-xs", "TopRight" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Position on screen" }
                                    }
                                }
                            }
                        }
                    }

                    // use_toast methods
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "use_toast() Methods" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Method" }
                                        th { class: "py-3 px-4 text-left font-medium", "Arguments" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "success" }
                                        td { class: "py-3 px-4 font-mono text-xs", "(title, options)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Show success toast" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "error" }
                                        td { class: "py-3 px-4 font-mono text-xs", "(title, options)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Show error toast" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "warning" }
                                        td { class: "py-3 px-4 font-mono text-xs", "(title, options)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Show warning toast" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "info" }
                                        td { class: "py-3 px-4 font-mono text-xs", "(title, options)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Show info toast" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "remove" }
                                        td { class: "py-3 px-4 font-mono text-xs", "(id)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Remove toast by ID" }
                                    }
                                }
                            }
                        }
                    }

                    // ToastOptions
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "ToastOptions" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Method" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "with_description(text)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Add description text" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", ".duration(Duration)" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Set custom duration" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", ".permanent()" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Make toast permanent (no auto-dismiss)" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
