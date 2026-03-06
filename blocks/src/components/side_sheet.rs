use crate::use_unique_id;
use dioxus::document::eval;
use dioxus::prelude::Key;
use dioxus::prelude::*;
use lucide_dioxus::X;

// Side from which the sheet appears (matches shadcn sheet.tsx)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SideSheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SideSheetSide {
    fn content_classes(&self) -> &'static str {
        match self {
            SideSheetSide::Top => "inset-x-0 top-0 h-auto border-b",
            SideSheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
            SideSheetSide::Bottom => "inset-x-0 bottom-0 h-auto border-t",
            SideSheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
        }
    }

    fn open_animation(&self) -> &'static str {
        match self {
            SideSheetSide::Top => "data-[state=open]:slide-in-from-top",
            SideSheetSide::Right => "data-[state=open]:slide-in-from-right",
            SideSheetSide::Bottom => "data-[state=open]:slide-in-from-bottom",
            SideSheetSide::Left => "data-[state=open]:slide-in-from-left",
        }
    }

    fn close_animation(&self) -> &'static str {
        match self {
            SideSheetSide::Top => "data-[state=closed]:slide-out-to-top",
            SideSheetSide::Right => "data-[state=closed]:slide-out-to-right",
            SideSheetSide::Bottom => "data-[state=closed]:slide-out-to-bottom",
            SideSheetSide::Left => "data-[state=closed]:slide-out-to-left",
        }
    }
}

// Context for sharing state between side sheet components
#[derive(Clone)]
pub struct SideSheetContext {
    is_open: Signal<bool>,
    side: SideSheetSide,
}

// Main SideSheet component that provides context
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetProps {
    #[props(default = SideSheetSide::Right)]
    pub side: SideSheetSide,

    #[props(default = false)]
    pub default_open: bool,

    pub children: Element,
}

#[component]
pub fn SideSheet(props: SideSheetProps) -> Element {
    let is_open = use_signal(|| props.default_open);

    let context = SideSheetContext {
        is_open,
        side: props.side,
    };

    use_context_provider(|| context);

    rsx! {
        {props.children}
    }
}

// Trigger component to open the side sheet
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetTriggerProps {
    #[props(default)]
    pub class: Option<String>,

    pub children: Element,
}

#[component]
pub fn SideSheetTrigger(props: SideSheetTriggerProps) -> Element {
    let mut context = use_context::<SideSheetContext>();

    let on_click = move |_| {
        context.is_open.set(true);
    };

    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "w-auto inline-block {class}",
            "data-slot": "sheet-trigger",
            onclick: on_click,
            {props.children}
        }
    }
}

// Close trigger component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetCloseProps {
    pub children: Element,
}

#[component]
pub fn SideSheetClose(props: SideSheetCloseProps) -> Element {
    let mut context = use_context::<SideSheetContext>();

    let on_click = move |_| {
        context.is_open.set(false);
    };

    rsx! {
        div {
            "data-slot": "sheet-close",
            onclick: on_click,
            {props.children}
        }
    }
}

// Overlay component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetOverlayProps {
    #[props(default = "bg-black/80".to_string())]
    pub class: String,
}

#[component]
pub fn SideSheetOverlay(props: SideSheetOverlayProps) -> Element {
    let mut context = use_context::<SideSheetContext>();

    let on_click = move |_| {
        context.is_open.set(false);
    };

    let is_open = *context.is_open.read();

    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/50 \
                    data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
                    data-[state=open]:animate-in data-[state=open]:fade-in-0 \
                    data-[state=closed]:hidden {props.class}",
            "data-slot": "sheet-overlay",
            "data-state": if is_open { "open" } else { "closed" },
            onclick: on_click,
            aria_hidden: "true",
        }
    }
}

// Content component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetContentProps {
    #[props(default = "".to_string())]
    pub class: String,

    /// Optional ID for the content container
    #[props(default)]
    pub id: Option<String>,

    pub children: Element,
}

#[component]
pub fn SideSheetContent(props: SideSheetContentProps) -> Element {
    let mut context = use_context::<SideSheetContext>();
    let is_open = *context.is_open.read();

    // Generate unique ID for focus trap
    let content_id = use_unique_id();
    let id = props.id.clone().unwrap_or_else(|| content_id());
    let id_for_effect = id.clone();

    let side_classes = context.side.content_classes();
    let open_animation = context.side.open_animation();
    let close_animation = context.side.close_animation();

    // Set up focus trap when open
    use_effect(move || {
        if !is_open {
            return;
        }

        let id_clone = id_for_effect.clone();
        spawn(async move {
            // Focus the first focusable element in the sheet
            let focus_script = format!(
                r#"
                (function() {{
                    const container = document.getElementById('{id}');
                    if (!container) return;

                    const focusable = container.querySelectorAll(
                        'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
                    );
                    if (focusable.length > 0) {{
                        focusable[0].focus();
                    }}
                }})()
                "#,
                id = id_clone,
            );
            let _ = eval(&focus_script).await;
        });
    });

    // Handle Escape key to close
    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == Key::Escape {
            context.is_open.set(false);
        }
    };

    rsx! {
        // Portal-like behavior - render at the root level
        div {
            class: "fixed z-50",
            "data-slot": "sheet",
            "data-state": if is_open { "open" } else { "closed" },

            // Overlay
            SideSheetOverlay {}

            // Content (matches shadcn sheet.tsx)
            div {
                id: id,
                class: "fixed z-50 flex flex-col gap-4 bg-background shadow-lg transition ease-in-out \
                        data-[state=closed]:animate-out data-[state=closed]:duration-300 \
                        data-[state=open]:animate-in data-[state=open]:duration-500 \
                        {side_classes} {open_animation} {close_animation} {props.class}",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "side-sheet-title",
                aria_describedby: "side-sheet-description",
                "data-slot": "sheet-content",
                "data-state": if is_open { "open" } else { "closed" },
                onkeydown: handle_keydown,
                tabindex: "-1",

                {props.children}
            }
        }
    }
}

// Header component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetHeaderProps {
    #[props(default = "".to_string())]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn SideSheetHeader(props: SideSheetHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1.5 p-4 {props.class}",
            "data-slot": "sheet-header",
            {props.children}
        }
    }
}

// Title component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetTitleProps {
    #[props(default = "".to_string())]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn SideSheetTitle(props: SideSheetTitleProps) -> Element {
    rsx! {
        h2 {
            id: "side-sheet-title",
            class: "font-semibold text-foreground {props.class}",
            "data-slot": "sheet-title",
            {props.children}
        }
    }
}

// Description component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetDescriptionProps {
    #[props(default = "".to_string())]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn SideSheetDescription(props: SideSheetDescriptionProps) -> Element {
    rsx! {
        p {
            id: "side-sheet-description",
            class: "text-sm text-muted-foreground {props.class}",
            "data-slot": "sheet-description",
            {props.children}
        }
    }
}

// Body component for main content area
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetBodyProps {
    #[props(default = "".to_string())]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn SideSheetBody(props: SideSheetBodyProps) -> Element {
    rsx! {
        div {
            class: "flex-1 overflow-y-auto {props.class}",
            "data-slot": "sheet-body",
            {props.children}
        }
    }
}

// Footer component for action buttons
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetFooterProps {
    #[props(default = "".to_string())]
    pub class: String,

    pub children: Element,
}

#[component]
pub fn SideSheetFooter(props: SideSheetFooterProps) -> Element {
    rsx! {
        div {
            class: "mt-auto flex flex-col gap-2 p-4 {props.class}",
            "data-slot": "sheet-footer",
            {props.children}
        }
    }
}

// Default close button component for convenience
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetCloseButtonProps {
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn SideSheetCloseButton(props: SideSheetCloseButtonProps) -> Element {
    let mut context = use_context::<SideSheetContext>();

    rsx! {
        button {
            class: "absolute top-4 right-4 rounded-xs opacity-70 ring-offset-background transition-opacity \
                    hover:opacity-100 focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 \
                    disabled:pointer-events-none data-[state=open]:bg-secondary {props.class}",
            onclick: move |_| context.is_open.set(false),
            r#type: "button",
            aria_label: "Close",

            X {
                class: "size-4"
            }
            span {
                class: "sr-only",
                "Close"
            }
        }
    }
}
