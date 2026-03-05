//! Tabs component for organizing content into switchable panels.

use crate::use_unique_id;
use dioxus::prelude::*;
use dioxus::prelude::Key;

/// Visual variant for tabs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TabsVariant {
    #[default]
    Default,
    /// Line variant with underline indicator.
    Line,
}

/// Orientation for tabs layout.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Context for managing tab state.
#[derive(Clone)]
struct TabsContext {
    value: Signal<String>,
    variant: TabsVariant,
    orientation: TabsOrientation,
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

    /// Visual variant.
    #[props(default)]
    pub variant: TabsVariant,

    /// Layout orientation.
    #[props(default)]
    pub orientation: TabsOrientation,

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
        variant: props.variant,
        orientation: props.orientation,
        on_value_change: props.on_value_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    let orientation_class = match props.orientation {
        TabsOrientation::Horizontal => "group/tabs flex gap-2 flex-col",
        TabsOrientation::Vertical => "group/tabs flex gap-2",
    };

    let classes = format!("{} {}", orientation_class, custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "tabs",
            "data-orientation": match props.orientation {
                TabsOrientation::Horizontal => "horizontal",
                TabsOrientation::Vertical => "vertical",
            },
            "data-variant": match props.variant {
                TabsVariant::Default => "default",
                TabsVariant::Line => "line",
            },
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
    let context = use_context::<TabsContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let base_classes = match context.variant {
        TabsVariant::Default => match context.orientation {
            TabsOrientation::Horizontal => {
                "group/tabs-list inline-flex w-fit items-center justify-center rounded-lg bg-muted p-[3px] text-muted-foreground h-9"
            }
            TabsOrientation::Vertical => {
                "group/tabs-list inline-flex w-fit items-center justify-center rounded-lg bg-muted p-[3px] text-muted-foreground h-fit flex-col"
            }
        },
        TabsVariant::Line => match context.orientation {
            TabsOrientation::Horizontal => {
                "group/tabs-list inline-flex w-fit items-center justify-center rounded-none bg-transparent p-[3px] text-muted-foreground h-9 gap-1"
            }
            TabsOrientation::Vertical => {
                "group/tabs-list inline-flex w-fit items-center justify-center rounded-none bg-transparent p-[3px] text-muted-foreground h-fit flex-col gap-1"
            }
        },
    };

    let classes = format!("{} {}", base_classes, custom_class);

    rsx! {
        div {
            class: classes,
            role: "tablist",
            "data-slot": "tabs-list",
            "data-orientation": match context.orientation {
                TabsOrientation::Horizontal => "horizontal",
                TabsOrientation::Vertical => "vertical",
            },
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
    let mut context = use_context::<TabsContext>();
    let is_active = *context.value.read() == props.value;

    let tab_id = use_unique_id();

    let custom_class = props.class.as_deref().unwrap_or("");

    let state_classes = if is_active {
        "data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm dark:data-[state=active]:border-input dark:data-[state=active]:bg-input/30"
    } else {
        "text-foreground/60 dark:text-muted-foreground"
    };

    let classes = format!(
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md \
         border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-all \
         hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring \
         disabled:pointer-events-none disabled:opacity-50 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {} {}",
        state_classes,
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
            if props.disabled {
                return;
            }
            let should_activate = match event.key() {
                Key::Enter => true,
                Key::Character(ref s) if s == " " => true,
                _ => false,
            };
            if should_activate {
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

    let classes = format!("flex-1 outline-none {}", custom_class);

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
