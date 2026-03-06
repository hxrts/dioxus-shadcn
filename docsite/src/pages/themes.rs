//! Themes page with theme picker and preview.

use crate::components::{
    Announcement, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading, ThemeSelector,
};
use dioxus::prelude::*;
use lumen_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    input::Input,
    label::Label,
};

const TITLE: &str = "Pick a Color. Make it yours.";
const DESCRIPTION: &str =
    "Try our hand-picked themes. Copy and paste them into your project. New theme editor coming soon.";

/// Theme configuration.
#[derive(Clone, PartialEq)]
struct Theme {
    name: &'static str,
    description: &'static str,
    primary: &'static str,
    primary_foreground: &'static str,
    accent: &'static str,
}

/// Available themes.
const THEMES: &[Theme] = &[
    Theme {
        name: "Zinc",
        description: "Clean and minimal with neutral tones.",
        primary: "oklch(0.205 0 0)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.97 0 0)",
    },
    Theme {
        name: "Slate",
        description: "Cool gray tones with blue undertones.",
        primary: "oklch(0.205 0.015 255)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.97 0.005 255)",
    },
    Theme {
        name: "Rose",
        description: "Warm and inviting with pink accents.",
        primary: "oklch(0.55 0.2 350)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.95 0.03 350)",
    },
    Theme {
        name: "Blue",
        description: "Classic blue theme for a professional look.",
        primary: "oklch(0.55 0.2 250)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.95 0.03 250)",
    },
    Theme {
        name: "Green",
        description: "Fresh and natural with green tones.",
        primary: "oklch(0.55 0.18 145)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.95 0.03 145)",
    },
    Theme {
        name: "Orange",
        description: "Energetic and bold with warm orange.",
        primary: "oklch(0.65 0.2 45)",
        primary_foreground: "oklch(0.15 0 0)",
        accent: "oklch(0.95 0.03 45)",
    },
    Theme {
        name: "Violet",
        description: "Creative and modern with purple hues.",
        primary: "oklch(0.55 0.2 290)",
        primary_foreground: "oklch(0.985 0 0)",
        accent: "oklch(0.95 0.03 290)",
    },
    Theme {
        name: "Yellow",
        description: "Bright and cheerful with sunny yellow.",
        primary: "oklch(0.75 0.18 85)",
        primary_foreground: "oklch(0.15 0 0)",
        accent: "oklch(0.95 0.05 85)",
    },
];

/// Themes page component.
#[component]
pub fn Themes() -> Element {
    let mut selected_theme = use_signal(|| 0usize);

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
                    a {
                        href: "/docs/theming",
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
                    div { class: "grid gap-8 lg:grid-cols-[1fr_400px]",
                        div {
                            h2 { class: "mb-4 text-xl font-semibold", "Choose a Theme" }
                            div { class: "grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4",
                                for (i, theme) in THEMES.iter().enumerate() {
                                    ThemeCard {
                                        theme: theme.clone(),
                                        selected: *selected_theme.read() == i,
                                        on_select: move |_| selected_theme.set(i),
                                    }
                                }
                            }
                        }

                        div {
                            h2 { class: "mb-4 text-xl font-semibold", "Preview" }
                            ThemePreview { theme: THEMES[*selected_theme.read()].clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Theme selection card.
#[component]
fn ThemeCard(theme: Theme, selected: bool, on_select: EventHandler<()>) -> Element {
    let border_class = if selected {
        "border-2 border-primary"
    } else {
        "border border-border hover:border-muted-foreground/50"
    };

    rsx! {
        button {
            class: "flex flex-col items-start gap-2 rounded-lg p-4 text-left transition-colors {border_class}",
            onclick: move |_| on_select.call(()),

            div { class: "flex gap-2",
                div {
                    class: "h-6 w-6 rounded-full border border-border",
                    style: "background-color: {theme.primary}",
                }
                div {
                    class: "h-6 w-6 rounded-full border border-border",
                    style: "background-color: {theme.accent}",
                }
            }

            div {
                p { class: "text-sm font-medium", "{theme.name}" }
                p { class: "line-clamp-2 text-xs text-muted-foreground", "{theme.description}" }
            }
        }
    }
}

/// Theme preview component showing components with the selected theme.
#[component]
fn ThemePreview(theme: Theme) -> Element {
    rsx! {
        div {
            class: "rounded-lg border border-border p-6",
            style: "--preview-primary: {theme.primary}; --preview-primary-foreground: {theme.primary_foreground}; --preview-accent: {theme.accent};",

            Card {
                CardHeader {
                    CardTitle { "Create Account" }
                    CardDescription { "Enter your details to get started." }
                }
                CardContent {
                    div { class: "grid gap-4",
                        div { class: "grid gap-2",
                            Label { for_id: "name", "Name" }
                            Input { id: "name", placeholder: "John Doe" }
                        }
                        div { class: "grid gap-2",
                            Label { for_id: "email", "Email" }
                            Input { id: "email", r#type: "email", placeholder: "john@example.com" }
                        }
                        div { class: "flex gap-2 pt-2",
                            Button {
                                variant: ButtonVariant::Default,
                                class: "flex-1",
                                "Create Account"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                "Cancel"
                            }
                        }
                    }
                }
            }

            div { class: "mt-6 border-t border-border pt-6",
                p { class: "mb-3 text-sm font-medium", "Button Variants" }
                div { class: "flex flex-wrap gap-2",
                    Button { variant: ButtonVariant::Default, size: ButtonSize::Small, "Default" }
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Small, "Secondary" }
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Small, "Outline" }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Small, "Ghost" }
                    Button { variant: ButtonVariant::Destructive, size: ButtonSize::Small, "Destructive" }
                }
            }
        }
    }
}
