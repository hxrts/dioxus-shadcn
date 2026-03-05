//! Tabs component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::tabs::*;
use dioxus::prelude::*;

/// Tabs documentation page.
#[component]
pub fn TabsDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Tabs",
                description: "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::tabs::{
    Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Tabs { default_value: Some("account".to_string()),
        TabsList {
            TabsTrigger { value: "account".to_string(), "Account" }
            TabsTrigger { value: "password".to_string(), "Password" }
        }
        TabsContent { value: "account".to_string(),
            p { "Account settings here." }
        }
        TabsContent { value: "password".to_string(),
            p { "Password settings here." }
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
                        "A basic tabs component with card content."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("tabs_basic.rs".to_string()),
                        TabsBasicExample {}
                    }
                }

                // Line Variant
                div { class: "space-y-4",
                    h3 { id: "line-variant", class: "text-xl font-medium", "Line Variant" }
                    p { class: "text-muted-foreground",
                        "Use the line variant for a different visual style with an underline indicator."
                    }
                    ComponentPreview {
                        source: LINE_VARIANT_SOURCE.to_string(),
                        filename: Some("tabs_line.rs".to_string()),
                        TabsLineVariantExample {}
                    }
                }

                // Controlled
                div { class: "space-y-4",
                    h3 { id: "controlled", class: "text-xl font-medium", "Controlled" }
                    p { class: "text-muted-foreground",
                        "Use the value prop with on_value_change for controlled state."
                    }
                    ComponentPreview {
                        source: CONTROLLED_SOURCE.to_string(),
                        filename: Some("tabs_controlled.rs".to_string()),
                        TabsControlledExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                div { class: "space-y-6",
                    // Tabs props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "Tabs" }
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
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<Signal<String>>" }
                                        td { class: "py-3 px-4 font-mono text-xs", "None" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Controlled active tab" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                        td { class: "py-3 px-4 font-mono text-xs", "None" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Default active tab" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "on_value_change" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Callback<String>" }
                                        td { class: "py-3 px-4 font-mono text-xs", "-" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Called when tab changes" }
                                    }
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                        td { class: "py-3 px-4 font-mono text-xs", "TabsVariant" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Visual variant (Default, Line)" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                        td { class: "py-3 px-4 font-mono text-xs", "TabsOrientation" }
                                        td { class: "py-3 px-4 font-mono text-xs", "Horizontal" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Layout orientation" }
                                    }
                                }
                            }
                        }
                    }

                    // TabsTrigger props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "TabsTrigger" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                        th { class: "py-3 px-4 text-left font-medium", "Type" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr { class: "border-b border-border",
                                        td { class: "py-3 px-4 font-mono text-xs", "value" }
                                        td { class: "py-3 px-4 font-mono text-xs", "String" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Value identifying this tab (required)" }
                                    }
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                        td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Disable the trigger" }
                                    }
                                }
                            }
                        }
                    }

                    // TabsContent props
                    div { class: "space-y-2",
                        h3 { class: "text-lg font-medium", "TabsContent" }
                        div { class: "overflow-x-auto",
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b border-border",
                                        th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                        th { class: "py-3 px-4 text-left font-medium", "Type" }
                                        th { class: "py-3 px-4 text-left font-medium", "Description" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { class: "py-3 px-4 font-mono text-xs", "value" }
                                        td { class: "py-3 px-4 font-mono text-xs", "String" }
                                        td { class: "py-3 px-4 text-muted-foreground", "Value matching the trigger (required)" }
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
