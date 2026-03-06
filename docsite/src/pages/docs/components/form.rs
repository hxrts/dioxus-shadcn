//! Form component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::form::*;
use dioxus::prelude::*;

/// Form documentation page.
#[component]
pub fn FormDoc() -> Element {
    let usage_source = r##"rsx! {
    Form {
        on_submit: move |_| {
            // Handle submission
        },

        FormField { name: "email",
            FormLabel { "Email" }
            FormControl {
                Input { placeholder: "Enter your email" }
            }
            FormDescription { "Your email address." }
            FormMessage {}
        }

        Button { button_type: "submit", "Submit" }
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Form",
                description: "Building forms with validation and accessible error messages.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use dioxus_shadcn::components::form::{{Form, FormField, FormLabel, FormControl, FormDescription, FormMessage}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
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
                        "A form with labeled inputs and descriptions."
                    }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("form_basic.rs".to_string()),
                        FormBasicExample {}
                    }
                }
            }

            // API Reference
            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                // Form
                h3 { class: "text-lg font-medium mt-6", "Form" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "on_submit" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<Callback<FormEvent>>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Called when form is submitted" }
                            }
                        }
                    }
                }

                // FormField
                h3 { class: "text-lg font-medium mt-6", "FormField" }
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
                                td { class: "py-3 px-4 font-mono text-xs", "name" }
                                td { class: "py-3 px-4 font-mono text-xs", "String" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "The field name for error mapping" }
                            }
                        }
                    }
                }

                // Hooks
                h3 { class: "text-lg font-medium mt-6", "Hooks" }
                p { class: "text-muted-foreground",
                    "Use "
                    code { class: "px-1.5 py-0.5 rounded bg-muted font-mono text-xs", "use_form()" }
                    " to access form context for programmatic validation."
                }
            }
        }
    }
}
