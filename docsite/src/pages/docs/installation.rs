//! Installation documentation page.

use crate::components::docs::{CodeBlock, DocHeader};
use dioxus::prelude::*;
use dioxus_shadcn::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

/// Installation guide page.
#[component]
pub fn InstallationDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Installation",
                description: "How to install and configure dioxus-shadcn in your project.",
            }

            section { class: "space-y-4",
                h2 { id: "prerequisites", class: "text-2xl font-semibold tracking-tight", "Prerequisites" }
                ul { class: "list-disc list-inside space-y-2 text-muted-foreground",
                    li {
                        a {
                            href: "https://www.rust-lang.org/tools/install",
                            target: "_blank",
                            class: "font-medium text-foreground underline underline-offset-4 hover:no-underline",
                            "Rust"
                        }
                        " installed"
                    }
                    li {
                        a {
                            href: "https://dioxuslabs.com/learn/0.7/getting_started/#installing-the-cli",
                            target: "_blank",
                            class: "font-medium text-foreground underline underline-offset-4 hover:no-underline",
                            "Dioxus CLI"
                        }
                        " installed"
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }

                Tabs {
                    default_value: "cargo".to_string(),

                    TabsList {
                        TabsTrigger { value: "cargo".to_string(), "Cargo" }
                        TabsTrigger { value: "git".to_string(), "Git" }
                    }

                    TabsContent { value: "cargo".to_string(),
                        CodeBlock {
                            source: r#"cargo add dioxus-shadcn"#.to_string(),
                            language: "bash".to_string(),
                        }
                    }

                    TabsContent { value: "git".to_string(),
                        CodeBlock {
                            source: r#"# In Cargo.toml
[dependencies]
dioxus-shadcn = { git = "https://github.com/hxrts/dioxus-shadcn.git" }"#.to_string(),
                            language: "toml".to_string(),
                        }
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "tailwind", class: "text-2xl font-semibold tracking-tight", "Tailwind CSS Setup" }
                p { class: "text-muted-foreground leading-7",
                    "Create a tailwind.css file with the theme CSS variables:"
                }

                CodeBlock {
                    source: TAILWIND_CSS_EXAMPLE.to_string(),
                    filename: Some("tailwind.css".to_string()),
                    language: "css".to_string(),
                    collapsible: true,
                    default_collapsed: true,
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                p { class: "text-muted-foreground leading-7",
                    "Import and use components in your Dioxus application:"
                }

                CodeBlock {
                    source: USAGE_EXAMPLE.to_string(),
                    language: "rust".to_string(),
                }
            }
        }
    }
}

const TAILWIND_CSS_EXAMPLE: &str = r#"@import "tailwindcss";
@config "./tailwind-config.js";

body {
    background-color: var(--background);
    color: var(--foreground);
}

:root {
    --background: oklch(1 0 0);
    --foreground: oklch(0.145 0 0);
    --primary: oklch(0.205 0 0);
    --primary-foreground: oklch(0.985 0 0);
    --secondary: oklch(0.97 0 0);
    --secondary-foreground: oklch(0.205 0 0);
    --muted: oklch(0.97 0 0);
    --muted-foreground: oklch(0.556 0 0);
    --accent: oklch(0.97 0 0);
    --accent-foreground: oklch(0.205 0 0);
    --destructive: oklch(0.577 0.245 27.325);
    --border: oklch(0.922 0 0);
    --input: oklch(0.922 0 0);
    --ring: oklch(0.708 0 0);
    --radius: 0.625rem;
}

@media (prefers-color-scheme: dark) {
    :root {
        --background: oklch(0.145 0 0);
        --foreground: oklch(0.985 0 0);
        --primary: oklch(0.985 0 0);
        --primary-foreground: oklch(0.205 0 0);
        /* ... */
    }
}"#;

const USAGE_EXAMPLE: &str = r#"use dioxus::prelude::*;
use dioxus_shadcn::components::button::{Button, ButtonVariant};

#[component]
fn App() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Default,
            "Click me"
        }
    }
}"#;
