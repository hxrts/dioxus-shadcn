//! Site footer component - matches shadcn-ui v4 site-footer style.

use dioxus::prelude::*;

/// Site footer component - matches shadcn/ui v4 exactly.
#[component]
pub fn SiteFooter() -> Element {
    // Reference: group-has-[.section-soft]/body:bg-surface/40 dark:bg-transparent
    rsx! {
        footer {
            class: "dark:bg-transparent",

            div {
                class: "container-wrapper px-4 xl:px-6",

                div {
                    class: "flex h-14 items-center justify-between",

                    div {
                        class: "w-full px-1 text-center text-xs leading-loose text-muted-foreground sm:text-sm",

                        "Built with "
                        a {
                            href: "https://dioxuslabs.com",
                            target: "_blank",
                            rel: "noreferrer",
                            class: "font-medium underline underline-offset-4",
                            "Dioxus"
                        }
                        " and "
                        a {
                            href: "https://ui.shadcn.com",
                            target: "_blank",
                            rel: "noreferrer",
                            class: "font-medium underline underline-offset-4",
                            "shadcn/ui"
                        }
                        ". Source code on "
                        a {
                            href: "https://github.com/hxrts/dioxus-shadcn",
                            target: "_blank",
                            rel: "noreferrer",
                            class: "font-medium underline underline-offset-4",
                            "GitHub"
                        }
                        "."
                    }
                }
            }
        }
    }
}
