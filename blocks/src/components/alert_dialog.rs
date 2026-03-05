//! AlertDialog component for important confirmations.
//!
//! A modal dialog that interrupts the user with important content and expects
//! a response. Unlike regular dialogs, alert dialogs require explicit action
//! and cannot be dismissed by clicking outside.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing alert dialog state.
#[derive(Clone)]
pub struct AlertDialogContext {
    /// Whether the dialog is open.
    pub open: Signal<bool>,
    /// Size variant.
    pub size: AlertDialogSize,
    /// The content element ID.
    pub content_id: String,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
}

impl AlertDialogContext {
    /// Close the dialog.
    pub fn close(&mut self) {
        self.open.set(false);
        if let Some(callback) = &self.on_open_change {
            callback.call(false);
        }
    }

    /// Open the dialog.
    pub fn open_dialog(&mut self) {
        self.open.set(true);
        if let Some(callback) = &self.on_open_change {
            callback.call(true);
        }
    }
}

/// Size variants for the alert dialog.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AlertDialogSize {
    Sm,
    #[default]
    Default,
}

/// Props for AlertDialog.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    /// Controlled open state.
    #[props(default)]
    pub open: Option<Signal<bool>>,

    /// Default open state for uncontrolled mode.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Dialog content.
    pub children: Element,
}

/// An alert dialog for important confirmations.
///
/// # Example
///
/// ```rust
/// let open = use_signal(|| false);
///
/// rsx! {
///     AlertDialog {
///         open: open,
///         on_open_change: move |v| open.set(v),
///
///         AlertDialogTrigger {
///             Button { "Delete account" }
///         }
///         AlertDialogContent {
///             AlertDialogHeader {
///                 AlertDialogTitle { "Are you absolutely sure?" }
///                 AlertDialogDescription {
///                     "This action cannot be undone. This will permanently delete your account."
///                 }
///             }
///             AlertDialogFooter {
///                 AlertDialogCancel { "Cancel" }
///                 AlertDialogAction { "Continue" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let dialog_id = use_unique_id();
    let content_id = format!("alert-dialog-{}", dialog_id());

    // Internal state for uncontrolled mode
    let internal_open = use_signal(|| props.default_open);

    // Use controlled or internal state
    let open = props.open.unwrap_or(internal_open);

    let context = AlertDialogContext {
        open,
        size: AlertDialogSize::Default,
        content_id,
        on_open_change: props.on_open_change.clone(),
    };

    use_context_provider(|| context);

    rsx! {
        div {
            "data-slot": "alert-dialog",
            "data-state": if *open.read() { "open" } else { "closed" },

            {props.children}
        }
    }
}

/// Props for AlertDialogTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTriggerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content.
    pub children: Element,
}

/// The button that triggers the alert dialog.
#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    let context = use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = {
        let mut context = context.clone();
        move |_| {
            context.open_dialog();
        }
    };

    rsx! {
        div {
            class: custom_class,
            "data-slot": "alert-dialog-trigger",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for AlertDialogContent.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    /// Size variant.
    #[props(default)]
    pub size: AlertDialogSize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Dialog content.
    pub children: Element,
}

/// The main content area of the alert dialog.
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let context = use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    if !*context.open.read() {
        return rsx! {};
    }

    // Update context with size
    let context = context;
    // Note: In a real implementation we'd update the context

    let size_class = match props.size {
        AlertDialogSize::Sm => "max-w-xs",
        AlertDialogSize::Default => "max-w-lg",
    };

    let classes = format!(
        "group/alert-dialog-content fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] \
         translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background \
         p-6 shadow-lg duration-200 \
         data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
         data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 \
         data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-lg \
         {} {}",
        size_class, custom_class
    );

    // Handle escape key - alert dialogs should NOT close on escape by default
    // They require explicit action

    rsx! {
        // Overlay - does NOT close on click for alert dialogs
        div {
            class: "fixed inset-0 z-50 bg-black/50 \
                    data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
                    data-[state=open]:animate-in data-[state=open]:fade-in-0",
            "data-slot": "alert-dialog-overlay",
            "data-state": "open",
        }

        // Content panel
        div {
            role: "alertdialog",
            id: context.content_id.clone(),
            class: classes,
            "data-slot": "alert-dialog-content",
            "data-size": match props.size {
                AlertDialogSize::Sm => "sm",
                AlertDialogSize::Default => "default",
            },
            "data-state": "open",
            aria_modal: "true",
            aria_labelledby: format!("{}-title", context.content_id),
            aria_describedby: format!("{}-description", context.content_id),
            tabindex: "-1",

            {props.children}
        }
    }
}

/// Props for AlertDialogHeader.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section for alert dialog content.
#[component]
pub fn AlertDialogHeader(props: AlertDialogHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "grid grid-rows-[auto_1fr] place-items-center gap-1.5 text-center \
         has-data-[slot=alert-dialog-media]:grid-rows-[auto_auto_1fr] has-data-[slot=alert-dialog-media]:gap-x-6 \
         sm:group-data-[size=default]/alert-dialog-content:place-items-start \
         sm:group-data-[size=default]/alert-dialog-content:text-left \
         sm:group-data-[size=default]/alert-dialog-content:has-data-[slot=alert-dialog-media]:grid-rows-[auto_1fr] \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "alert-dialog-header",
            {props.children}
        }
    }
}

/// Props for AlertDialogFooter.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogFooterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content.
    pub children: Element,
}

/// Footer section for alert dialog actions.
#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-col-reverse gap-2 \
         group-data-[size=sm]/alert-dialog-content:grid group-data-[size=sm]/alert-dialog-content:grid-cols-2 \
         sm:flex-row sm:justify-end {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "alert-dialog-footer",
            {props.children}
        }
    }
}

/// Props for AlertDialogTitle.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for the alert dialog.
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let context = try_use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "text-lg font-semibold \
         sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2 \
         {}",
        custom_class
    );

    let id = context
        .map(|ctx| format!("{}-title", ctx.content_id))
        .unwrap_or_default();

    rsx! {
        h2 {
            id: id,
            class: classes,
            "data-slot": "alert-dialog-title",
            {props.children}
        }
    }
}

/// Props for AlertDialogDescription.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for the alert dialog.
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let context = try_use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-sm text-muted-foreground {}", custom_class);

    let id = context
        .map(|ctx| format!("{}-description", ctx.content_id))
        .unwrap_or_default();

    rsx! {
        p {
            id: id,
            class: classes,
            "data-slot": "alert-dialog-description",
            {props.children}
        }
    }
}

/// Props for AlertDialogMedia.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogMediaProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Media content (icon, image, etc.).
    pub children: Element,
}

/// Media section for alert dialog (icon or image).
#[component]
pub fn AlertDialogMedia(props: AlertDialogMediaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "mb-2 inline-flex size-16 items-center justify-center rounded-md bg-muted \
         sm:group-data-[size=default]/alert-dialog-content:row-span-2 \
         *:[svg:not([class*='size-'])]:size-8 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "alert-dialog-media",
            {props.children}
        }
    }
}

/// Button variant for actions.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AlertDialogButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Props for AlertDialogAction.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    /// Button variant.
    #[props(default)]
    pub variant: AlertDialogButtonVariant,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Button content.
    pub children: Element,
}

/// The action button that confirms and closes the alert dialog.
#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let context = use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        AlertDialogButtonVariant::Default => {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        }
        AlertDialogButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
        AlertDialogButtonVariant::Outline => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        AlertDialogButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }
        AlertDialogButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        AlertDialogButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let classes = format!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm \
         font-medium ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 \
         {} {}",
        variant_class, custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        let on_click = props.on_click.clone();
        move |_| {
            if let Some(callback) = &on_click {
                callback.call(());
            }
            context.close();
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "alert-dialog-action",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for AlertDialogCancel.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    /// Button variant.
    #[props(default = AlertDialogButtonVariant::Outline)]
    pub variant: AlertDialogButtonVariant,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Button content.
    pub children: Element,
}

/// The cancel button that closes the alert dialog without action.
#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let context = use_context::<AlertDialogContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        AlertDialogButtonVariant::Default => {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        }
        AlertDialogButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
        AlertDialogButtonVariant::Outline => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        AlertDialogButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }
        AlertDialogButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        AlertDialogButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };

    let classes = format!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm \
         font-medium ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 \
         {} {}",
        variant_class, custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        let on_click = props.on_click.clone();
        move |_| {
            if let Some(callback) = &on_click {
                callback.call(());
            }
            context.close();
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "alert-dialog-cancel",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Hook to access the alert dialog context.
pub fn use_alert_dialog() -> AlertDialogContext {
    use_context::<AlertDialogContext>()
}
