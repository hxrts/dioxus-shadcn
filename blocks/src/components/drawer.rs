//! Drawer component for slide-out panels.
//!
//! A drawer component that slides in from any edge of the screen,
//! similar to a sheet but with gesture support patterns.

use dioxus::prelude::*;
use lucide_dioxus::X;

/// Direction from which the drawer appears.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DrawerDirection {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl DrawerDirection {
    fn content_classes(&self) -> &'static str {
        match self {
            DrawerDirection::Top => "inset-x-0 top-0 mb-24 max-h-[80vh] rounded-b-lg border-b",
            DrawerDirection::Bottom => "inset-x-0 bottom-0 mt-24 max-h-[80vh] rounded-t-lg border-t",
            DrawerDirection::Left => "inset-y-0 left-0 mr-24 h-full w-3/4 max-w-sm rounded-r-lg border-r",
            DrawerDirection::Right => "inset-y-0 right-0 ml-24 h-full w-3/4 max-w-sm rounded-l-lg border-l",
        }
    }

    fn open_animation(&self) -> &'static str {
        match self {
            DrawerDirection::Top => "data-[state=open]:slide-in-from-top",
            DrawerDirection::Bottom => "data-[state=open]:slide-in-from-bottom",
            DrawerDirection::Left => "data-[state=open]:slide-in-from-left",
            DrawerDirection::Right => "data-[state=open]:slide-in-from-right",
        }
    }

    fn close_animation(&self) -> &'static str {
        match self {
            DrawerDirection::Top => "data-[state=closed]:slide-out-to-top",
            DrawerDirection::Bottom => "data-[state=closed]:slide-out-to-bottom",
            DrawerDirection::Left => "data-[state=closed]:slide-out-to-left",
            DrawerDirection::Right => "data-[state=closed]:slide-out-to-right",
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            DrawerDirection::Top => "top",
            DrawerDirection::Bottom => "bottom",
            DrawerDirection::Left => "left",
            DrawerDirection::Right => "right",
        }
    }
}

/// Context for drawer state.
#[derive(Clone)]
pub struct DrawerContext {
    pub open: Signal<bool>,
    pub direction: DrawerDirection,
}

impl DrawerContext {
    pub fn close(&mut self) {
        self.open.set(false);
    }

    pub fn toggle(&mut self) {
        let current = *self.open.read();
        self.open.set(!current);
    }
}

/// Props for Drawer.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    /// Controlled open state.
    #[props(default)]
    pub open: Option<Signal<bool>>,

    /// Default open state for uncontrolled mode.
    #[props(default)]
    pub default_open: bool,

    /// Direction from which the drawer appears.
    #[props(default)]
    pub direction: DrawerDirection,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Drawer content.
    pub children: Element,
}

/// A drawer component for slide-out panels.
///
/// # Example
///
/// ```rust
/// let open = use_signal(|| false);
///
/// rsx! {
///     Drawer {
///         open: open,
///         direction: DrawerDirection::Bottom,
///
///         DrawerTrigger {
///             Button { "Open Drawer" }
///         }
///         DrawerContent {
///             DrawerHeader {
///                 DrawerTitle { "Drawer Title" }
///                 DrawerDescription { "Drawer description text." }
///             }
///             div { class: "p-4",
///                 "Drawer content goes here."
///             }
///             DrawerFooter {
///                 Button { "Close" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    // Internal state for uncontrolled mode
    let internal_open = use_signal(|| props.default_open);

    // Use controlled or internal state
    let open = props.open.unwrap_or(internal_open);

    let context = DrawerContext {
        open,
        direction: props.direction,
    };

    use_context_provider(|| context);

    rsx! {
        div {
            "data-slot": "drawer",
            "data-state": if *open.read() { "open" } else { "closed" },
            "data-direction": props.direction.as_str(),
            {props.children}
        }
    }
}

/// Props for DrawerTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerTriggerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content.
    pub children: Element,
}

/// A trigger button that opens the drawer.
#[component]
pub fn DrawerTrigger(props: DrawerTriggerProps) -> Element {
    let mut context = use_context::<DrawerContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = move |_| {
        context.open.set(true);
    };

    rsx! {
        div {
            class: "inline-block {custom_class}",
            "data-slot": "drawer-trigger",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for DrawerClose.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerCloseProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Close button content.
    pub children: Element,
}

/// A button that closes the drawer.
#[component]
pub fn DrawerClose(props: DrawerCloseProps) -> Element {
    let mut context = use_context::<DrawerContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = move |_| {
        context.close();
    };

    rsx! {
        div {
            class: custom_class,
            "data-slot": "drawer-close",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for DrawerOverlay.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerOverlayProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// The overlay backdrop for the drawer.
#[component]
pub fn DrawerOverlay(props: DrawerOverlayProps) -> Element {
    let mut context = use_context::<DrawerContext>();
    let custom_class = props.class.as_deref().unwrap_or("");
    let is_open = *context.open.read();

    let handle_click = move |_| {
        context.close();
    };

    let classes = format!(
        "fixed inset-0 z-50 bg-black/50 \
         data-[state=open]:animate-in data-[state=open]:fade-in-0 \
         data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "drawer-overlay",
            "data-state": if is_open { "open" } else { "closed" },
            onclick: handle_click,
            "aria-hidden": "true",
        }
    }
}

/// Props for DrawerContent.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerContentProps {
    /// Whether to show the close button.
    #[props(default = true)]
    pub show_close: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Drawer content.
    pub children: Element,
}

/// The main content panel of the drawer.
#[component]
pub fn DrawerContent(props: DrawerContentProps) -> Element {
    let context = use_context::<DrawerContext>();
    let custom_class = props.class.as_deref().unwrap_or("");
    let is_open = *context.open.read();

    if !is_open {
        return rsx! {};
    }

    let direction_class = context.direction.content_classes();
    let open_animation = context.direction.open_animation();
    let close_animation = context.direction.close_animation();

    let classes = format!(
        "fixed z-50 flex flex-col gap-4 bg-background shadow-lg \
         data-[state=open]:animate-in data-[state=closed]:animate-out \
         data-[state=open]:duration-300 data-[state=closed]:duration-200 \
         {} {} {} {}",
        direction_class, open_animation, close_animation, custom_class
    );

    // Clone context for closures
    let direction = context.direction;
    let mut keydown_context = context.clone();
    let mut close_context = context.clone();

    // Handle escape key
    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == Key::Escape {
            keydown_context.close();
        }
    };

    rsx! {
        // Portal wrapper
        div {
            class: "fixed inset-0 z-50",
            "data-slot": "drawer-portal",

            DrawerOverlay {}

            div {
                class: classes,
                "data-slot": "drawer-content",
                "data-state": "open",
                "data-direction": direction.as_str(),
                role: "dialog",
                "aria-modal": "true",
                tabindex: "-1",
                onkeydown: handle_keydown,

                // Handle bar for bottom/top drawers
                if matches!(direction, DrawerDirection::Bottom | DrawerDirection::Top) {
                    div {
                        class: "mx-auto mt-4 h-1.5 w-12 shrink-0 rounded-full bg-muted",
                        "data-slot": "drawer-handle",
                    }
                }

                {props.children}

                // Close button
                if props.show_close {
                    button {
                        class: "absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background \
                                transition-opacity hover:opacity-100 \
                                focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
                                disabled:pointer-events-none",
                        "data-slot": "drawer-close-button",
                        onclick: move |_| close_context.close(),
                        "aria-label": "Close",

                        X { class: "size-4" }
                    }
                }
            }
        }
    }
}

/// Props for DrawerHeader.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section for the drawer.
#[component]
pub fn DrawerHeader(props: DrawerHeaderProps) -> Element {
    let context = use_context::<DrawerContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let text_align = match context.direction {
        DrawerDirection::Bottom | DrawerDirection::Top => "text-center",
        DrawerDirection::Left | DrawerDirection::Right => "text-left",
    };

    let classes = format!(
        "flex flex-col gap-1.5 p-4 {} {}",
        text_align, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "drawer-header",
            {props.children}
        }
    }
}

/// Props for DrawerFooter.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerFooterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content.
    pub children: Element,
}

/// Footer section for the drawer.
#[component]
pub fn DrawerFooter(props: DrawerFooterProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "mt-auto flex flex-col gap-2 p-4 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "drawer-footer",
            {props.children}
        }
    }
}

/// Props for DrawerTitle.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for the drawer.
#[component]
pub fn DrawerTitle(props: DrawerTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "font-semibold text-foreground {}",
        custom_class
    );

    rsx! {
        h2 {
            class: classes,
            "data-slot": "drawer-title",
            {props.children}
        }
    }
}

/// Props for DrawerDescription.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for the drawer.
#[component]
pub fn DrawerDescription(props: DrawerDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "text-sm text-muted-foreground {}",
        custom_class
    );

    rsx! {
        p {
            class: classes,
            "data-slot": "drawer-description",
            {props.children}
        }
    }
}

/// Hook to access the drawer context.
pub fn use_drawer() -> DrawerContext {
    use_context::<DrawerContext>()
}
