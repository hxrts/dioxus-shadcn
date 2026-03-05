//! Tabs component for organizing content into switchable panels.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing tab state.
#[derive(Clone)]
struct TabsContext {
    value: Signal<String>,
    on_value_change: Option<Callback<String>>,
}

/// Props for the Tabs component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The controlled value of the active tab.
    #[props(default)]
    pub value: Option<Signal<String>>,

    /// The default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Callback when the active tab changes.
    #[props(default)]
    pub on_value_change: Option<Callback<String>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Tab content (TabsList and TabsContent).
    pub children: Element,
}

/// Container for a set of tabs and their content panels.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Tabs {
///         default_value: "account",
///
///         TabsList {
///             TabsTrigger { value: "account", "Account" }
///             TabsTrigger { value: "password", "Password" }
///         }
///
///         TabsContent { value: "account",
///             p { "Account settings here." }
///         }
///         TabsContent { value: "password",
///             p { "Password settings here." }
///         }
///     }
/// }
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let default = props.default_value.clone().unwrap_or_default();
    let internal_value = use_signal(|| default);

    let value = props.value.unwrap_or(internal_value);

    let context = TabsContext {
        value,
        on_value_change: props.on_value_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: custom_class,
            "data-slot": "tabs",
            {props.children}
        }
    }
}

/// Props for TabsList.
#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Tab triggers.
    pub children: Element,
}

/// Container for tab triggers.
#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            role: "tablist",
            "data-slot": "tabs-list",
            {props.children}
        }
    }
}

/// Props for TabsTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    /// The value that identifies this tab.
    pub value: String,

    /// Whether the tab is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Tab label content.
    pub children: Element,
}

/// A clickable tab trigger that switches the active panel.
#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let context = use_context::<TabsContext>();
    let is_active = *context.value.read() == props.value;

    let tab_id = use_unique_id();

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 \
         text-sm font-medium ring-offset-background transition-all \
         focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 \
         {} {}",
        if is_active {
            "bg-background text-foreground shadow-sm"
        } else {
            "hover:bg-background/50 hover:text-foreground"
        },
        custom_class
    );

    let handle_click = {
        let value = props.value.clone();
        let on_change = context.on_value_change.clone();
        move |_| {
            if !props.disabled {
                context.value.set(value.clone());
                if let Some(callback) = &on_change {
                    callback.call(value.clone());
                }
            }
        }
    };

    let handle_keydown = {
        let value = props.value.clone();
        let on_change = context.on_value_change.clone();
        move |event: KeyboardEvent| {
            if !props.disabled && (event.key() == "Enter" || event.key() == " ") {
                event.prevent_default();
                context.value.set(value.clone());
                if let Some(callback) = &on_change {
                    callback.call(value.clone());
                }
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "tab",
            id: tab_id,
            class: classes,
            "data-slot": "tabs-trigger",
            "data-state": if is_active { "active" } else { "inactive" },
            "data-value": props.value.clone(),
            aria_selected: is_active.to_string(),
            aria_controls: format!("panel-{}", props.value),
            disabled: props.disabled,
            tabindex: if is_active { "0" } else { "-1" },
            onclick: handle_click,
            onkeydown: handle_keydown,
            {props.children}
        }
    }
}

/// Props for TabsContent.
#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    /// The value that identifies which tab this content belongs to.
    pub value: String,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Panel content.
    pub children: Element,
}

/// Content panel that is shown when its corresponding tab is active.
#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let context = use_context::<TabsContext>();
    let is_active = *context.value.read() == props.value;

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "mt-2 ring-offset-background focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {}",
        custom_class
    );

    if !is_active {
        return rsx! {};
    }

    rsx! {
        div {
            role: "tabpanel",
            id: "panel-{props.value}",
            class: classes,
            "data-slot": "tabs-content",
            "data-state": "active",
            "data-value": props.value.clone(),
            tabindex: "0",
            {props.children}
        }
    }
}
