//! Dialog component for modal interactions.
//!
//! A modal dialog component with overlay, focus management, and accessibility support.

use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogCtx};

/// The props for the [`Dialog`] component (wrapper for DialogRoot)
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The ID of the dialog root element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// A callback that is called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the dialog root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog root component.
    pub children: Element,
}

/// The root dialog component that manages open state.
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let class = [
        "fixed inset-0 z-50 flex items-center justify-center",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogRoot {
            id: props.id,
            class,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            "data-slot": "dialog",
            {props.children}
        }
    }
}

/// Alias for backward compatibility
pub use Dialog as DialogRoot;

/// The props for the [`DialogTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// The open state signal to toggle when clicked.
    pub open: Signal<bool>,

    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The trigger element (typically a button).
    pub children: Element,
}

/// A button or element that triggers the dialog to open.
///
/// Note: This is a standalone component that requires passing the open signal.
/// For use within a Dialog, you can also just use a regular button with onclick.
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    let mut open = props.open;

    rsx! {
        button {
            r#type: "button",
            class: props.class,
            onclick: move |_| open.set(true),
            "data-slot": "dialog-trigger",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogOverlay`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogOverlayProps {
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the overlay element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The overlay backdrop behind the dialog content.
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.is_open();

    let class = [
        "fixed inset-0 z-50 bg-black/50",
        "data-[state=closed]:animate-out data-[state=closed]:fade-out-0",
        "data-[state=open]:animate-in data-[state=open]:fade-in-0",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        div {
            class,
            "data-slot": "dialog-overlay",
            "data-state": if open { "open" } else { "closed" },
            ..props.attributes,
        }
    }
}

/// The props for the [`DialogContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    /// The ID of the dialog content element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Whether to show the close button in the top-right corner.
    #[props(default = true)]
    pub show_close_button: bool,

    /// Additional attributes to apply to the dialog content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog content.
    pub children: Element,
}

/// The main content area of the dialog.
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let ctx: DialogCtx = use_context();

    let class = [
        "relative z-50 grid w-full max-w-[calc(100%-2rem)]",
        "gap-4 rounded-lg border bg-background p-6 shadow-lg",
        "duration-200 outline-none",
        "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
        "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        "sm:max-w-lg",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogContent {
            id: props.id,
            class,
            attributes: props.attributes,
            "data-slot": "dialog-content",

            {props.children}

            if props.show_close_button {
                button {
                    r#type: "button",
                    class: "absolute top-4 right-4 rounded-xs opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                    "data-slot": "dialog-close",
                    onclick: move |_| ctx.set_open(false),

                    // X icon
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                    span {
                        class: "sr-only",
                        "Close"
                    }
                }
            }
        }
    }
}

/// The props for the [`DialogClose`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the close element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The close element content.
    pub children: Element,
}

/// A button that closes the dialog when clicked.
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let ctx: DialogCtx = use_context();

    rsx! {
        button {
            r#type: "button",
            class: props.class,
            onclick: move |_| ctx.set_open(false),
            "data-slot": "dialog-close",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogHeader`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the header element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog header.
    pub children: Element,
}

/// A container for the dialog title and description.
#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let class = [
        "flex flex-col gap-2 text-center sm:text-left",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        div {
            class,
            "data-slot": "dialog-header",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogFooter`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Whether to show a close button in the footer.
    #[props(default)]
    pub show_close_button: bool,

    /// Additional attributes to apply to the footer element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog footer.
    pub children: Element,
}

/// A container for dialog actions (buttons).
#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let ctx: DialogCtx = use_context();

    let class = [
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        div {
            class,
            "data-slot": "dialog-footer",
            ..props.attributes,

            {props.children}

            if props.show_close_button {
                button {
                    r#type: "button",
                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2",
                    onclick: move |_| ctx.set_open(false),
                    "Close"
                }
            }
        }
    }
}

/// The props for the [`DialogTitle`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    /// The ID of the dialog title element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the dialog title element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog title.
    pub children: Element,
}

/// The title of the dialog.
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let class = [
        "text-lg leading-none font-semibold",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            class,
            attributes: props.attributes,
            "data-slot": "dialog-title",
            {props.children}
        }
    }
}

/// The props for the [`DialogDescription`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    /// The ID of the dialog description element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Extra classes
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes for the dialog description element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog description.
    pub children: Element,
}

/// The description text of the dialog.
#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let class = [
        "text-sm text-muted-foreground",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            class,
            attributes: props.attributes,
            "data-slot": "dialog-description",
            {props.children}
        }
    }
}

/// Hook to access the dialog context.
pub fn use_dialog() -> DialogCtx {
    use_context::<DialogCtx>()
}
