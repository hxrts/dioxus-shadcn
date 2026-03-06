//! Component preview with live demo and code tabs.

use super::CodeBlock;
use dioxus::prelude::*;
use dioxus_shadcn::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

/// Props for the ComponentPreview component.
#[derive(Props, Clone, PartialEq)]
pub struct ComponentPreviewProps {
    /// The source code to display in the Code tab.
    pub source: String,

    /// Optional filename to display.
    #[props(default)]
    pub filename: Option<String>,

    /// Optional title for the preview section.
    #[props(default)]
    pub title: Option<String>,

    /// Optional description text.
    #[props(default)]
    pub description: Option<String>,

    /// Additional CSS classes for the preview container.
    #[props(default)]
    pub preview_class: Option<String>,

    /// The live component to render in the Preview tab.
    pub children: Element,
}

/// A component preview with live demo and tabbed code display.
///
/// Mirrors shadcn-ui's ComponentPreview pattern with Preview/Code tabs.
#[component]
pub fn ComponentPreview(props: ComponentPreviewProps) -> Element {
    let preview_class = props.preview_class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "rounded-lg border border-border overflow-hidden my-6",
            "data-slot": "component-preview",

            // Optional header with title/description
            if props.title.is_some() || props.description.is_some() {
                div { class: "px-4 py-3 border-b border-border bg-muted/30",
                    if let Some(title) = &props.title {
                        h4 { class: "text-sm font-medium text-foreground", "{title}" }
                    }
                    if let Some(description) = &props.description {
                        p { class: "text-xs text-muted-foreground mt-1", "{description}" }
                    }
                }
            }

            Tabs {
                default_value: "preview".to_string(),

                TabsList {
                    class: "w-full justify-start rounded-none border-b border-border bg-transparent p-0 h-10",

                    TabsTrigger {
                        value: "preview".to_string(),
                        class: "relative rounded-none border-b-2 border-transparent bg-transparent px-4 py-2 font-medium text-muted-foreground shadow-none transition-none data-[state=active]:border-primary data-[state=active]:text-foreground data-[state=active]:shadow-none",
                        "Preview"
                    }
                    TabsTrigger {
                        value: "code".to_string(),
                        class: "relative rounded-none border-b-2 border-transparent bg-transparent px-4 py-2 font-medium text-muted-foreground shadow-none transition-none data-[state=active]:border-primary data-[state=active]:text-foreground data-[state=active]:shadow-none",
                        "Code"
                    }
                }

                TabsContent {
                    value: "preview".to_string(),
                    class: "mt-0 border-0",

                    div {
                        class: "flex min-h-[200px] w-full items-center justify-center p-10 {preview_class}",
                        {props.children}
                    }
                }

                TabsContent {
                    value: "code".to_string(),
                    class: "mt-0 border-0",

                    CodeBlock {
                        source: props.source.clone(),
                        filename: props.filename.clone(),
                        language: "rust".to_string(),
                        max_height: Some("350px".to_string()),
                    }
                }
            }
        }
    }
}
