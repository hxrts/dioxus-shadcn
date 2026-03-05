//! ScrollArea component for custom scrollbars.

use dioxus::prelude::*;

/// Scrollbar visibility options.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScrollbarVisibility {
    #[default]
    Auto,
    Always,
    Hover,
    Never,
}

/// Props for ScrollArea.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// Height of the scroll area. Can be any CSS value.
    #[props(default)]
    pub height: Option<String>,

    /// Maximum height of the scroll area.
    #[props(default)]
    pub max_height: Option<String>,

    /// When to show the scrollbar.
    #[props(default)]
    pub scrollbar: ScrollbarVisibility,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content to scroll.
    pub children: Element,
}

/// A scrollable area with styled scrollbars.
///
/// # Example
///
/// ```rust
/// rsx! {
///     ScrollArea {
///         height: "200px",
///
///         div { class: "p-4",
///             // Long content here
///             for i in 0..50 {
///                 p { "Item {i}" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let scrollbar_classes = match props.scrollbar {
        ScrollbarVisibility::Auto => "",
        ScrollbarVisibility::Always => "overflow-y-scroll",
        ScrollbarVisibility::Hover => "[&::-webkit-scrollbar]:opacity-0 [&:hover::-webkit-scrollbar]:opacity-100",
        ScrollbarVisibility::Never => "scrollbar-none",
    };

    let mut style_parts = vec![];
    if let Some(h) = &props.height {
        style_parts.push(format!("height: {}", h));
    }
    if let Some(mh) = &props.max_height {
        style_parts.push(format!("max-height: {}", mh));
    }
    let style = style_parts.join("; ");

    let classes = format!(
        "relative overflow-auto \
         [&::-webkit-scrollbar]:w-2.5 \
         [&::-webkit-scrollbar]:h-2.5 \
         [&::-webkit-scrollbar-track]:bg-transparent \
         [&::-webkit-scrollbar-thumb]:rounded-full \
         [&::-webkit-scrollbar-thumb]:bg-border \
         hover:[&::-webkit-scrollbar-thumb]:bg-muted-foreground/30 \
         {} {}",
        scrollbar_classes, custom_class
    );

    rsx! {
        div {
            class: classes,
            style: style,
            "data-slot": "scroll-area",
            "data-scrollbar": match props.scrollbar {
                ScrollbarVisibility::Auto => "auto",
                ScrollbarVisibility::Always => "always",
                ScrollbarVisibility::Hover => "hover",
                ScrollbarVisibility::Never => "never",
            },

            {props.children}
        }
    }
}

/// A horizontal scroll area.
#[derive(Props, Clone, PartialEq)]
pub struct HorizontalScrollAreaProps {
    /// Width of the scroll area.
    #[props(default)]
    pub width: Option<String>,

    /// Maximum width.
    #[props(default)]
    pub max_width: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content to scroll.
    pub children: Element,
}

/// A horizontally scrollable area.
#[component]
pub fn HorizontalScrollArea(props: HorizontalScrollAreaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let mut style_parts = vec![];
    if let Some(w) = &props.width {
        style_parts.push(format!("width: {}", w));
    }
    if let Some(mw) = &props.max_width {
        style_parts.push(format!("max-width: {}", mw));
    }
    let style = style_parts.join("; ");

    let classes = format!(
        "relative overflow-x-auto overflow-y-hidden whitespace-nowrap \
         [&::-webkit-scrollbar]:h-2.5 \
         [&::-webkit-scrollbar-track]:bg-transparent \
         [&::-webkit-scrollbar-thumb]:rounded-full \
         [&::-webkit-scrollbar-thumb]:bg-border \
         hover:[&::-webkit-scrollbar-thumb]:bg-muted-foreground/30 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            style: style,
            "data-slot": "horizontal-scroll-area",
            {props.children}
        }
    }
}
