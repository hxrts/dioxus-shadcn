//! Tooltip component for displaying contextual information on hover.

use dioxus::prelude::*;

/// Side where the tooltip appears.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    fn position_classes(&self) -> &'static str {
        match self {
            TooltipSide::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
            TooltipSide::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
            TooltipSide::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
            TooltipSide::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
        }
    }

    fn animation_classes(&self) -> &'static str {
        match self {
            TooltipSide::Top => "animate-in fade-in-0 zoom-in-95 slide-in-from-bottom-2",
            TooltipSide::Right => "animate-in fade-in-0 zoom-in-95 slide-in-from-left-2",
            TooltipSide::Bottom => "animate-in fade-in-0 zoom-in-95 slide-in-from-top-2",
            TooltipSide::Left => "animate-in fade-in-0 zoom-in-95 slide-in-from-right-2",
        }
    }
}

/// Props for the Tooltip component.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// The content to show in the tooltip.
    pub content: String,

    /// Which side the tooltip appears on.
    #[props(default)]
    pub side: TooltipSide,

    /// Delay before showing the tooltip (in ms).
    #[props(default = 200)]
    pub delay_ms: u32,

    /// Additional CSS classes for the tooltip content.
    #[props(default)]
    pub class: Option<String>,

    /// The trigger element.
    pub children: Element,
}

/// A popup that displays information when hovering over an element.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Tooltip {
///         content: "This is helpful information",
///
///         Button { "Hover me" }
///     }
///
///     Tooltip {
///         content: "Settings",
///         side: TooltipSide::Right,
///
///         IconButton {
///             Settings {}
///         }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut is_visible = use_signal(|| false);
    let mut is_hovering = use_signal(|| false);

    let custom_class = props.class.as_deref().unwrap_or("");
    let position_classes = props.side.position_classes();
    let animation_classes = props.side.animation_classes();

    let tooltip_classes = format!(
        "absolute z-50 w-fit rounded-md bg-foreground px-3 py-1.5 \
         text-xs text-balance text-background {} {} {}",
        position_classes, animation_classes, custom_class
    );

    // Handle hover with delay
    let _delay = props.delay_ms;
    use_effect(move || {
        if *is_hovering.read() {
            spawn(async move {
                // Simple delay using async sleep
                #[cfg(target_arch = "wasm32")]
                {
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        let window = web_sys::window().unwrap();
                        window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                &resolve,
                                delay as i32,
                            )
                            .unwrap();
                    });
                    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                }
                if *is_hovering.read() {
                    is_visible.set(true);
                }
            });
        } else {
            is_visible.set(false);
        }
    });

    let on_mouse_enter = move |_| {
        is_hovering.set(true);
    };

    let on_mouse_leave = move |_| {
        is_hovering.set(false);
    };

    let on_focus = move |_| {
        is_visible.set(true);
    };

    let on_blur = move |_| {
        is_visible.set(false);
    };

    rsx! {
        div {
            class: "relative inline-block",
            "data-slot": "tooltip",
            onmouseenter: on_mouse_enter,
            onmouseleave: on_mouse_leave,
            onfocus: on_focus,
            onblur: on_blur,

            // Trigger
            {props.children}

            // Tooltip content
            if *is_visible.read() {
                div {
                    class: tooltip_classes,
                    role: "tooltip",
                    "data-slot": "tooltip-content",
                    "data-side": match props.side {
                        TooltipSide::Top => "top",
                        TooltipSide::Right => "right",
                        TooltipSide::Bottom => "bottom",
                        TooltipSide::Left => "left",
                    },
                    "{props.content}"
                }
            }
        }
    }
}

/// A simpler tooltip that uses the native title attribute.
/// Less customizable but works without JavaScript.
#[derive(Props, Clone, PartialEq)]
pub struct SimpleTooltipProps {
    /// The tooltip text.
    pub title: String,

    /// The trigger element.
    pub children: Element,
}

/// A simple tooltip using the native title attribute.
#[component]
pub fn SimpleTooltip(props: SimpleTooltipProps) -> Element {
    rsx! {
        span {
            title: props.title,
            "data-slot": "simple-tooltip",
            {props.children}
        }
    }
}
