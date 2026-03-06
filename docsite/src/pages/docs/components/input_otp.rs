//! InputOTP component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::input_otp::*;
use dioxus::prelude::*;

/// InputOTP documentation page.
#[component]
pub fn InputOTPDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Input OTP",
                description: "A one-time password input component with individual character slots.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::input_otp::{
    InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator,
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"let otp = use_signal(String::new);

rsx! {
    InputOTP {
        value: otp,
        max_length: 6,
        on_complete: move |code| { /* verify */ },

        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
        }
        InputOTPSeparator {}
        InputOTPGroup {
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
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
                        "A 6-digit OTP input with a separator."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("input_otp_basic.rs".to_string()),
                        InputOTPBasicExample {}
                    }
                }

                // Pattern
                div { class: "space-y-4",
                    h3 { id: "pattern", class: "text-xl font-medium", "Pattern" }
                    p { class: "text-muted-foreground",
                        "Restrict input to only digits."
                    }
                    ComponentPreview {
                        source: PATTERN_SOURCE.to_string(),
                        filename: Some("input_otp_pattern.rs".to_string()),
                        InputOTPPatternExample {}
                    }
                }

                // Disabled
                div { class: "space-y-4",
                    h3 { id: "disabled", class: "text-xl font-medium", "Disabled" }
                    p { class: "text-muted-foreground",
                        "A disabled OTP input."
                    }
                    ComponentPreview {
                        source: DISABLED_SOURCE.to_string(),
                        filename: Some("input_otp_disabled.rs".to_string()),
                        InputOTPDisabledExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // InputOTP
                h3 { class: "text-lg font-medium mt-6", "InputOTP" }
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
                                td { class: "py-3 px-4 text-muted-foreground", "Controlled value" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "default_value" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Initial value (uncontrolled)" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "max_length" }
                                td { class: "py-3 px-4 font-mono text-xs", "usize" }
                                td { class: "py-3 px-4 font-mono text-xs", "6" }
                                td { class: "py-3 px-4 text-muted-foreground", "Maximum number of characters" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "on_change" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when value changes" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "on_complete" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<String>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when OTP is complete" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "pattern" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Input validation pattern" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disable the input" }
                            }
                        }
                    }
                }

                // InputOTPSlot
                h3 { class: "text-lg font-medium mt-6", "InputOTPSlot" }
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
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "index" }
                                td { class: "py-3 px-4 font-mono text-xs", "usize" }
                                td { class: "py-3 px-4 font-mono text-xs", "required" }
                                td { class: "py-3 px-4 text-muted-foreground", "Slot index (0-based)" }
                            }
                        }
                    }
                }
            }
        }
    }
}
