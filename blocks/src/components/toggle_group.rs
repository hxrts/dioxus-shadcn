//! ToggleGroup component for grouped toggles.
//!
//! A set of two-state buttons that can be toggled on or off.

use dioxus::prelude::*;
use super::toggle::{ToggleVariant, ToggleSize};

/// Context for managing toggle group state.
#[derive(Clone)]
pub struct ToggleGroupContext {
    /// The type of selection (single or multiple).
    pub toggle_type: ToggleGroupType,
    /// Currently selected values.
    pub value: Signal<Vec<String>>,
    /// Visual variant for all items.
    pub variant: ToggleVariant,
    /// Size for all items.
    pub size: ToggleSize,
    /// Whether all items are disabled.
    pub disabled: bool,
    /// Callback when value changes.
    pub on_value_change: Option<Callback<Vec<String>>>,
}

impl ToggleGroupContext {
    /// Toggle a value.
    pub fn toggle(&mut self, item_value: String) {
        let mut current = self.value.read().clone();

        match self.toggle_type {
            ToggleGroupType::Single => {
                if current.contains(&item_value) {
                    current.clear();
                } else {
                    current = vec![item_value];
                }
            }
            ToggleGroupType::Multiple => {
                if let Some(pos) = current.iter().position(|v| v == &item_value) {
                    current.remove(pos);
                } else {
                    current.push(item_value);
                }
            }
        }

        self.value.set(current.clone());
        if let Some(callback) = &self.on_value_change {
            callback.call(current);
        }
    }

    /// Check if a value is selected.
    pub fn is_selected(&self, item_value: &str) -> bool {
        self.value.read().contains(&item_value.to_string())
    }
}

/// Type of selection for the toggle group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

/// Props for ToggleGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Type of selection.
    #[props(default)]
    pub toggle_type: ToggleGroupType,

    /// Controlled value (single or multiple).
    #[props(default)]
    pub value: Option<Signal<Vec<String>>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<Vec<String>>,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Option<Callback<Vec<String>>>,

    /// Visual variant for all items.
    #[props(default)]
    pub variant: ToggleVariant,

    /// Size for all items.
    #[props(default)]
    pub size: ToggleSize,

    /// Whether all items are disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Toggle items.
    pub children: Element,
}

/// A group of toggle buttons.
///
/// # Example
///
/// ```rust
/// let alignment = use_signal(|| vec!["left".to_string()]);
///
/// rsx! {
///     ToggleGroup {
///         toggle_type: ToggleGroupType::Single,
///         value: alignment,
///         on_value_change: move |v| alignment.set(v),
///
///         ToggleGroupItem { value: "left", /* Icon */ }
///         ToggleGroupItem { value: "center", /* Icon */ }
///         ToggleGroupItem { value: "right", /* Icon */ }
///     }
/// }
/// ```
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    // Internal state for uncontrolled mode
    let internal_value = use_signal(|| props.default_value.clone().unwrap_or_default());

    // Use controlled or internal state
    let value = props.value.unwrap_or(internal_value);

    let context = ToggleGroupContext {
        toggle_type: props.toggle_type,
        value,
        variant: props.variant,
        size: props.size,
        disabled: props.disabled,
        on_value_change: props.on_value_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/toggle-group flex w-fit items-center rounded-md {}",
        custom_class
    );

    rsx! {
        div {
            role: "group",
            class: classes,
            "data-slot": "toggle-group",
            "data-variant": match props.variant {
                ToggleVariant::Default => "default",
                ToggleVariant::Outline => "outline",
            },
            "data-size": match props.size {
                ToggleSize::Sm => "sm",
                ToggleSize::Default => "default",
                ToggleSize::Lg => "lg",
            },
            "data-type": match props.toggle_type {
                ToggleGroupType::Single => "single",
                ToggleGroupType::Multiple => "multiple",
            },
            {props.children}
        }
    }
}

/// Props for ToggleGroupItem.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
    /// The value of this item.
    pub value: String,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content (usually an icon).
    pub children: Element,
}

/// A toggle item within a ToggleGroup.
#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let context = use_context::<ToggleGroupContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let is_pressed = context.is_selected(&props.value);
    let is_disabled = props.disabled || context.disabled;

    let variant_class = match context.variant {
        ToggleVariant::Default => "bg-transparent",
        ToggleVariant::Outline => {
            "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
        }
    };

    let size_class = match context.size {
        ToggleSize::Sm => "h-8 min-w-8 px-1.5",
        ToggleSize::Default => "h-9 min-w-9 px-2",
        ToggleSize::Lg => "h-10 min-w-10 px-2.5",
    };

    let classes = format!(
        "inline-flex w-auto min-w-0 shrink-0 items-center justify-center gap-2 text-sm font-medium whitespace-nowrap px-3 \
         transition-[color,box-shadow] outline-none \
         hover:bg-muted hover:text-muted-foreground \
         focus:z-10 focus-visible:z-10 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:pointer-events-none disabled:opacity-50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 \
         data-[state=on]:bg-accent data-[state=on]:text-accent-foreground \
         rounded-none first:rounded-l-md last:rounded-r-md \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {} {} {}",
        variant_class, size_class, custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        let value = props.value.clone();
        let disabled = is_disabled;
        move |_| {
            if !disabled {
                context.toggle(value.clone());
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "toggle-group-item",
            "data-state": if is_pressed { "on" } else { "off" },
            "data-value": props.value.clone(),
            aria_pressed: is_pressed.to_string(),
            disabled: is_disabled,
            onclick: handle_click,

            {props.children}
        }
    }
}

/// Hook to access the toggle group context.
pub fn use_toggle_group() -> ToggleGroupContext {
    use_context::<ToggleGroupContext>()
}
