//! Tooltip component for displaying contextual information on hover.

use dioxus::prelude::*;

/// Context for managing global tooltip settings.
#[derive(Clone, Copy)]
pub struct TooltipProviderContext {
    /// Default delay before showing tooltips (in ms).
    pub delay_ms: u32,
    /// Whether to skip delay for subsequent tooltips.
    pub skip_delay_duration: u32,
}

impl Default for TooltipProviderContext {
    fn default() -> Self {
        Self {
            delay_ms: 200,
            skip_delay_duration: 300,
        }
    }
}

/// Props for TooltipProvider.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProviderProps {
    /// Default delay before showing tooltips (in ms).
    #[props(default = 200)]
    pub delay_ms: u32,

    /// Duration to skip delay after showing a tooltip (in ms).
    #[props(default = 300)]
    pub skip_delay_duration: u32,

    /// Children elements.
    pub children: Element,
}

/// Provides global tooltip configuration for all nested tooltips.
///
/// # Example
///
/// ```rust
/// rsx! {
///     TooltipProvider {
///         delay_ms: 400,
///
///         // All tooltips inside will use 400ms delay by default
///         Tooltip { content: "First tooltip", Button { "One" } }
///         Tooltip { content: "Second tooltip", Button { "Two" } }
///     }
/// }
/// ```
#[component]
pub fn TooltipProvider(props: TooltipProviderProps) -> Element {
    let context = TooltipProviderContext {
        delay_ms: props.delay_ms,
        skip_delay_duration: props.skip_delay_duration,
    };

    use_context_provider(|| context);

    rsx! {
        div {
            "data-slot": "tooltip-provider",
            {props.children}
        }
    }
}

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

    fn arrow_position_classes(&self) -> &'static str {
        match self {
            TooltipSide::Top => "bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2 rotate-45",
            TooltipSide::Right => "left-0 top-1/2 -translate-y-1/2 -translate-x-1/2 rotate-45",
            TooltipSide::Bottom => "top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 rotate-45",
            TooltipSide::Left => "right-0 top-1/2 -translate-y-1/2 translate-x-1/2 rotate-45",
        }
    }
}

/// Props for the Tooltip component.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// The text content to show in the tooltip.
    /// Use this for simple text tooltips.
    #[props(default)]
    pub content: Option<String>,

    /// The element content to show in the tooltip.
    /// Use this for rich content tooltips.
    #[props(default)]
    pub content_element: Option<Element>,

    /// Which side the tooltip appears on.
    #[props(default)]
    pub side: TooltipSide,

    /// Delay before showing the tooltip (in ms).
    #[props(default = 200)]
    pub delay_ms: u32,

    /// Whether to show the arrow pointing to the trigger.
    #[props(default = true)]
    pub show_arrow: bool,

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
///     // Simple text tooltip
///     Tooltip {
///         content: "This is helpful information",
///
///         Button { "Hover me" }
///     }
///
///     // Rich content tooltip
///     Tooltip {
///         content_element: rsx! {
///             div { class: "flex items-center gap-2",
///                 span { "Settings" }
///                 kbd { class: "text-xs", "⌘S" }
///             }
///         },
///         side: TooltipSide::Right,
///
///         IconButton {
///             Settings {}
///         }
///     }
///
///     // Without arrow
///     Tooltip {
///         content: "No arrow",
///         show_arrow: false,
///
///         span { "Hover" }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut is_visible = use_signal(|| false);
    let mut is_hovering = use_signal(|| false);

    // Get delay from provider context if available, otherwise use prop
    let provider_context = try_use_context::<TooltipProviderContext>();
    let effective_delay = props.delay_ms.max(
        provider_context.map(|ctx| ctx.delay_ms).unwrap_or(0)
    );
    // If prop is 200 (default), prefer provider's delay
    let delay = if props.delay_ms == 200 {
        provider_context.map(|ctx| ctx.delay_ms).unwrap_or(props.delay_ms)
    } else {
        props.delay_ms
    };
    let _ = (effective_delay, delay); // Reserved for skip delay logic

    let custom_class = props.class.as_deref().unwrap_or("");
    let position_classes = props.side.position_classes();
    let animation_classes = props.side.animation_classes();
    let arrow_classes = props.side.arrow_position_classes();

    let tooltip_classes = format!(
        "absolute z-50 w-fit rounded-md bg-foreground px-3 py-1.5 \
         text-xs text-balance text-background \
         data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
         {} {} {}",
        position_classes, animation_classes, custom_class
    );

    // Handle hover with delay (using provider's delay if available)
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

                    // Render content - either text or element
                    if let Some(ref text) = props.content {
                        "{text}"
                    } else if let Some(element) = &props.content_element {
                        {element.clone()}
                    }

                    // Arrow element
                    if props.show_arrow {
                        div {
                            class: "absolute size-2 bg-foreground {arrow_classes}",
                            "data-slot": "tooltip-arrow",
                            "aria-hidden": "true",
                        }
                    }
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
