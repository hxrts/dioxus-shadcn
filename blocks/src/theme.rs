//! OKLCH-based Theming System for Lumen Blocks
//!
//! This module provides a comprehensive theming system compatible with shadcn-ui themes.
//! It uses the OKLCH color space for perceptually uniform colors and supports
//! light/dark modes with smooth transitions.
//!
//! # Example
//!
//! ```rust
//! use lumen_blocks::theme::{ThemeProvider, use_theme, themes};
//!
//! fn App() -> Element {
//!     rsx! {
//!         ThemeProvider {
//!             theme: themes::neutral(),
//!
//!             // Your app content
//!             MyComponent {}
//!         }
//!     }
//! }
//!
//! fn MyComponent() -> Element {
//!     let theme = use_theme();
//!
//!     rsx! {
//!         button {
//!             onclick: move |_| theme.toggle_color_scheme(),
//!             "Toggle Dark Mode"
//!         }
//!     }
//! }
//! ```

use dioxus::document::eval;
use dioxus::prelude::*;

/// OKLCH color representation.
///
/// OKLCH is a perceptually uniform color space that makes it easier to create
/// harmonious color palettes with predictable lightness relationships.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OklchColor {
    /// Lightness (0.0 to 1.0)
    pub l: f64,
    /// Chroma (0.0 to ~0.4, though practical values rarely exceed 0.3)
    pub c: f64,
    /// Hue angle (0.0 to 360.0)
    pub h: f64,
}

impl OklchColor {
    /// Create a new OKLCH color.
    pub const fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }

    /// Create a neutral (gray) color with no chroma.
    pub const fn neutral(l: f64) -> Self {
        Self { l, c: 0.0, h: 0.0 }
    }

    /// Convert to CSS oklch() function string.
    pub fn to_css(&self) -> String {
        format!("oklch({} {} {})", self.l, self.c, self.h)
    }
}

impl std::fmt::Display for OklchColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

/// Complete set of theme colors.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeColors {
    pub background: OklchColor,
    pub foreground: OklchColor,
    pub card: OklchColor,
    pub card_foreground: OklchColor,
    pub popover: OklchColor,
    pub popover_foreground: OklchColor,
    pub primary: OklchColor,
    pub primary_foreground: OklchColor,
    pub secondary: OklchColor,
    pub secondary_foreground: OklchColor,
    pub muted: OklchColor,
    pub muted_foreground: OklchColor,
    pub accent: OklchColor,
    pub accent_foreground: OklchColor,
    pub destructive: OklchColor,
    pub destructive_foreground: OklchColor,
    pub border: OklchColor,
    pub input: OklchColor,
    pub ring: OklchColor,
    // Chart colors for data visualization
    pub chart_1: OklchColor,
    pub chart_2: OklchColor,
    pub chart_3: OklchColor,
    pub chart_4: OklchColor,
    pub chart_5: OklchColor,
    // Sidebar colors
    pub sidebar: OklchColor,
    pub sidebar_foreground: OklchColor,
    pub sidebar_primary: OklchColor,
    pub sidebar_primary_foreground: OklchColor,
    pub sidebar_accent: OklchColor,
    pub sidebar_accent_foreground: OklchColor,
    pub sidebar_border: OklchColor,
    pub sidebar_ring: OklchColor,
}

/// A complete theme definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Theme identifier
    pub name: String,
    /// Light mode colors
    pub light: ThemeColors,
    /// Dark mode colors
    pub dark: ThemeColors,
    /// Border radius scale (in rem)
    pub radius: f64,
}

impl Theme {
    /// Get colors for the specified color scheme.
    pub fn colors(&self, scheme: ColorScheme) -> &ThemeColors {
        match scheme {
            ColorScheme::Light => &self.light,
            ColorScheme::Dark => &self.dark,
            ColorScheme::System => &self.light, // Default to light, actual handled by CSS
        }
    }
}

/// Color scheme preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorScheme {
    Light,
    Dark,
    #[default]
    System,
}

impl ColorScheme {
    /// Get the CSS class for this color scheme.
    pub fn class(&self) -> &'static str {
        match self {
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
            ColorScheme::System => "",
        }
    }
}

/// Theme context provided to the component tree.
#[derive(Clone)]
pub struct ThemeContext {
    /// The current theme
    pub theme: Signal<Theme>,
    /// The current color scheme preference
    pub color_scheme: Signal<ColorScheme>,
    /// The resolved color scheme (Light or Dark, never System)
    resolved_scheme: Signal<ColorScheme>,
}

impl ThemeContext {
    /// Get the current theme.
    pub fn current(&self) -> Theme {
        self.theme.read().clone()
    }

    /// Get the current color scheme preference.
    pub fn color_scheme(&self) -> ColorScheme {
        *self.color_scheme.read()
    }

    /// Get the resolved color scheme (accounts for System preference).
    pub fn resolved_scheme(&self) -> ColorScheme {
        *self.resolved_scheme.read()
    }

    /// Set the color scheme.
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme.set(scheme);
    }

    /// Toggle between light and dark modes.
    pub fn toggle_color_scheme(&mut self) {
        let current = self.resolved_scheme();
        let new_scheme = match current {
            ColorScheme::Light => ColorScheme::Dark,
            ColorScheme::Dark => ColorScheme::Light,
            ColorScheme::System => ColorScheme::Dark,
        };
        self.color_scheme.set(new_scheme);
    }

    /// Set a new theme.
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme.set(theme);
    }

    /// Get the current colors based on resolved scheme.
    pub fn colors(&self) -> ThemeColors {
        self.theme.read().colors(self.resolved_scheme()).clone()
    }
}

/// Props for the ThemeProvider component.
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    /// The theme to use
    pub theme: Theme,

    /// Initial color scheme preference
    #[props(default)]
    pub color_scheme: ColorScheme,

    /// Children to render
    pub children: Element,
}

/// Theme provider component that injects theme context and CSS variables.
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme = use_signal(|| props.theme.clone());
    let color_scheme = use_signal(|| props.color_scheme);
    let mut resolved_scheme = use_signal(|| {
        match props.color_scheme {
            ColorScheme::System => ColorScheme::Light, // Will be updated by effect
            other => other,
        }
    });

    // Detect system preference
    use_effect(move || {
        if color_scheme() == ColorScheme::System {
            // Use JavaScript to detect system preference
            spawn(async move {
                let detect_script = r#"
                    (function() {
                        return window.matchMedia('(prefers-color-scheme: dark)').matches;
                    })()
                "#;
                if let Ok(result) = eval(detect_script).await {
                    if let Ok(is_dark) = result.to_string().parse::<bool>() {
                        resolved_scheme.set(if is_dark {
                            ColorScheme::Dark
                        } else {
                            ColorScheme::Light
                        });
                    }
                }
            });
        } else {
            resolved_scheme.set(color_scheme());
        }
    });

    let context = ThemeContext {
        theme,
        color_scheme,
        resolved_scheme,
    };

    use_context_provider(|| context.clone());

    // Generate CSS variables
    let css_vars = use_memo(move || {
        let t = theme();
        let scheme = resolved_scheme();
        generate_css_variables(&t, scheme)
    });

    let scheme_class = resolved_scheme().class();

    rsx! {
        style { "{css_vars}" }
        div {
            class: scheme_class,
            "data-slot": "theme-root",
            "data-theme": theme().name,
            "data-color-scheme": "{scheme_class}",
            {props.children}
        }
    }
}

/// Hook to access the theme context.
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

/// Generate CSS custom property declarations for a theme.
fn generate_css_variables(theme: &Theme, scheme: ColorScheme) -> String {
    let colors = theme.colors(scheme);

    format!(
        r#"
:root {{
    --background: {background};
    --foreground: {foreground};
    --card: {card};
    --card-foreground: {card_foreground};
    --popover: {popover};
    --popover-foreground: {popover_foreground};
    --primary: {primary};
    --primary-foreground: {primary_foreground};
    --secondary: {secondary};
    --secondary-foreground: {secondary_foreground};
    --muted: {muted};
    --muted-foreground: {muted_foreground};
    --accent: {accent};
    --accent-foreground: {accent_foreground};
    --destructive: {destructive};
    --destructive-foreground: {destructive_foreground};
    --border: {border};
    --input: {input};
    --ring: {ring};
    --chart-1: {chart_1};
    --chart-2: {chart_2};
    --chart-3: {chart_3};
    --chart-4: {chart_4};
    --chart-5: {chart_5};
    --sidebar: {sidebar};
    --sidebar-foreground: {sidebar_foreground};
    --sidebar-primary: {sidebar_primary};
    --sidebar-primary-foreground: {sidebar_primary_foreground};
    --sidebar-accent: {sidebar_accent};
    --sidebar-accent-foreground: {sidebar_accent_foreground};
    --sidebar-border: {sidebar_border};
    --sidebar-ring: {sidebar_ring};
    --radius: {radius}rem;
}}
"#,
        background = colors.background,
        foreground = colors.foreground,
        card = colors.card,
        card_foreground = colors.card_foreground,
        popover = colors.popover,
        popover_foreground = colors.popover_foreground,
        primary = colors.primary,
        primary_foreground = colors.primary_foreground,
        secondary = colors.secondary,
        secondary_foreground = colors.secondary_foreground,
        muted = colors.muted,
        muted_foreground = colors.muted_foreground,
        accent = colors.accent,
        accent_foreground = colors.accent_foreground,
        destructive = colors.destructive,
        destructive_foreground = colors.destructive_foreground,
        border = colors.border,
        input = colors.input,
        ring = colors.ring,
        chart_1 = colors.chart_1,
        chart_2 = colors.chart_2,
        chart_3 = colors.chart_3,
        chart_4 = colors.chart_4,
        chart_5 = colors.chart_5,
        sidebar = colors.sidebar,
        sidebar_foreground = colors.sidebar_foreground,
        sidebar_primary = colors.sidebar_primary,
        sidebar_primary_foreground = colors.sidebar_primary_foreground,
        sidebar_accent = colors.sidebar_accent,
        sidebar_accent_foreground = colors.sidebar_accent_foreground,
        sidebar_border = colors.sidebar_border,
        sidebar_ring = colors.sidebar_ring,
        radius = theme.radius,
    )
}

/// Predefined themes matching shadcn-ui.
pub mod themes {
    use super::*;

    /// Neutral gray theme (default shadcn theme).
    pub fn neutral() -> Theme {
        Theme {
            name: "neutral".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::neutral(0.145),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::neutral(0.145),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::neutral(0.145),
                primary: OklchColor::neutral(0.205),
                primary_foreground: OklchColor::neutral(0.985),
                secondary: OklchColor::neutral(0.97),
                secondary_foreground: OklchColor::neutral(0.205),
                muted: OklchColor::neutral(0.97),
                muted_foreground: OklchColor::neutral(0.556),
                accent: OklchColor::neutral(0.97),
                accent_foreground: OklchColor::neutral(0.205),
                destructive: OklchColor::new(0.5757, 0.2352, 27.92),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::neutral(0.922),
                input: OklchColor::neutral(0.922),
                ring: OklchColor::neutral(0.708),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::neutral(0.985),
                sidebar_foreground: OklchColor::neutral(0.145),
                sidebar_primary: OklchColor::neutral(0.205),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::neutral(0.97),
                sidebar_accent_foreground: OklchColor::neutral(0.205),
                sidebar_border: OklchColor::neutral(0.922),
                sidebar_ring: OklchColor::neutral(0.708),
            },
            dark: ThemeColors {
                background: OklchColor::neutral(0.145),
                foreground: OklchColor::neutral(0.985),
                card: OklchColor::neutral(0.145),
                card_foreground: OklchColor::neutral(0.985),
                popover: OklchColor::neutral(0.145),
                popover_foreground: OklchColor::neutral(0.985),
                primary: OklchColor::neutral(0.985),
                primary_foreground: OklchColor::neutral(0.205),
                secondary: OklchColor::neutral(0.269),
                secondary_foreground: OklchColor::neutral(0.985),
                muted: OklchColor::neutral(0.269),
                muted_foreground: OklchColor::neutral(0.708),
                accent: OklchColor::neutral(0.269),
                accent_foreground: OklchColor::neutral(0.985),
                destructive: OklchColor::new(0.5058, 0.2066, 27.85),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::neutral(0.269),
                input: OklchColor::neutral(0.269),
                ring: OklchColor::neutral(0.439),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::neutral(0.205),
                sidebar_foreground: OklchColor::neutral(0.985),
                sidebar_primary: OklchColor::new(0.488, 0.243, 264.376),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::neutral(0.269),
                sidebar_accent_foreground: OklchColor::neutral(0.985),
                sidebar_border: OklchColor::neutral(0.269),
                sidebar_ring: OklchColor::neutral(0.439),
            },
            radius: 0.625,
        }
    }

    /// Zinc gray theme.
    pub fn zinc() -> Theme {
        Theme {
            name: "zinc".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::new(0.141, 0.005, 285.823),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::new(0.141, 0.005, 285.823),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::new(0.141, 0.005, 285.823),
                primary: OklchColor::new(0.21, 0.006, 285.885),
                primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                secondary: OklchColor::new(0.967, 0.001, 286.375),
                secondary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                muted: OklchColor::new(0.967, 0.001, 286.375),
                muted_foreground: OklchColor::new(0.552, 0.016, 285.938),
                accent: OklchColor::new(0.967, 0.001, 286.375),
                accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                destructive: OklchColor::new(0.577, 0.245, 27.325),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.92, 0.004, 286.32),
                input: OklchColor::new(0.92, 0.004, 286.32),
                ring: OklchColor::new(0.705, 0.015, 286.067),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_foreground: OklchColor::new(0.141, 0.005, 285.823),
                sidebar_primary: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.967, 0.001, 286.375),
                sidebar_accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_border: OklchColor::new(0.92, 0.004, 286.32),
                sidebar_ring: OklchColor::new(0.705, 0.015, 286.067),
            },
            dark: ThemeColors {
                background: OklchColor::new(0.141, 0.005, 285.823),
                foreground: OklchColor::new(0.985, 0.002, 247.839),
                card: OklchColor::new(0.141, 0.005, 285.823),
                card_foreground: OklchColor::new(0.985, 0.002, 247.839),
                popover: OklchColor::new(0.141, 0.005, 285.823),
                popover_foreground: OklchColor::new(0.985, 0.002, 247.839),
                primary: OklchColor::new(0.985, 0.002, 247.839),
                primary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                secondary: OklchColor::new(0.274, 0.006, 286.033),
                secondary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                muted: OklchColor::new(0.274, 0.006, 286.033),
                muted_foreground: OklchColor::new(0.705, 0.015, 286.067),
                accent: OklchColor::new(0.274, 0.006, 286.033),
                accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                destructive: OklchColor::new(0.704, 0.191, 22.216),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.274, 0.006, 286.033),
                input: OklchColor::new(0.274, 0.006, 286.033),
                ring: OklchColor::new(0.442, 0.017, 285.786),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_primary: OklchColor::new(0.488, 0.243, 264.376),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_border: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_ring: OklchColor::new(0.442, 0.017, 285.786),
            },
            radius: 0.625,
        }
    }

    /// Blue theme.
    pub fn blue() -> Theme {
        Theme {
            name: "blue".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::new(0.141, 0.005, 285.823),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::new(0.141, 0.005, 285.823),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::new(0.141, 0.005, 285.823),
                primary: OklchColor::new(0.546, 0.245, 262.881),
                primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                secondary: OklchColor::new(0.967, 0.001, 286.375),
                secondary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                muted: OklchColor::new(0.967, 0.001, 286.375),
                muted_foreground: OklchColor::new(0.552, 0.016, 285.938),
                accent: OklchColor::new(0.967, 0.001, 286.375),
                accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                destructive: OklchColor::new(0.577, 0.245, 27.325),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.92, 0.004, 286.32),
                input: OklchColor::new(0.92, 0.004, 286.32),
                ring: OklchColor::new(0.546, 0.245, 262.881),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_foreground: OklchColor::new(0.141, 0.005, 285.823),
                sidebar_primary: OklchColor::new(0.546, 0.245, 262.881),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.967, 0.001, 286.375),
                sidebar_accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_border: OklchColor::new(0.92, 0.004, 286.32),
                sidebar_ring: OklchColor::new(0.546, 0.245, 262.881),
            },
            dark: ThemeColors {
                background: OklchColor::new(0.141, 0.005, 285.823),
                foreground: OklchColor::new(0.985, 0.002, 247.839),
                card: OklchColor::new(0.141, 0.005, 285.823),
                card_foreground: OklchColor::new(0.985, 0.002, 247.839),
                popover: OklchColor::new(0.141, 0.005, 285.823),
                popover_foreground: OklchColor::new(0.985, 0.002, 247.839),
                primary: OklchColor::new(0.546, 0.245, 262.881),
                primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                secondary: OklchColor::new(0.274, 0.006, 286.033),
                secondary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                muted: OklchColor::new(0.274, 0.006, 286.033),
                muted_foreground: OklchColor::new(0.705, 0.015, 286.067),
                accent: OklchColor::new(0.274, 0.006, 286.033),
                accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                destructive: OklchColor::new(0.704, 0.191, 22.216),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.274, 0.006, 286.033),
                input: OklchColor::new(0.274, 0.006, 286.033),
                ring: OklchColor::new(0.546, 0.245, 262.881),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_primary: OklchColor::new(0.546, 0.245, 262.881),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_border: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_ring: OklchColor::new(0.546, 0.245, 262.881),
            },
            radius: 0.625,
        }
    }

    /// Green theme.
    pub fn green() -> Theme {
        Theme {
            name: "green".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::new(0.141, 0.005, 285.823),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::new(0.141, 0.005, 285.823),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::new(0.141, 0.005, 285.823),
                primary: OklchColor::new(0.527, 0.154, 150.069),
                primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                secondary: OklchColor::new(0.967, 0.001, 286.375),
                secondary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                muted: OklchColor::new(0.967, 0.001, 286.375),
                muted_foreground: OklchColor::new(0.552, 0.016, 285.938),
                accent: OklchColor::new(0.967, 0.001, 286.375),
                accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                destructive: OklchColor::new(0.577, 0.245, 27.325),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.92, 0.004, 286.32),
                input: OklchColor::new(0.92, 0.004, 286.32),
                ring: OklchColor::new(0.527, 0.154, 150.069),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_foreground: OklchColor::new(0.141, 0.005, 285.823),
                sidebar_primary: OklchColor::new(0.527, 0.154, 150.069),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.967, 0.001, 286.375),
                sidebar_accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_border: OklchColor::new(0.92, 0.004, 286.32),
                sidebar_ring: OklchColor::new(0.527, 0.154, 150.069),
            },
            dark: ThemeColors {
                background: OklchColor::new(0.141, 0.005, 285.823),
                foreground: OklchColor::new(0.985, 0.002, 247.839),
                card: OklchColor::new(0.141, 0.005, 285.823),
                card_foreground: OklchColor::new(0.985, 0.002, 247.839),
                popover: OklchColor::new(0.141, 0.005, 285.823),
                popover_foreground: OklchColor::new(0.985, 0.002, 247.839),
                primary: OklchColor::new(0.527, 0.154, 150.069),
                primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                secondary: OklchColor::new(0.274, 0.006, 286.033),
                secondary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                muted: OklchColor::new(0.274, 0.006, 286.033),
                muted_foreground: OklchColor::new(0.705, 0.015, 286.067),
                accent: OklchColor::new(0.274, 0.006, 286.033),
                accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                destructive: OklchColor::new(0.704, 0.191, 22.216),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.274, 0.006, 286.033),
                input: OklchColor::new(0.274, 0.006, 286.033),
                ring: OklchColor::new(0.527, 0.154, 150.069),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_primary: OklchColor::new(0.527, 0.154, 150.069),
                sidebar_primary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_accent: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_border: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_ring: OklchColor::new(0.527, 0.154, 150.069),
            },
            radius: 0.625,
        }
    }

    /// Red/Rose theme.
    pub fn red() -> Theme {
        Theme {
            name: "red".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::new(0.141, 0.005, 285.823),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::new(0.141, 0.005, 285.823),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::new(0.141, 0.005, 285.823),
                primary: OklchColor::new(0.577, 0.245, 27.325),
                primary_foreground: OklchColor::neutral(0.985),
                secondary: OklchColor::new(0.967, 0.001, 286.375),
                secondary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                muted: OklchColor::new(0.967, 0.001, 286.375),
                muted_foreground: OklchColor::new(0.552, 0.016, 285.938),
                accent: OklchColor::new(0.967, 0.001, 286.375),
                accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                destructive: OklchColor::new(0.577, 0.245, 27.325),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.92, 0.004, 286.32),
                input: OklchColor::new(0.92, 0.004, 286.32),
                ring: OklchColor::new(0.577, 0.245, 27.325),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_foreground: OklchColor::new(0.141, 0.005, 285.823),
                sidebar_primary: OklchColor::new(0.577, 0.245, 27.325),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::new(0.967, 0.001, 286.375),
                sidebar_accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_border: OklchColor::new(0.92, 0.004, 286.32),
                sidebar_ring: OklchColor::new(0.577, 0.245, 27.325),
            },
            dark: ThemeColors {
                background: OklchColor::new(0.141, 0.005, 285.823),
                foreground: OklchColor::new(0.985, 0.002, 247.839),
                card: OklchColor::new(0.141, 0.005, 285.823),
                card_foreground: OklchColor::new(0.985, 0.002, 247.839),
                popover: OklchColor::new(0.141, 0.005, 285.823),
                popover_foreground: OklchColor::new(0.985, 0.002, 247.839),
                primary: OklchColor::new(0.704, 0.191, 22.216),
                primary_foreground: OklchColor::neutral(0.985),
                secondary: OklchColor::new(0.274, 0.006, 286.033),
                secondary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                muted: OklchColor::new(0.274, 0.006, 286.033),
                muted_foreground: OklchColor::new(0.705, 0.015, 286.067),
                accent: OklchColor::new(0.274, 0.006, 286.033),
                accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                destructive: OklchColor::new(0.704, 0.191, 22.216),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.274, 0.006, 286.033),
                input: OklchColor::new(0.274, 0.006, 286.033),
                ring: OklchColor::new(0.704, 0.191, 22.216),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_primary: OklchColor::new(0.704, 0.191, 22.216),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_border: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_ring: OklchColor::new(0.704, 0.191, 22.216),
            },
            radius: 0.625,
        }
    }

    /// Violet/Purple theme.
    pub fn violet() -> Theme {
        Theme {
            name: "violet".to_string(),
            light: ThemeColors {
                background: OklchColor::neutral(1.0),
                foreground: OklchColor::new(0.141, 0.005, 285.823),
                card: OklchColor::neutral(1.0),
                card_foreground: OklchColor::new(0.141, 0.005, 285.823),
                popover: OklchColor::neutral(1.0),
                popover_foreground: OklchColor::new(0.141, 0.005, 285.823),
                primary: OklchColor::new(0.541, 0.281, 293.009),
                primary_foreground: OklchColor::neutral(0.985),
                secondary: OklchColor::new(0.967, 0.001, 286.375),
                secondary_foreground: OklchColor::new(0.21, 0.006, 285.885),
                muted: OklchColor::new(0.967, 0.001, 286.375),
                muted_foreground: OklchColor::new(0.552, 0.016, 285.938),
                accent: OklchColor::new(0.967, 0.001, 286.375),
                accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                destructive: OklchColor::new(0.577, 0.245, 27.325),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.92, 0.004, 286.32),
                input: OklchColor::new(0.92, 0.004, 286.32),
                ring: OklchColor::new(0.541, 0.281, 293.009),
                chart_1: OklchColor::new(0.646, 0.222, 41.116),
                chart_2: OklchColor::new(0.6, 0.118, 184.704),
                chart_3: OklchColor::new(0.398, 0.07, 227.392),
                chart_4: OklchColor::new(0.828, 0.189, 84.429),
                chart_5: OklchColor::new(0.769, 0.188, 70.08),
                sidebar: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_foreground: OklchColor::new(0.141, 0.005, 285.823),
                sidebar_primary: OklchColor::new(0.541, 0.281, 293.009),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::new(0.967, 0.001, 286.375),
                sidebar_accent_foreground: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_border: OklchColor::new(0.92, 0.004, 286.32),
                sidebar_ring: OklchColor::new(0.541, 0.281, 293.009),
            },
            dark: ThemeColors {
                background: OklchColor::new(0.141, 0.005, 285.823),
                foreground: OklchColor::new(0.985, 0.002, 247.839),
                card: OklchColor::new(0.141, 0.005, 285.823),
                card_foreground: OklchColor::new(0.985, 0.002, 247.839),
                popover: OklchColor::new(0.141, 0.005, 285.823),
                popover_foreground: OklchColor::new(0.985, 0.002, 247.839),
                primary: OklchColor::new(0.541, 0.281, 293.009),
                primary_foreground: OklchColor::neutral(0.985),
                secondary: OklchColor::new(0.274, 0.006, 286.033),
                secondary_foreground: OklchColor::new(0.985, 0.002, 247.839),
                muted: OklchColor::new(0.274, 0.006, 286.033),
                muted_foreground: OklchColor::new(0.705, 0.015, 286.067),
                accent: OklchColor::new(0.274, 0.006, 286.033),
                accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                destructive: OklchColor::new(0.704, 0.191, 22.216),
                destructive_foreground: OklchColor::neutral(0.985),
                border: OklchColor::new(0.274, 0.006, 286.033),
                input: OklchColor::new(0.274, 0.006, 286.033),
                ring: OklchColor::new(0.541, 0.281, 293.009),
                chart_1: OklchColor::new(0.488, 0.243, 264.376),
                chart_2: OklchColor::new(0.696, 0.17, 162.48),
                chart_3: OklchColor::new(0.769, 0.188, 70.08),
                chart_4: OklchColor::new(0.627, 0.265, 303.9),
                chart_5: OklchColor::new(0.645, 0.246, 16.439),
                sidebar: OklchColor::new(0.21, 0.006, 285.885),
                sidebar_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_primary: OklchColor::new(0.541, 0.281, 293.009),
                sidebar_primary_foreground: OklchColor::neutral(0.985),
                sidebar_accent: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_accent_foreground: OklchColor::new(0.985, 0.002, 247.839),
                sidebar_border: OklchColor::new(0.274, 0.006, 286.033),
                sidebar_ring: OklchColor::new(0.541, 0.281, 293.009),
            },
            radius: 0.625,
        }
    }
}
