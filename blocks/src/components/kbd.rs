//! Kbd component for displaying keyboard keys.
//!
//! A styled keyboard key element for showing keyboard shortcuts.

use dioxus::prelude::*;

/// Props for Kbd.
#[derive(Props, Clone, PartialEq)]
pub struct KbdProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Key content.
    pub children: Element,
}

/// A keyboard key display component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     // Single key
///     Kbd { "K" }
///
///     // With modifier
///     KbdGroup {
///         Kbd { "Ctrl" }
///         Kbd { "C" }
///     }
///
///     // In a tooltip or menu
///     div {
///         "Copy"
///         Kbd { class: "ml-auto", "Ctrl+C" }
///     }
/// }
/// ```
#[component]
pub fn Kbd(props: KbdProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 \
         rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none \
         [&_svg:not([class*='size-'])]:size-3 \
         [[data-slot=tooltip-content]_&]:bg-background/20 [[data-slot=tooltip-content]_&]:text-background \
         dark:[[data-slot=tooltip-content]_&]:bg-background/10 \
         {}",
        custom_class
    );

    rsx! {
        kbd {
            class: classes,
            "data-slot": "kbd",
            {props.children}
        }
    }
}

/// Props for KbdGroup.
#[derive(Props, Clone, PartialEq)]
pub struct KbdGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Grouped keys.
    pub children: Element,
}

/// A container for grouping multiple keyboard keys.
///
/// # Example
///
/// ```rust
/// rsx! {
///     KbdGroup {
///         Kbd { "Cmd" }
///         Kbd { "Shift" }
///         Kbd { "P" }
///     }
/// }
/// ```
#[component]
pub fn KbdGroup(props: KbdGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("inline-flex items-center gap-1 {}", custom_class);

    rsx! {
        span {
            class: classes,
            "data-slot": "kbd-group",
            {props.children}
        }
    }
}
