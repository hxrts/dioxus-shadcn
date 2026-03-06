//! Syntax-highlighted code block with copy functionality.

use dioxus::prelude::*;
use lucide_dioxus::{Check, Copy};
use dioxus_shadcn::components::button::{Button, ButtonSize, ButtonVariant};

/// Props for the CodeBlock component.
#[derive(Props, Clone, PartialEq)]
pub struct CodeBlockProps {
    /// The source code to display.
    pub source: String,

    /// Optional filename to display in the header.
    #[props(default)]
    pub filename: Option<String>,

    /// Programming language for syntax highlighting hints.
    #[props(default = "rust".to_string())]
    pub language: String,

    /// Whether the code block can be collapsed.
    #[props(default = false)]
    pub collapsible: bool,

    /// Initial collapsed state (only used if collapsible is true).
    #[props(default = false)]
    pub default_collapsed: bool,

    /// Maximum height before scrolling.
    #[props(default)]
    pub max_height: Option<String>,
}

/// A code block with syntax highlighting and copy-to-clipboard functionality.
#[component]
pub fn CodeBlock(props: CodeBlockProps) -> Element {
    let mut copied = use_signal(|| false);
    let mut collapsed = use_signal(|| props.collapsible && props.default_collapsed);

    let source_for_clipboard = props.source.clone();
    let max_height = props
        .max_height
        .clone()
        .unwrap_or_else(|| "400px".to_string());

    rsx! {
        div {
            class: "rounded-lg border border-border overflow-hidden",
            "data-slot": "code-block",

            // Header with filename and actions
            div {
                class: "bg-muted/50 flex items-center justify-between px-4 py-2 border-b border-border",

                // Filename or language indicator
                div { class: "flex items-center gap-2",
                    if let Some(filename) = &props.filename {
                        span { class: "text-xs font-mono text-muted-foreground", "{filename}" }
                    } else {
                        span { class: "text-xs font-mono text-muted-foreground", "{props.language}" }
                    }
                }

                // Actions
                div { class: "flex items-center gap-1",
                    // Collapse toggle (if collapsible)
                    if props.collapsible {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            class: "h-7 w-7 p-0",
                            on_click: move |_| collapsed.toggle(),
                            if collapsed() {
                                span { class: "text-xs", "Show" }
                            } else {
                                span { class: "text-xs", "Hide" }
                            }
                        }
                    }

                    // Copy button
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Small,
                        class: "h-7 w-7 p-0",
                        on_click: move |_| copied.set(true),
                        "onclick": format!("navigator.clipboard.writeText(`{}`)", source_for_clipboard.replace('`', "\\`").replace("${", "\\${")),
                        if copied() {
                            Check { class: "h-3.5 w-3.5 text-green-500" }
                        } else {
                            Copy { class: "h-3.5 w-3.5" }
                        }
                    }
                }
            }

            // Code content
            if !collapsed() {
                div {
                    class: "overflow-auto text-xs",
                    style: "max-height: {max_height}",

                    pre {
                        class: "p-4 bg-[#0d0d0d] m-0",

                        code {
                            class: "font-mono text-zinc-300 whitespace-pre",
                            "{props.source}"
                        }
                    }
                }
            }
        }
    }
}
