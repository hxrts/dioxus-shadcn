//! ScrollArea component for custom scrollbars.
//!
//! A scrollable area with styled scrollbars matching shadcn-ui patterns.

use dioxus::prelude::*;

/// Scrollbar orientation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScrollbarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Scrollbar visibility options.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScrollbarVisibility {
    #[default]
    Auto,
    Always,
    Hover,
    Never,
}

/// Context for scroll area state.
#[derive(Clone)]
pub struct ScrollAreaContext {
    /// Whether to show the scrollbar.
    pub visibility: ScrollbarVisibility,
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

    /// Width of the scroll area.
    #[props(default)]
    pub width: Option<String>,

    /// Maximum width of the scroll area.
    #[props(default)]
    pub max_width: Option<String>,

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
///         ScrollAreaViewport {
///             div { class: "p-4",
///                 for i in 0..50 {
///                     p { "Item {i}" }
///                 }
///             }
///         }
///         ScrollBar { orientation: ScrollbarOrientation::Vertical }
///     }
/// }
/// ```
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    // Provide context for child components
    use_context_provider(|| ScrollAreaContext {
        visibility: props.scrollbar,
    });

    let mut style_parts = vec![];
    if let Some(h) = &props.height {
        style_parts.push(format!("height: {}", h));
    }
    if let Some(mh) = &props.max_height {
        style_parts.push(format!("max-height: {}", mh));
    }
    if let Some(w) = &props.width {
        style_parts.push(format!("width: {}", w));
    }
    if let Some(mw) = &props.max_width {
        style_parts.push(format!("max-width: {}", mw));
    }
    let style = style_parts.join("; ");

    let classes = format!("relative {}", custom_class);

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

/// Props for ScrollAreaViewport.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaViewportProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content to scroll.
    pub children: Element,
}

/// The scrollable viewport container.
#[component]
pub fn ScrollAreaViewport(props: ScrollAreaViewportProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "size-full rounded-[inherit] overflow-auto \
         transition-[color,box-shadow] outline-none \
         focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 \
         [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none] \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "scroll-area-viewport",
            tabindex: "0",

            {props.children}
        }
    }
}

/// Props for ScrollBar.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollBarProps {
    /// Scrollbar orientation.
    #[props(default)]
    pub orientation: ScrollbarOrientation,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A styled scrollbar indicator.
#[component]
pub fn ScrollBar(props: ScrollBarProps) -> Element {
    let context = try_use_context::<ScrollAreaContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    // Check visibility
    let visibility = context
        .map(|ctx| ctx.visibility)
        .unwrap_or(ScrollbarVisibility::Auto);

    if matches!(visibility, ScrollbarVisibility::Never) {
        return rsx! {};
    }

    let orientation_classes = match props.orientation {
        ScrollbarOrientation::Vertical => "h-full w-2.5 border-l border-l-transparent p-px",
        ScrollbarOrientation::Horizontal => "h-2.5 w-full flex-col border-t border-t-transparent p-px",
    };

    let visibility_classes = match visibility {
        ScrollbarVisibility::Auto => "",
        ScrollbarVisibility::Always => "",
        ScrollbarVisibility::Hover => "opacity-0 group-hover/scroll-area:opacity-100 transition-opacity",
        ScrollbarVisibility::Never => "hidden",
    };

    let position_classes = match props.orientation {
        ScrollbarOrientation::Vertical => "absolute right-0 top-0 bottom-0",
        ScrollbarOrientation::Horizontal => "absolute bottom-0 left-0 right-0",
    };

    let classes = format!(
        "flex touch-none select-none {} {} {} {}",
        orientation_classes, visibility_classes, position_classes, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "scroll-area-scrollbar",
            "data-orientation": match props.orientation {
                ScrollbarOrientation::Vertical => "vertical",
                ScrollbarOrientation::Horizontal => "horizontal",
            },
            "data-state": "visible",

            ScrollAreaThumb { orientation: props.orientation }
        }
    }
}

/// Props for ScrollAreaThumb.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaThumbProps {
    /// Scrollbar orientation (inherited from parent).
    #[props(default)]
    pub orientation: ScrollbarOrientation,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// The draggable scrollbar thumb.
#[component]
pub fn ScrollAreaThumb(props: ScrollAreaThumbProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "relative flex-1 rounded-full bg-border {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "scroll-area-thumb",
        }
    }
}

/// Props for ScrollAreaCorner.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaCornerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Corner element where scrollbars meet.
#[component]
pub fn ScrollAreaCorner(props: ScrollAreaCornerProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "absolute right-0 bottom-0 w-2.5 h-2.5 bg-transparent {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "scroll-area-corner",
        }
    }
}

/// A horizontal scroll area (convenience wrapper).
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

    rsx! {
        ScrollArea {
            width: props.width,
            max_width: props.max_width,
            class: format!("group/scroll-area {}", custom_class),

            ScrollAreaViewport {
                class: "whitespace-nowrap",
                {props.children}
            }
            ScrollBar { orientation: ScrollbarOrientation::Horizontal }
        }
    }
}
