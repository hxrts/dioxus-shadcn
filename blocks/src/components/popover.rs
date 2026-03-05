//! Popover component for floating content.
//!
//! A floating panel triggered by a button, positioned relative to its trigger.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing popover state.
#[derive(Clone)]
pub struct PopoverContext {
    /// Whether the popover is open.
    pub open: Signal<bool>,
    /// The trigger element ID.
    pub trigger_id: String,
    /// The content element ID.
    pub content_id: String,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

impl PopoverContext {
    /// Toggle the open state.
    pub fn toggle(&mut self) {
        let new_state = !*self.open.read();
        self.open.set(new_state);
        if let Some(callback) = &self.on_open_change {
            callback.call(new_state);
        }
    }

    /// Close the popover.
    pub fn close(&mut self) {
        self.open.set(false);
        if let Some(callback) = &self.on_open_change {
            callback.call(false);
        }
    }

    /// Open the popover.
    pub fn open_popover(&mut self) {
        self.open.set(true);
        if let Some(callback) = &self.on_open_change {
            callback.call(true);
        }
    }
}

/// Side for popover positioning.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PopoverSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

/// Alignment for popover positioning.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

/// Props for Popover.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Controlled open state.
    #[props(default)]
    pub open: Option<Signal<bool>>,

    /// Default open state for uncontrolled mode.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Popover content.
    pub children: Element,
}

/// A popover component for floating content.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Popover {
///         PopoverTrigger {
///             Button { "Open popover" }
///         }
///         PopoverContent {
///             PopoverHeader {
///                 PopoverTitle { "Dimensions" }
///                 PopoverDescription { "Set the dimensions for the layer." }
///             }
///             // Content here
///         }
///     }
/// }
/// ```
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let popover_id = use_unique_id();
    let trigger_id = format!("popover-trigger-{}", popover_id());
    let content_id = format!("popover-content-{}", popover_id());

    // Internal state for uncontrolled mode
    let internal_open = use_signal(|| props.default_open);

    // Use controlled or internal state
    let open = props.open.unwrap_or(internal_open);

    let context = PopoverContext {
        open,
        trigger_id,
        content_id,
        on_open_change: props.on_open_change.clone(),
    };

    use_context_provider(|| context);

    // Handle escape key to close
    let handle_keydown = {
        let mut open = open;
        let on_open_change = props.on_open_change.clone();
        move |event: KeyboardEvent| {
            if event.key() == Key::Escape && *open.read() {
                open.set(false);
                if let Some(callback) = &on_open_change {
                    callback.call(false);
                }
            }
        }
    };

    rsx! {
        div {
            class: "relative inline-block",
            "data-slot": "popover",
            "data-state": if *open.read() { "open" } else { "closed" },
            onkeydown: handle_keydown,

            {props.children}
        }
    }
}

/// Props for PopoverTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Whether the trigger is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content.
    pub children: Element,
}

/// The button that triggers the popover.
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let context = use_context::<PopoverContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = {
        let mut context = context.clone();
        let disabled = props.disabled;
        move |_| {
            if !disabled {
                context.toggle();
            }
        }
    };

    rsx! {
        div {
            id: context.trigger_id.clone(),
            class: custom_class,
            "data-slot": "popover-trigger",
            "data-state": if *context.open.read() { "open" } else { "closed" },
            aria_expanded: context.open.read().to_string(),
            aria_haspopup: "dialog",
            aria_controls: context.content_id.clone(),
            onclick: handle_click,

            {props.children}
        }
    }
}

/// Props for PopoverAnchor.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverAnchorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Anchor content.
    pub children: Element,
}

/// An optional anchor element for positioning the popover.
#[component]
pub fn PopoverAnchor(props: PopoverAnchorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: custom_class,
            "data-slot": "popover-anchor",
            {props.children}
        }
    }
}

/// Props for PopoverContent.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    /// Which side to position the popover.
    #[props(default)]
    pub side: PopoverSide,

    /// Alignment along the side.
    #[props(default)]
    pub align: PopoverAlign,

    /// Offset from the trigger in pixels.
    #[props(default = 4)]
    pub side_offset: i32,

    /// Width of the popover. Defaults to 18rem (w-72).
    #[props(default)]
    pub width: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Popover content.
    pub children: Element,
}

/// The floating content panel.
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let context = use_context::<PopoverContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    if !*context.open.read() {
        return rsx! {};
    }

    let width = props.width.as_deref().unwrap_or("18rem");

    let side_class = match props.side {
        PopoverSide::Top => "bottom-full mb-1",
        PopoverSide::Right => "left-full ml-1",
        PopoverSide::Bottom => "top-full mt-1",
        PopoverSide::Left => "right-full mr-1",
    };

    let align_class = match props.align {
        PopoverAlign::Start => "left-0",
        PopoverAlign::Center => "left-1/2 -translate-x-1/2",
        PopoverAlign::End => "right-0",
    };

    // Compute transform-origin based on side and align
    let origin_class = match (props.side, props.align) {
        (PopoverSide::Top, PopoverAlign::Start) => "origin-bottom-left",
        (PopoverSide::Top, PopoverAlign::Center) => "origin-bottom",
        (PopoverSide::Top, PopoverAlign::End) => "origin-bottom-right",
        (PopoverSide::Bottom, PopoverAlign::Start) => "origin-top-left",
        (PopoverSide::Bottom, PopoverAlign::Center) => "origin-top",
        (PopoverSide::Bottom, PopoverAlign::End) => "origin-top-right",
        (PopoverSide::Left, PopoverAlign::Start) => "origin-top-right",
        (PopoverSide::Left, PopoverAlign::Center) => "origin-right",
        (PopoverSide::Left, PopoverAlign::End) => "origin-bottom-right",
        (PopoverSide::Right, PopoverAlign::Start) => "origin-top-left",
        (PopoverSide::Right, PopoverAlign::Center) => "origin-left",
        (PopoverSide::Right, PopoverAlign::End) => "origin-bottom-left",
    };

    let classes = format!(
        "z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden \
         {} \
         data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
         data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
         data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
         data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 \
         absolute {} {} {}",
        origin_class, side_class, align_class, custom_class
    );

    let context_for_overlay = context.clone();
    rsx! {
        // Overlay to catch clicks outside
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                let mut ctx = context_for_overlay.clone();
                ctx.close();
            },
        }

        // Content panel
        div {
            role: "dialog",
            id: context.content_id.clone(),
            class: classes,
            style: "width: {width}; margin-top: {props.side_offset}px;",
            "data-slot": "popover-content",
            "data-state": "open",
            "data-side": match props.side {
                PopoverSide::Top => "top",
                PopoverSide::Right => "right",
                PopoverSide::Bottom => "bottom",
                PopoverSide::Left => "left",
            },
            "data-align": match props.align {
                PopoverAlign::Start => "start",
                PopoverAlign::Center => "center",
                PopoverAlign::End => "end",
            },
            aria_labelledby: format!("{}-title", context.content_id),
            aria_describedby: format!("{}-description", context.content_id),
            tabindex: "-1",

            {props.children}
        }
    }
}

/// Props for PopoverHeader.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section for popover content.
#[component]
pub fn PopoverHeader(props: PopoverHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex flex-col gap-1 text-sm {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "popover-header",
            {props.children}
        }
    }
}

/// Props for PopoverTitle.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for the popover.
#[component]
pub fn PopoverTitle(props: PopoverTitleProps) -> Element {
    let context = try_use_context::<PopoverContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("font-medium {}", custom_class);

    let id = context
        .map(|ctx| format!("{}-title", ctx.content_id))
        .unwrap_or_default();

    rsx! {
        h2 {
            id: id,
            class: classes,
            "data-slot": "popover-title",
            {props.children}
        }
    }
}

/// Props for PopoverDescription.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for the popover.
#[component]
pub fn PopoverDescription(props: PopoverDescriptionProps) -> Element {
    let context = try_use_context::<PopoverContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-muted-foreground {}", custom_class);

    let id = context
        .map(|ctx| format!("{}-description", ctx.content_id))
        .unwrap_or_default();

    rsx! {
        p {
            id: id,
            class: classes,
            "data-slot": "popover-description",
            {props.children}
        }
    }
}

/// Props for PopoverClose.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverCloseProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Close button content.
    pub children: Element,
}

/// A button that closes the popover.
#[component]
pub fn PopoverClose(props: PopoverCloseProps) -> Element {
    let context = use_context::<PopoverContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = {
        let mut context = context.clone();
        move |_| {
            context.close();
        }
    };

    rsx! {
        div {
            class: custom_class,
            "data-slot": "popover-close",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Hook to access the popover context.
pub fn use_popover() -> PopoverContext {
    use_context::<PopoverContext>()
}
