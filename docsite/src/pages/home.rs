//! Home page component mirroring the shadcn-ui v4 landing structure.

use crate::components::{
    Announcement, ExamplesNav, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
    PageNav, ThemeSelector,
};
use dioxus::prelude::*;
use dioxus_shadcn::components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    input::Input,
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
                    Link {
                        to: "/docs/installation",
                        class: "inline-flex h-[31px] items-center justify-center rounded-lg bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Get Started"
                    }
                    Link {
                        to: "/docs/components",
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
        div { class: "mx-auto grid gap-8 py-1 theme-container md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 xl:gap-6 2xl:gap-8",
            div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                PreviewCard {
                    title: "Account Details",
                    description: "Configure your public profile.",
                    div { class: "grid gap-3",
                        Input { placeholder: "Display name" }
                        Input { placeholder: "Email address" }
                        Button { class: "w-full", "Save changes" }
                    }
                }
            }

            div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                PreviewCard {
                    title: "Quick Actions",
                    description: "Compose and publish updates.",
                    div { class: "space-y-4",
                        div { class: "flex flex-wrap gap-2",
                            Badge { "UI" }
                            Badge { "Accessibility" }
                            Badge { "Performance" }
                        }
                        Button { class: "w-full", "Create New Project" }
                        Button { variant: ButtonVariant::Ghost, class: "w-full", "Browse Examples" }
                    }
                }

                PreviewCard {
                    title: "Status",
                    description: "Current system health.",
                    div { class: "grid gap-2 text-sm",
                        div { class: "flex items-center justify-between",
                            span { class: "text-muted-foreground", "API" }
                            Badge { "Operational" }
                        }
                        div { class: "flex items-center justify-between",
                            span { class: "text-muted-foreground", "Database" }
                            Badge { variant: BadgeVariant::Secondary, "Healthy" }
                        }
                        div { class: "flex items-center justify-between",
                            span { class: "text-muted-foreground", "Jobs" }
                            Badge { variant: BadgeVariant::Outline, "3 queued" }
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                PreviewCard {
                    title: "Release Notes",
                    description: "Monitor conversion and release status.",
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
        }
    }
}

#[component]
fn PreviewCard(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "{title}" }
                CardDescription { "{description}" }
            }
            CardContent { class: "space-y-4",
                {children}
                Separator {}
                p { class: "text-xs text-muted-foreground", "Preview component content" }
            }
        }
    }
}
