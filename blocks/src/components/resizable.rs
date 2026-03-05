//! Resizable component for split panel layouts.
//!
//! A set of components for creating resizable panel layouts with
//! draggable handles.

use dioxus::prelude::*;

/// Direction for the resizable panel group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Context for resizable panel state.
#[derive(Clone, Copy)]
pub struct ResizableContext {
    pub direction: ResizableDirection,
}

/// Props for ResizablePanelGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelGroupProps {
    /// Layout direction.
    #[props(default)]
    pub direction: ResizableDirection,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Panel children.
    pub children: Element,
}

/// A container for resizable panels.
///
/// # Example
///
/// ```rust
/// rsx! {
///     ResizablePanelGroup {
///         direction: ResizableDirection::Horizontal,
///
///         ResizablePanel { default_size: 50.0,
///             div { "Left panel" }
///         }
///         ResizableHandle { with_handle: true }
///         ResizablePanel { default_size: 50.0,
///             div { "Right panel" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    use_context_provider(|| ResizableContext {
        direction: props.direction,
    });

    let direction_class = match props.direction {
        ResizableDirection::Horizontal => "flex-row",
        ResizableDirection::Vertical => "flex-col",
    };

    let classes = format!(
        "flex h-full w-full {} {}",
        direction_class, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "resizable-panel-group",
            "data-direction": match props.direction {
                ResizableDirection::Horizontal => "horizontal",
                ResizableDirection::Vertical => "vertical",
            },
            "data-panel-group": "true",
            {props.children}
        }
    }
}

/// Props for ResizablePanel.
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelProps {
    /// Default size as a percentage (0-100).
    #[props(default = 50.0)]
    pub default_size: f64,

    /// Minimum size as a percentage.
    #[props(default)]
    pub min_size: Option<f64>,

    /// Maximum size as a percentage.
    #[props(default)]
    pub max_size: Option<f64>,

    /// Whether the panel is collapsible.
    #[props(default)]
    pub collapsible: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Panel content.
    pub children: Element,
}

/// A resizable panel within a panel group.
#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let context = use_context::<ResizableContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    // Calculate flex basis from default size
    let style = format!("flex-basis: {}%; flex-grow: 1; flex-shrink: 1;", props.default_size);

    let min_style = props.min_size.map(|s| format!("min-{}: {}%;",
        match context.direction {
            ResizableDirection::Horizontal => "width",
            ResizableDirection::Vertical => "height",
        }, s));

    let max_style = props.max_size.map(|s| format!("max-{}: {}%;",
        match context.direction {
            ResizableDirection::Horizontal => "width",
            ResizableDirection::Vertical => "height",
        }, s));

    let full_style = format!(
        "{} {} {}",
        style,
        min_style.unwrap_or_default(),
        max_style.unwrap_or_default()
    );

    let classes = format!("overflow-hidden {}", custom_class);

    rsx! {
        div {
            class: classes,
            style: full_style,
            "data-slot": "resizable-panel",
            "data-panel": "true",
            "data-collapsible": props.collapsible.to_string(),
            {props.children}
        }
    }
}

/// Props for ResizableHandle.
#[derive(Props, Clone, PartialEq)]
pub struct ResizableHandleProps {
    /// Whether to show a grip handle icon.
    #[props(default)]
    pub with_handle: bool,

    /// Whether the handle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A draggable handle between resizable panels.
#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let context = use_context::<ResizableContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let direction_class = match context.direction {
        ResizableDirection::Horizontal => "w-px cursor-col-resize",
        ResizableDirection::Vertical => "h-px cursor-row-resize",
    };

    let classes = format!(
        "relative flex items-center justify-center bg-border \
         after:absolute after:inset-y-0 after:left-1/2 after:-translate-x-1/2 \
         focus-visible:outline-1 focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 \
         {} {} {}",
        direction_class,
        match context.direction {
            ResizableDirection::Horizontal => "after:w-1",
            ResizableDirection::Vertical => "after:h-1",
        },
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "resizable-handle",
            "data-panel-resize-handle": "true",
            "data-disabled": props.disabled.to_string(),
            "data-direction": match context.direction {
                ResizableDirection::Horizontal => "horizontal",
                ResizableDirection::Vertical => "vertical",
            },
            role: "separator",
            tabindex: if props.disabled { "-1" } else { "0" },

            if props.with_handle {
                div {
                    class: "z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border",

                    // GripVertical icon
                    svg {
                        class: "size-2.5",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",

                        circle { cx: "9", cy: "12", r: "1" }
                        circle { cx: "9", cy: "5", r: "1" }
                        circle { cx: "9", cy: "19", r: "1" }
                        circle { cx: "15", cy: "12", r: "1" }
                        circle { cx: "15", cy: "5", r: "1" }
                        circle { cx: "15", cy: "19", r: "1" }
                    }
                }
            }
        }
    }
}
