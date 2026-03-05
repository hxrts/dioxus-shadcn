//! Home page component.

use crate::components::FeatureCard;
use crate::examples::button::ButtonVariantsExample;
use crate::pages::docs::DocsRoute;
use crate::LOGO;
use dioxus::prelude::*;
use lucide_dioxus::{Check, Palette, PersonStanding, Wind};
use lumen_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    toast::ToastProvider,
};

/// Home page component.
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background relative",
            div { class: "max-w-6xl mx-auto px-6 py-12",

                // Hero section
                div { class: "text-center mb-12",
                    img {
                        class: "w-48 h-48 mx-auto mb-4 dark:invert",
                        src: LOGO,
                        alt: "dioxus-shadcn"
                    }
                    h1 { class: "text-4xl font-bold text-foreground mb-4", "dioxus-shadcn" }
                    p { class: "text-xl text-muted-foreground mb-4",
                        "shadcn UI components for Dioxus"
                    }

                    div {
                        class: "inline-block px-3 py-1 mb-8 text-xs font-medium rounded-full bg-primary/10 text-primary border border-primary/20",
                        "v0.3.0"
                    }

                    div { class: "flex justify-center gap-4",
                        a { href: "/docs",
                            Button {
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Large,
                                "View Docs"
                            }
                        }
                        a {
                            href: "https://github.com/hxrts/dioxus-shadcn",
                            target: "_blank",
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Large,
                                "GitHub"
                            }
                        }
                    }
                }

                // Feature Cards
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-6 mb-24",
                    FeatureCard {
                        title: "40+ Components".to_string(),
                        description: "Comprehensive UI components built for modern web apps".to_string(),
                        icon: rsx! { Check { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Tailwind CSS v4".to_string(),
                        description: "Styled with Tailwind and dark mode support".to_string(),
                        icon: rsx! { Wind { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Accessible".to_string(),
                        description: "Built with ARIA best practices via Dioxus Primitives".to_string(),
                        icon: rsx! { PersonStanding { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "OKLCH Theming".to_string(),
                        description: "Perceptually uniform color themes with multiple presets".to_string(),
                        icon: rsx! { Palette { class: "w-8 h-8 text-primary" } }
                    }
                }

                // Component Preview
                ToastProvider {
                    div { class: "rounded-lg",
                        h2 { class: "text-3xl font-semibold text-foreground mb-6", "Component Preview" }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                            // Button preview
                            HomeComponentCard {
                                title: "Button",
                                href: DocsRoute::ButtonPage.to_path(),
                                col_span: "md:col-span-2",
                                ButtonVariantsExample {}
                            }

                            // Placeholder cards for other components
                            HomeComponentCard {
                                title: "Dialog",
                                href: DocsRoute::DialogPage.to_path(),
                                div { class: "text-muted-foreground text-sm",
                                    "Modal dialogs and alerts"
                                }
                            }

                            HomeComponentCard {
                                title: "Tabs",
                                href: DocsRoute::TabsPage.to_path(),
                                div { class: "text-muted-foreground text-sm",
                                    "Tabbed content panels"
                                }
                            }

                            HomeComponentCard {
                                title: "Dropdown",
                                href: DocsRoute::DropdownPage.to_path(),
                                div { class: "text-muted-foreground text-sm",
                                    "Dropdown menus and selects"
                                }
                            }

                            HomeComponentCard {
                                title: "Toast",
                                href: DocsRoute::ToastPage.to_path(),
                                div { class: "text-muted-foreground text-sm",
                                    "Toast notifications"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Simple component card for the home page.
#[component]
fn HomeComponentCard(
    title: &'static str,
    href: &'static str,
    #[props(default)] col_span: Option<&'static str>,
    children: Element,
) -> Element {
    let grid_class = col_span.unwrap_or("");

    rsx! {
        div {
            class: "bg-background rounded-lg border border-border p-6 transition-all hover:shadow-md {grid_class}",
            div {
                class: "flex justify-between items-center mb-4",
                h3 { class: "font-medium text-foreground", "{title}" }
                a {
                    href: "{href}",
                    class: "text-sm text-primary hover:underline",
                    "View Docs"
                }
            }
            div {
                class: "flex justify-stretch py-2",
                {children}
            }
        }
    }
}
