//! Home page component mirroring the shadcn-ui v4 landing structure.

use crate::components::{
    Announcement, ExamplesNav, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
    PageNav, ThemeSelector,
};
use dioxus::prelude::*;
use lumen_blocks::components::{
    badge::Badge,
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    input::Input,
    progress::Progress,
    separator::Separator,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
};

const TITLE: &str = "The Foundation for your Design System";
const DESCRIPTION: &str = "A set of beautifully designed components that you can customize, extend, and build on. Start here then make it your own. Open Source. Open Code.";

/// Home page component.
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col",
            PageHeader {
                Announcement {}
                PageHeaderHeading { class: "max-w-4xl", "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    a {
                        href: "/docs/installation",
                        class: "inline-flex h-[31px] items-center justify-center rounded-lg bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Get Started"
                    }
                    a {
                        href: "/docs/components",
                        class: "inline-flex h-8 items-center justify-center rounded-lg px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "View Components"
                    }
                }
            }

            PageNav { class: "hidden md:flex",
                ExamplesNav { class: "flex-1 overflow-hidden [&>a:first-child]:text-primary" }
                ThemeSelector { class: "mr-4 hidden md:flex" }
            }

            div { class: "container-wrapper flex-1 section-soft pb-6",
                div { class: "container overflow-hidden",
                    section { class: "-mx-4 w-[160vw] overflow-hidden rounded-lg border border-border/50 md:hidden md:w-[150vw]",
                        img {
                            src: "/r/styles/new-york-v4/dashboard-01-light.png",
                            alt: "Dashboard",
                            class: "block w-full dark:hidden",
                        }
                        img {
                            src: "/r/styles/new-york-v4/dashboard-01-dark.png",
                            alt: "Dashboard",
                            class: "hidden w-full dark:block",
                        }
                    }

                    section { class: "hidden theme-container md:block",
                        RootComponentsPreview {}
                    }
                }
            }
        }
    }
}

#[component]
fn RootComponentsPreview() -> Element {
    rsx! {
        div { class: "mx-auto max-w-6xl py-6",
            div { class: "grid gap-4 lg:grid-cols-[1.6fr_1fr]",
            Card {
                CardHeader {
                    CardTitle { "Project Overview" }
                    CardDescription { "Monitor conversion and release status." }
                }
                CardContent { class: "space-y-6",
                    div { class: "grid gap-4 sm:grid-cols-2",
                        div { class: "space-y-2",
                            p { class: "text-sm text-muted-foreground", "Revenue" }
                            p { class: "text-2xl font-semibold", "$82,430" }
                            Progress { value: 71 }
                        }
                        div { class: "space-y-2",
                            p { class: "text-sm text-muted-foreground", "Retention" }
                            p { class: "text-2xl font-semibold", "64%" }
                            Progress { value: 64 }
                        }
                    }

                    Separator {}

                    Tabs { default_value: "deployments",
                        TabsList { class: "grid w-full grid-cols-2",
                            TabsTrigger { value: "deployments", "Deployments" }
                            TabsTrigger { value: "activity", "Activity" }
                        }
                        TabsContent { value: "deployments", class: "space-y-3",
                            p { class: "text-sm text-muted-foreground", "Production rollout completes in 2 hours." }
                            Button { "Open Release Notes" }
                        }
                        TabsContent { value: "activity", class: "space-y-3",
                            p { class: "text-sm text-muted-foreground", "18 pull requests merged this week." }
                            Button { variant: ButtonVariant::Outline, "Review Changes" }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Quick Actions" }
                    CardDescription { "Compose and publish updates." }
                }
                CardContent { class: "space-y-4",
                    Input { placeholder: "Search components..." }
                    div { class: "flex flex-wrap gap-2",
                        Badge { "UI" }
                        Badge { "Accessibility" }
                        Badge { "Performance" }
                    }
                    Button { class: "w-full", "Create New Project" }
                    Button { variant: ButtonVariant::Ghost, class: "w-full", "Browse Examples" }
                }
            }
            }
        }
    }
}
