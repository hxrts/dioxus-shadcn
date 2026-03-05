//! Button component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::button::*;
use dioxus::prelude::*;

/// Button documentation page.
#[component]
pub fn ButtonDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Button",
                description: "Displays a button or a component that looks like a button.",
            }

            // Installation
            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use dioxus_shadcn::components::button::{Button, ButtonVariant, ButtonSize};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Usage
            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Button {
        variant: ButtonVariant::Default,
        "Click me"
    }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Examples
            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                // Variants
                div { class: "space-y-4",
                    h3 { id: "variants", class: "text-xl font-medium", "Variants" }
                    p { class: "text-muted-foreground",
                        "Use the variant prop to change the visual style of the button."
                    }
                    ComponentPreview {
                        source: VARIANTS_SOURCE.to_string(),
                        filename: Some("button_variants.rs".to_string()),
                        ButtonVariantsExample {}
                    }
                }

                // Sizes
                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    p { class: "text-muted-foreground",
                        "Buttons come in three sizes: small, medium (default), and large."
                    }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("button_sizes.rs".to_string()),
                        ButtonSizesExample {}
                    }
                }

                // States
                div { class: "space-y-4",
                    h3 { id: "states", class: "text-xl font-medium", "States" }
                    p { class: "text-muted-foreground",
                        "Buttons can be disabled or show a loading state."
                    }
                    ComponentPreview {
                        source: STATES_SOURCE.to_string(),
                        filename: Some("button_states.rs".to_string()),
                        ButtonStatesExample {}
                    }
                }

                // With Icons
                div { class: "space-y-4",
                    h3 { id: "with-icons", class: "text-xl font-medium", "With Icons" }
                    p { class: "text-muted-foreground",
                        "Add icons to the left or right of the button label."
                    }
                    ComponentPreview {
                        source: ICONS_SOURCE.to_string(),
                        filename: Some("button_icons.rs".to_string()),
                        ButtonWithIconsExample {}
                    }
                }

                // Icon Buttons
                div { class: "space-y-4",
                    h3 { id: "icon-buttons", class: "text-xl font-medium", "Icon Buttons" }
                    p { class: "text-muted-foreground",
                        "Use is_icon_button for square buttons containing only an icon."
                    }
                    ComponentPreview {
                        source: ICON_BUTTONS_SOURCE.to_string(),
                        filename: Some("icon_buttons.rs".to_string()),
                        IconButtonsExample {}
                    }
                }

                // Full Width
                div { class: "space-y-4",
                    h3 { id: "full-width", class: "text-xl font-medium", "Full Width" }
                    p { class: "text-muted-foreground",
                        "Make a button span the full width of its container."
                    }
                    ComponentPreview {
                        source: FULL_WIDTH_SOURCE.to_string(),
                        filename: Some("button_full_width.rs".to_string()),
                        FullWidthButtonExample {}
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
                                td { class: "py-3 px-4 font-mono text-xs", "variant" }
                                td { class: "py-3 px-4 font-mono text-xs", "ButtonVariant" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Visual style variant" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "ButtonSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Medium" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size of the button" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "disabled" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Disables the button" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "loading" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Shows loading spinner" }
                            }
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "full_width" }
                                td { class: "py-3 px-4 font-mono text-xs", "bool" }
                                td { class: "py-3 px-4 font-mono text-xs", "false" }
                                td { class: "py-3 px-4 text-muted-foreground", "Expands to full width" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "on_click" }
                                td { class: "py-3 px-4 font-mono text-xs", "Callback<MouseEvent>" }
                                td { class: "py-3 px-4 font-mono text-xs", "-" }
                                td { class: "py-3 px-4 text-muted-foreground", "Click event handler" }
                            }
                        }
                    }
                }
            }
        }
    }
}
