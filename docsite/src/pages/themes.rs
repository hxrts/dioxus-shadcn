//! Themes page with selector row and cards showcase.

use crate::components::{
    Announcement, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading, ThemeSelector,
};
use dioxus::prelude::*;
use dioxus_shadcn::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

const TITLE: &str = "Pick a Color. Make it yours.";
const DESCRIPTION: &str =
    "Try our hand-picked themes. Copy and paste them into your project. New theme editor coming soon.";

/// Themes page component.
#[component]
pub fn Themes() -> Element {
    rsx! {
        div {
            PageHeader {
                Announcement {}
                PageHeaderHeading { "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    a {
                        href: "#themes",
                        class: "inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Browse Themes"
                    }
                    Link {
                        to: "/docs/theming",
                        class: "inline-flex h-8 items-center justify-center rounded-md px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "Documentation"
                    }
                }
            }

            div { id: "themes", class: "container-wrapper scroll-mt-20",
                div { class: "container flex items-center justify-between gap-8 px-6 py-4 md:px-8",
                    ThemeSelector {}
                }
            }

            div { class: "container-wrapper flex flex-1 flex-col section-soft pb-6",
                div { class: "container flex flex-1 flex-col theme-container",
                    ThemeCardsDemo {}
                }
            }
        }
    }
}

#[component]
fn ThemeCardsDemo() -> Element {
    rsx! {
        div { class: "md:grids-col-2 grid **:data-[slot=card]:shadow-none md:gap-4 lg:grid-cols-10 xl:grid-cols-11",
            div { class: "grid gap-4 lg:col-span-4 xl:col-span-6",
                ThemeDemoCard {
                    title: "Stats",
                    description: "Revenue and active users overview.",
                    body_class: "h-32 bg-muted/20 rounded-md"
                }

                div { class: "grid gap-4 md:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2",
                    div { class: "flex flex-col gap-4",
                        ThemeDemoCard {
                            title: "Forms",
                            description: "Form layout preview.",
                            body_class: "h-40 bg-muted/20 rounded-md"
                        }
                        ThemeDemoCard {
                            title: "Team Members",
                            description: "People list preview.",
                            body_class: "h-32 bg-muted/20 rounded-md"
                        }
                        ThemeDemoCard {
                            title: "Cookie Settings",
                            description: "Preference toggles preview.",
                            body_class: "h-28 bg-muted/20 rounded-md"
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        ThemeDemoCard {
                            title: "Create Account",
                            description: "Authentication card preview.",
                            body_class: "h-40 bg-muted/20 rounded-md"
                        }
                        ThemeDemoCard {
                            title: "Chat",
                            description: "Conversation card preview.",
                            body_class: "h-32 bg-muted/20 rounded-md"
                        }
                        ThemeDemoCard {
                            title: "Report Issue",
                            description: "Issue report panel preview.",
                            body_class: "h-28 bg-muted/20 rounded-md"
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-4 lg:col-span-6 xl:col-span-5",
                ThemeDemoCard {
                    title: "Calendar",
                    description: "Calendar component is not ported yet.",
                    body_class: "h-52 rounded-md border border-dashed"
                }
                ThemeDemoCard {
                    title: "Payments",
                    description: "Recent transactions and methods.",
                    body_class: "h-48 bg-muted/20 rounded-md"
                }
                ThemeDemoCard {
                    title: "Share",
                    description: "Share dialog preview.",
                    body_class: "h-40 bg-muted/20 rounded-md"
                }
            }
        }
    }
}

#[component]
fn ThemeDemoCard(
    title: &'static str,
    description: &'static str,
    body_class: &'static str,
) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { class: "text-base", "{title}" }
                CardDescription { "{description}" }
            }
            CardContent {
                div { class: "{body_class}" }
            }
        }
    }
}
