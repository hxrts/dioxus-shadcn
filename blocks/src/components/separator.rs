//! Separator component for visual division between content.

use dioxus::prelude::*;

/// Orientation of the separator.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Props for the Separator component.
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Orientation of the separator.
    #[props(default)]
    pub orientation: SeparatorOrientation,

    /// Whether the separator is purely decorative.
    #[props(default = true)]
    pub decorative: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A visual divider between content sections.
///
/// # Example
///
/// ```rust
/// rsx! {
///     div { "Content above" }
///     Separator {}
///     div { "Content below" }
///
///     // Vertical separator
///     div { class: "flex h-5 items-center space-x-4",
///         div { "Left" }
///         Separator { orientation: SeparatorOrientation::Vertical }
///         div { "Right" }
///     }
/// }
/// ```
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full \
         data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "separator",
            "data-orientation": match props.orientation {
                SeparatorOrientation::Horizontal => "horizontal",
                SeparatorOrientation::Vertical => "vertical",
            },
            role: if props.decorative { "none" } else { "separator" },
            aria_orientation: if !props.decorative {
                Some(match props.orientation {
                    SeparatorOrientation::Horizontal => "horizontal",
                    SeparatorOrientation::Vertical => "vertical",
                })
            } else { None },
        }
    }
}
