//! Documentation page header component.

use dioxus::prelude::*;
use lucide_dioxus::ExternalLink;

/// Props for the DocHeader component.
#[derive(Props, Clone, PartialEq)]
pub struct DocHeaderProps {
    /// The page title.
    pub title: &'static str,

    /// The page description.
    pub description: &'static str,

    /// Optional link to external documentation.
    #[props(default)]
    pub docs_url: Option<&'static str>,

    /// Optional link to API reference.
    #[props(default)]
    pub api_url: Option<&'static str>,
}

/// Header component for documentation pages.
///
/// Displays the title, description, and optional external links.
#[component]
pub fn DocHeader(props: DocHeaderProps) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            "data-slot": "doc-header",

            // Title
            h1 {
                class: "text-3xl font-bold tracking-tight text-foreground",
                "{props.title}"
            }

            // Description
            p {
                class: "text-lg text-muted-foreground",
                "{props.description}"
            }

            // External links
            if props.docs_url.is_some() || props.api_url.is_some() {
                div { class: "flex items-center gap-4 pt-2",
                    if let Some(docs_url) = props.docs_url {
                        a {
                            class: "inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground transition-colors",
                            href: "{docs_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Docs"
                            ExternalLink { class: "h-3 w-3" }
                        }
                    }
                    if let Some(api_url) = props.api_url {
                        a {
                            class: "inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground transition-colors",
                            href: "{api_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "API Reference"
                            ExternalLink { class: "h-3 w-3" }
                        }
                    }
                }
            }
        }
    }
}
