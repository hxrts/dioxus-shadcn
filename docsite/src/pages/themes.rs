//! Themes page with theme picker and preview.

use dioxus::prelude::*;
use lumen_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    input::Input,
    label::Label,
};

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
        div { class: "flex flex-1 flex-col",
            // Page header
            div {
                class: "border-b border-border/40",
                div {
                    class: "container max-w-screen-2xl",
                    div {
                        class: "flex flex-col items-center gap-4 py-12 md:py-16 text-center px-4",
                        h1 {
                            class: "text-3xl font-bold leading-tight tracking-tighter md:text-4xl",
                            "Themes"
                        }
                        p {
                            class: "max-w-2xl text-lg text-muted-foreground",
                            "Explore different color themes. All themes use OKLCH color space for perceptual uniformity."
                        }
                    }
                }
            }

            // Theme content
            div { class: "container max-w-screen-2xl px-4 md:px-6 py-12",
                div { class: "grid gap-8 lg:grid-cols-[1fr_400px]",
                    // Theme picker grid
                    div {
                        h2 { class: "text-xl font-semibold mb-4", "Choose a Theme" }
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

                    // Theme preview
                    div {
                        h2 { class: "text-xl font-semibold mb-4", "Preview" }
                        ThemePreview { theme: THEMES[*selected_theme.read()].clone() }
                    }
                }

                // CSS output section
                div { class: "mt-12",
                    h2 { class: "text-xl font-semibold mb-4", "CSS Variables" }
                    ThemeCSSOutput { theme: THEMES[*selected_theme.read()].clone() }
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

            // Color preview circles
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

            // Theme info
            div {
                p { class: "font-medium text-sm", "{theme.name}" }
                p { class: "text-xs text-muted-foreground line-clamp-2", "{theme.description}" }
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

            // Button variants preview
            div { class: "mt-6 pt-6 border-t border-border",
                p { class: "text-sm font-medium mb-3", "Button Variants" }
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

/// CSS output for the selected theme.
#[component]
fn ThemeCSSOutput(theme: Theme) -> Element {
    let css_code = format!(
        r#":root {{
  --primary: {};
  --primary-foreground: {};
  --accent: {};
}}"#,
        theme.primary, theme.primary_foreground, theme.accent
    );

    rsx! {
        div { class: "rounded-lg border border-border bg-muted/50 p-4",
            pre { class: "text-sm font-mono overflow-x-auto",
                code { "{css_code}" }
            }
        }
        p { class: "mt-2 text-sm text-muted-foreground",
            "Copy these CSS variables to your tailwind.css or globals.css file."
        }
    }
}
