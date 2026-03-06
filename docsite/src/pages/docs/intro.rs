//! Introduction documentation page.

use crate::components::docs::DocHeader;
use dioxus::prelude::*;

/// Introduction page for the documentation.
#[component]
pub fn IntroDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Introduction",
                description: "A high-fidelity port of shadcn UI to Dioxus.",
            }

            section { class: "space-y-4",
                h2 { id: "about", class: "text-2xl font-semibold tracking-tight", "About" }
                p { class: "text-muted-foreground leading-7",
                    "dioxus-shadcn provides accessible, styled components for building "
                    "modern web applications with Dioxus. Built on top of "
                    a {
                        href: "https://github.com/DioxusLabs/components",
                        target: "_blank",
                        class: "font-medium text-foreground underline underline-offset-4 hover:no-underline",
                        "Dioxus Primitives"
                    }
                    " and styled with Tailwind CSS v4."
                }
            }

            section { class: "space-y-4",
                h2 { id: "features", class: "text-2xl font-semibold tracking-tight", "Features" }
                ul { class: "list-disc list-inside space-y-2 text-muted-foreground",
                    li { "40+ components with shadcn-ui patterns" }
                    li { "OKLCH color theming with multiple presets" }
                    li { "Full ARIA accessibility via Dioxus Primitives" }
                    li { "Tailwind CSS v4 styling" }
                    li { "Type-safe Rust components" }
                }
            }

            section { class: "space-y-4",
                h2 { id: "philosophy", class: "text-2xl font-semibold tracking-tight", "Philosophy" }
                p { class: "text-muted-foreground leading-7",
                    "This library follows shadcn-ui's approach: components are designed to be "
                    "copied into your project and customized. The styling uses semantic CSS "
                    "custom properties (--primary, --background, etc.) for easy theming."
                }
            }

            section { class: "space-y-4",
                h2 { id: "getting-started", class: "text-2xl font-semibold tracking-tight", "Getting Started" }
                p { class: "text-muted-foreground leading-7",
                    "Check out the "
                    Link {
                        to: "/docs/installation",
                        class: "font-medium text-foreground underline underline-offset-4 hover:no-underline",
                        "Installation"
                    }
                    " guide to add dioxus-shadcn to your project, then explore the "
                    "component documentation in the sidebar."
                }
            }
        }
    }
}
