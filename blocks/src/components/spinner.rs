//! Spinner component for loading states.
//!
//! A simple loading spinner using an animated icon.

use dioxus::prelude::*;
use lucide_dioxus::LoaderCircle;

/// Size variants for the spinner.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpinnerSize {
    /// Extra small (size-3)
    Xs,
    /// Small (size-4)
    #[default]
    Sm,
    /// Medium (size-5)
    Md,
    /// Large (size-6)
    Lg,
    /// Extra large (size-8)
    Xl,
}

impl SpinnerSize {
    fn class(&self) -> &'static str {
        match self {
            SpinnerSize::Xs => "size-3",
            SpinnerSize::Sm => "size-4",
            SpinnerSize::Md => "size-5",
            SpinnerSize::Lg => "size-6",
            SpinnerSize::Xl => "size-8",
        }
    }
}

/// Props for Spinner.
#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    /// Size of the spinner.
    #[props(default)]
    pub size: SpinnerSize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A loading spinner component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Spinner {}
///
///     // With custom size
///     Spinner { size: SpinnerSize::Lg }
///
///     // With custom class
///     Spinner { class: "text-primary" }
/// }
/// ```
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let size_class = props.size.class();

    let classes = format!("animate-spin {} {}", size_class, custom_class);

    rsx! {
        span {
            role: "status",
            "aria-label": "Loading",
            "data-slot": "spinner",

            LoaderCircle {
                class: classes,
            }
        }
    }
}
