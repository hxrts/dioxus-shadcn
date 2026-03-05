//! ButtonGroup component for grouping buttons.
//!
//! A container for grouping multiple buttons with consistent styling and spacing.

use dioxus::prelude::*;

/// Orientation variants for the button group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Props for ButtonGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// Layout orientation.
    #[props(default)]
    pub orientation: ButtonGroupOrientation,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Grouped buttons.
    pub children: Element,
}

/// A container for grouping multiple buttons.
///
/// Automatically handles border radius and spacing for grouped buttons.
///
/// # Example
///
/// ```rust
/// rsx! {
///     ButtonGroup {
///         Button { "Left" }
///         Button { "Center" }
///         Button { "Right" }
///     }
///
///     // Vertical orientation
///     ButtonGroup {
///         orientation: ButtonGroupOrientation::Vertical,
///
///         Button { "Top" }
///         Button { "Middle" }
///         Button { "Bottom" }
///     }
/// }
/// ```
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let orientation_class = match props.orientation {
        ButtonGroupOrientation::Horizontal => {
            "flex-row \
             [&>*:first-child]:rounded-r-none [&>*:last-child]:rounded-l-none \
             [&>*:not(:first-child):not(:last-child)]:rounded-none \
             [&>*:not(:first-child)]:-ml-px"
        }
        ButtonGroupOrientation::Vertical => {
            "flex-col \
             [&>*:first-child]:rounded-b-none [&>*:last-child]:rounded-t-none \
             [&>*:not(:first-child):not(:last-child)]:rounded-none \
             [&>*:not(:first-child)]:-mt-px"
        }
    };

    let classes = format!(
        "inline-flex \
         [&>*]:relative [&>*:focus-visible]:z-10 [&>*:hover]:z-10 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {} {}",
        orientation_class, custom_class
    );

    rsx! {
        div {
            role: "group",
            class: classes,
            "data-slot": "button-group",
            "data-orientation": match props.orientation {
                ButtonGroupOrientation::Horizontal => "horizontal",
                ButtonGroupOrientation::Vertical => "vertical",
            },
            {props.children}
        }
    }
}

/// Props for ButtonGroupText.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupTextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Text content.
    pub children: Element,
}

/// A text element within a button group.
///
/// Useful for displaying labels or static content between buttons.
#[component]
pub fn ButtonGroupText(props: ButtonGroupTextProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex items-center justify-center border border-input bg-background px-3 \
         text-sm font-medium text-muted-foreground shadow-xs \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "button-group-text",
            {props.children}
        }
    }
}

/// Props for ButtonGroupSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupSeparatorProps {
    /// Orientation of the separator.
    #[props(default)]
    pub orientation: ButtonGroupOrientation,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between button group items.
#[component]
pub fn ButtonGroupSeparator(props: ButtonGroupSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let orientation_class = match props.orientation {
        ButtonGroupOrientation::Horizontal => "h-full w-px",
        ButtonGroupOrientation::Vertical => "h-px w-full",
    };

    let classes = format!(
        "bg-border {} {}",
        orientation_class, custom_class
    );

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "button-group-separator",
            "data-orientation": match props.orientation {
                ButtonGroupOrientation::Horizontal => "vertical",
                ButtonGroupOrientation::Vertical => "horizontal",
            },
            "aria-orientation": match props.orientation {
                ButtonGroupOrientation::Horizontal => "vertical",
                ButtonGroupOrientation::Vertical => "horizontal",
            },
        }
    }
}
