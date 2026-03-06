//! Tailwind-inspired color palette page.

use crate::components::{
    Announcement, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
};
use dioxus::prelude::*;

const TITLE: &str = "Tailwind Colors in Every Format";
const DESCRIPTION: &str =
    "The complete Tailwind color palette in HEX, RGB, HSL, CSS variables, and classes. Ready to copy and paste into your project.";

const PALETTES: [(&str, [(&str, &str); 6]); 6] = [
    (
        "Neutral",
        [
            ("50", "oklch(0.985 0 0)"),
            ("200", "oklch(0.922 0 0)"),
            ("400", "oklch(0.708 0 0)"),
            ("500", "oklch(0.556 0 0)"),
            ("700", "oklch(0.371 0 0)"),
            ("900", "oklch(0.205 0 0)"),
        ],
    ),
    (
        "Blue",
        [
            ("50", "oklch(0.97 0.02 250)"),
            ("200", "oklch(0.86 0.06 250)"),
            ("400", "oklch(0.72 0.13 250)"),
            ("500", "oklch(0.62 0.214 259.815)"),
            ("700", "oklch(0.5 0.2 258)"),
            ("900", "oklch(0.36 0.12 255)"),
        ],
    ),
    (
        "Green",
        [
            ("50", "oklch(0.982 0.018 155.826)"),
            ("200", "oklch(0.9 0.07 152)"),
            ("400", "oklch(0.792 0.209 151.711)"),
            ("500", "oklch(0.723 0.219 149.579)"),
            ("700", "oklch(0.527 0.154 150.069)"),
            ("900", "oklch(0.39 0.1 151)"),
        ],
    ),
    (
        "Amber",
        [
            ("50", "oklch(0.99 0.02 95)"),
            ("200", "oklch(0.92 0.07 93)"),
            ("400", "oklch(0.852 0.199 91.936)"),
            ("500", "oklch(0.795 0.184 86.047)"),
            ("700", "oklch(0.62 0.16 78)"),
            ("900", "oklch(0.46 0.12 73)"),
        ],
    ),
    (
        "Rose",
        [
            ("50", "oklch(0.98 0.02 355)"),
            ("200", "oklch(0.9 0.08 356)"),
            ("400", "oklch(0.76 0.15 354)"),
            ("500", "oklch(0.68 0.2 352)"),
            ("700", "oklch(0.52 0.17 350)"),
            ("900", "oklch(0.38 0.12 347)"),
        ],
    ),
    (
        "Purple",
        [
            ("50", "oklch(0.98 0.02 300)"),
            ("200", "oklch(0.9 0.07 300)"),
            ("400", "oklch(0.74 0.18 302)"),
            ("500", "oklch(0.627 0.265 303.9)"),
            ("700", "oklch(0.49 0.23 304)"),
            ("900", "oklch(0.36 0.14 304)"),
        ],
    ),
];

/// Colors reference page.
#[component]
pub fn Colors() -> Element {
    rsx! {
        div {
            PageHeader {
                Announcement {}
                PageHeaderHeading { "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    a {
                        href: "#colors",
                        class: "inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Browse Colors"
                    }
                    a {
                        href: "/docs/theming",
                        class: "inline-flex h-8 items-center justify-center rounded-md px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "Documentation"
                    }
                }
            }

            div { class: "hidden",
                div { class: "container-wrapper",
                    div { class: "container flex items-center justify-between gap-8 py-4" }
                }
            }

            div { class: "container-wrapper",
                div { class: "container py-6",
                    section { id: "colors", class: "scroll-mt-20",
                        div { class: "grid gap-8 lg:gap-16 xl:gap-20",
                            for (name, shades) in PALETTES {
                                article { class: "space-y-3",
                                    h2 { class: "text-xl font-semibold", "{name}" }
                                    div { class: "grid gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6",
                                        for (scale, color) in shades {
                                            div { class: "rounded-lg border border-border bg-card p-3",
                                                div {
                                                    class: "h-16 rounded-md border border-border/40",
                                                    style: "background: {color};",
                                                }
                                                div { class: "mt-2 space-y-1",
                                                    p { class: "text-sm font-medium", "{name} {scale}" }
                                                    p { class: "text-xs font-mono text-muted-foreground", "{color}" }
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
        }
    }
}
