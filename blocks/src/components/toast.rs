//! Toast notification component matching shadcn-ui/Sonner patterns.
//!
//! Provides toast notifications with actions, auto-dismiss, and keyboard support.

use crate::components::button::{Button, ButtonVariant};
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_sdk_time::use_timeout;
use lucide_dioxus::{Check, Info, LoaderCircle, OctagonX, TriangleAlert, X};
use std::time::Duration;

/// Toast types for different visual styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

impl ToastType {
    fn icon_component(&self) -> Element {
        match self {
            ToastType::Success => rsx! { Check { class: "size-4" } },
            ToastType::Error => rsx! { OctagonX { class: "size-4" } },
            ToastType::Warning => rsx! { TriangleAlert { class: "size-4" } },
            ToastType::Info => rsx! { Info { class: "size-4" } },
            ToastType::Loading => rsx! { LoaderCircle { class: "size-4 animate-spin" } },
        }
    }

    fn icon_classes(&self) -> &'static str {
        match self {
            ToastType::Success => "text-green-600 dark:text-green-400",
            ToastType::Error => "text-red-600 dark:text-red-400",
            ToastType::Warning => "text-yellow-600 dark:text-yellow-400",
            ToastType::Info => "text-foreground",
            ToastType::Loading => "text-muted-foreground",
        }
    }
}

/// An action button for a toast.
#[derive(Clone)]
pub struct ToastAction {
    /// Button label text.
    pub label: String,
    /// Callback when clicked.
    pub on_click: Callback<()>,
    /// Optional variant for styling.
    pub variant: Option<ButtonVariant>,
}

impl std::fmt::Debug for ToastAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToastAction")
            .field("label", &self.label)
            .field("variant", &self.variant)
            .finish_non_exhaustive()
    }
}

impl ToastAction {
    /// Create a new toast action.
    pub fn new(label: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            label: label.into(),
            on_click: Callback::new(move |_| on_click()),
            variant: None,
        }
    }

    /// Set the button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = Some(variant);
        self
    }
}

/// A single toast item.
#[derive(Debug, Clone)]
pub struct ToastItem {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub toast_type: ToastType,
    pub duration: Option<Duration>,
    pub permanent: bool,
    pub visible: bool,
    pub action: Option<ToastAction>,
    pub on_dismiss: Option<Callback<()>>,
}

impl PartialEq for ToastItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.description == other.description
            && self.toast_type == other.toast_type
            && self.duration == other.duration
            && self.permanent == other.permanent
            && self.visible == other.visible
    }
}

/// Context for toast management.
/// This is provided by ToastProvider and consumed by use_toast().
#[derive(Clone)]
pub struct ToastContext {
    toasts: Signal<Vec<ToastItem>>,
    next_id: Signal<usize>,
    default_duration: Duration,
    max_toasts: usize,
}

impl ToastContext {
    /// Show a toast with the given type and options.
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) -> usize {
        let mut next_id = self.next_id;
        let id = *next_id.read();
        next_id.set(id + 1);

        let toast = ToastItem {
            id,
            title,
            description: options.description,
            toast_type,
            duration: if options.permanent {
                None
            } else {
                options.duration.or(Some(self.default_duration))
            },
            permanent: options.permanent,
            visible: true,
            action: options.action,
            on_dismiss: options.on_dismiss,
        };

        let mut toasts = self.toasts;
        toasts.with_mut(|t| {
            t.push(toast);

            // Limit the number of toasts
            while t.len() > self.max_toasts {
                // Try to remove non-permanent toasts first
                if let Some(pos) = t.iter().position(|toast| !toast.permanent) {
                    t.remove(pos);
                } else {
                    t.remove(0);
                }
            }
        });

        id
    }

    /// Remove a toast by ID.
    pub fn remove(&self, id: usize) {
        let mut toasts = self.toasts;
        toasts.with_mut(|t| {
            if let Some(pos) = t.iter().position(|toast| toast.id == id) {
                // Call on_dismiss callback if set
                if let Some(callback) = &t[pos].on_dismiss {
                    callback.call(());
                }
                t.remove(pos);
            }
        });
    }

    /// Dismiss all toasts.
    pub fn dismiss_all(&self) {
        let mut toasts = self.toasts;
        toasts.with_mut(|t| {
            // Call on_dismiss for each toast
            for toast in t.iter() {
                if let Some(callback) = &toast.on_dismiss {
                    callback.call(());
                }
            }
            t.clear();
        });
    }

    /// Update an existing toast.
    pub fn update(&self, id: usize, options: ToastUpdateOptions) {
        let mut toasts = self.toasts;
        toasts.with_mut(|t| {
            if let Some(toast) = t.iter_mut().find(|toast| toast.id == id) {
                if let Some(title) = options.title {
                    toast.title = title;
                }
                if let Some(description) = options.description {
                    toast.description = Some(description);
                }
                if let Some(toast_type) = options.toast_type {
                    toast.toast_type = toast_type;
                }
            }
        });
    }

    /// Show a success toast.
    pub fn success(&self, title: impl Into<String>, options: Option<ToastOptions>) -> usize {
        self.show(
            title.into(),
            ToastType::Success,
            options.unwrap_or_default(),
        )
    }

    /// Show an error toast.
    pub fn error(&self, title: impl Into<String>, options: Option<ToastOptions>) -> usize {
        self.show(title.into(), ToastType::Error, options.unwrap_or_default())
    }

    /// Show a warning toast.
    pub fn warning(&self, title: impl Into<String>, options: Option<ToastOptions>) -> usize {
        self.show(
            title.into(),
            ToastType::Warning,
            options.unwrap_or_default(),
        )
    }

    /// Show an info toast.
    pub fn info(&self, title: impl Into<String>, options: Option<ToastOptions>) -> usize {
        self.show(title.into(), ToastType::Info, options.unwrap_or_default())
    }

    /// Show a loading toast.
    pub fn loading(&self, title: impl Into<String>, options: Option<ToastOptions>) -> usize {
        self.show(
            title.into(),
            ToastType::Loading,
            options.unwrap_or_default().permanent(),
        )
    }
}

/// Toast provider props.
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    /// Default duration for toasts (default: 5 seconds).
    #[props(default = Duration::from_secs(5))]
    pub default_duration: Duration,

    /// Maximum number of toasts to display (default: 10).
    #[props(default = 10)]
    pub max_toasts: usize,

    /// Position of the toast container.
    #[props(default = ToastPosition::TopRight)]
    pub position: ToastPosition,

    pub children: Element,
}

/// Position for the toast container.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
    TopCenter,
    BottomRight,
    BottomLeft,
    BottomCenter,
}

impl ToastPosition {
    fn classes(&self) -> &'static str {
        match self {
            ToastPosition::TopRight => "top-4 right-0 items-end",
            ToastPosition::TopLeft => "top-4 left-0 items-start",
            ToastPosition::TopCenter => "top-4 left-1/2 -translate-x-1/2 items-center",
            ToastPosition::BottomRight => "bottom-4 right-0 items-end",
            ToastPosition::BottomLeft => "bottom-4 left-0 items-start",
            ToastPosition::BottomCenter => "bottom-4 left-1/2 -translate-x-1/2 items-center",
        }
    }
}

/// Toast provider component.
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    let toasts = use_signal(Vec::new);
    let next_id = use_signal(|| 0usize);

    let context = ToastContext {
        toasts,
        next_id,
        default_duration: props.default_duration,
        max_toasts: props.max_toasts,
    };

    use_context_provider(|| context.clone());

    let position_classes = props.position.classes();

    rsx! {
        // Render children
        {props.children}

        // Toast container - fixed position overlay
        div {
            class: "fixed z-50 flex flex-col gap-2 w-full max-w-sm px-4 pointer-events-none {position_classes}",
            "data-slot": "toaster",
            aria_live: "polite",
            aria_atomic: "true",

            for toast in toasts.read().iter() {
                Toast {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    default_duration: props.default_duration,
                }
            }
        }
    }
}

/// Toast props.
#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    pub toast: ToastItem,
    pub default_duration: Duration,
}

/// Toast component.
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let context = use_context::<ToastContext>();
    let toast = props.toast.clone();
    let id = toast.id;
    let mut visible = use_signal(|| true);

    // Handle removing toast from the list
    let remove_toast = move || {
        context.remove(id);
    };

    // Handle starting exit animation
    let start_exit = move |_| {
        visible.set(false);
    };

    // Handle Escape key to dismiss
    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == Key::Escape {
            visible.set(false);
        }
    };

    // Set up auto-dismiss timer if not permanent
    if !toast.permanent {
        let duration = toast.duration.unwrap_or(props.default_duration);

        use_effect(move || {
            let timer = use_timeout(duration, move |()| {
                visible.set(false);
            });
            timer.action(());
        });
    }

    // Base styling using CSS variables for theme integration
    let base_classes = "pointer-events-auto relative flex w-full items-center justify-between gap-4 overflow-hidden rounded-md border p-4 shadow-md hover:shadow-lg transition-all duration-300 group bg-[var(--normal-bg,hsl(var(--popover)))] text-[var(--normal-text,hsl(var(--popover-foreground)))] border-[var(--normal-border,hsl(var(--border)))]";

    // Animation classes based on state
    let animation_classes = if !*visible.read() {
        "animate-out fade-out-0 slide-out-to-right-full"
    } else {
        "animate-in fade-in-0 slide-in-from-right-full"
    };

    // Combined classes
    let combined_classes = format!("{} {}", base_classes, animation_classes);

    rsx! {
        div {
            role: "alert",
            class: "{combined_classes}",
            "data-slot": "toast",
            "data-state": if *visible.read() { "visible" } else { "hidden" },
            "data-type": match toast.toast_type {
                ToastType::Success => "success",
                ToastType::Error => "error",
                ToastType::Warning => "warning",
                ToastType::Info => "info",
                ToastType::Loading => "loading",
            },
            tabindex: "0",
            aria_labelledby: "toast-title-{toast.id}",
            aria_describedby: if toast.description.is_some() {
                Some(format!("toast-desc-{}", toast.id))
            } else { None },
            onkeydown: handle_keydown,
            onanimationend: move |_| {
                if !*visible.read() {
                    remove_toast();
                }
            },

            div {
                class: "flex items-start gap-3 flex-1",

                // Icon
                div {
                    class: "flex-shrink-0 {toast.toast_type.icon_classes()}",
                    "data-slot": "toast-icon",
                    aria_hidden: "true",
                    {toast.toast_type.icon_component()}
                }

                // Content
                div {
                    class: "flex-1 space-y-1",

                    div {
                        class: "text-sm font-semibold leading-none tracking-tight",
                        "data-slot": "toast-title",
                        id: "toast-title-{toast.id}",
                        "{toast.title}"
                    }

                    if let Some(description) = &toast.description {
                        div {
                            class: "text-sm text-muted-foreground",
                            "data-slot": "toast-description",
                            id: "toast-desc-{toast.id}",
                            "{description}"
                        }
                    }
                }
            }

            // Action button (if provided)
            if let Some(action) = &toast.action {
                Button {
                    variant: action.variant.unwrap_or(ButtonVariant::Outline),
                    class: "shrink-0 h-8 text-xs",
                    on_click: {
                        let callback = action.on_click.clone();
                        move |_| callback.call(())
                    },
                    "{action.label}"
                }
            }

            // Close button
            Button {
                variant: ButtonVariant::Ghost,
                is_icon_button: true,
                aria_label: Some("Close".to_string()),
                on_click: start_exit,
                class: "absolute right-2 top-2 size-6 opacity-0 group-hover:opacity-100 transition-opacity",
                X { class: "size-3" }
            }
        }
    }
}

/// Toast options struct for easier API.
#[derive(Clone, Default)]
pub struct ToastOptions {
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub permanent: bool,
    pub action: Option<ToastAction>,
    pub on_dismiss: Option<Callback<()>>,
}

impl ToastOptions {
    /// Create options with a description.
    pub fn with_description(description: impl Into<String>) -> Self {
        Self {
            description: Some(description.into()),
            ..Default::default()
        }
    }

    /// Set the duration.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Make the toast permanent (no auto-dismiss).
    pub fn permanent(mut self) -> Self {
        self.permanent = true;
        self
    }

    /// Add an action button.
    pub fn action(mut self, action: ToastAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Add an on_dismiss callback.
    pub fn on_dismiss(mut self, callback: impl Fn() + 'static) -> Self {
        self.on_dismiss = Some(Callback::new(move |_| callback()));
        self
    }
}

/// Options for updating an existing toast.
#[derive(Clone, Default)]
pub struct ToastUpdateOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub toast_type: Option<ToastType>,
}

impl ToastUpdateOptions {
    /// Set a new title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set a new description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set a new toast type.
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = Some(toast_type);
        self
    }
}

/// Hook to use the toast API.
///
/// Must be called within a ToastProvider.
///
/// # Example
///
/// ```rust
/// let toast = use_toast();
///
/// // Show a simple success toast
/// toast.success("Operation completed!", None);
///
/// // Show a toast with description
/// toast.error("Failed to save", Some(ToastOptions::with_description("Please try again")));
///
/// // Show a loading toast and update it
/// let id = toast.loading("Processing...", None);
/// // Later...
/// toast.update(id, ToastUpdateOptions::default()
///     .title("Done!")
///     .toast_type(ToastType::Success));
///
/// // Show a toast with action
/// toast.info("File deleted", Some(ToastOptions::with_description("The file was moved to trash")
///     .action(ToastAction::new("Undo", || println!("Undo clicked!")))));
///
/// // Dismiss all toasts
/// toast.dismiss_all();
/// ```
pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>()
}

// ============================================================================
// Legacy API support (deprecated)
// ============================================================================

/// Legacy toast API using global signals.
///
/// **Deprecated**: Use `use_toast()` within a `ToastProvider` instead.
/// This is kept for backward compatibility but may cause issues with SSR.
#[deprecated(
    since = "0.4.0",
    note = "Use use_toast() within a ToastProvider instead"
)]
#[derive(Clone, Copy)]
pub struct Toasts;

// Global signals for legacy API
static LEGACY_TOASTS: GlobalSignal<Vec<ToastItem>> = Signal::global(Vec::new);
static LEGACY_NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

#[allow(deprecated)]
impl Toasts {
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        let mut next_id = LEGACY_NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let toast = ToastItem {
            id,
            title,
            description: options.description,
            toast_type,
            duration: if options.permanent {
                None
            } else {
                options.duration
            },
            permanent: options.permanent,
            visible: true,
            action: options.action,
            on_dismiss: options.on_dismiss,
        };

        let mut toasts = LEGACY_TOASTS.write();
        toasts.push(toast);

        while toasts.len() > 10 {
            if let Some(pos) = toasts.iter().position(|t| !t.permanent) {
                toasts.remove(pos);
            } else {
                toasts.remove(0);
            }
        }
    }

    pub fn success(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Success, options.unwrap_or_default());
    }

    pub fn error(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Error, options.unwrap_or_default());
    }

    pub fn warning(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Warning, options.unwrap_or_default());
    }

    pub fn info(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Info, options.unwrap_or_default());
    }
}
