//! RadioGroup component for selecting one option from a set.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing radio group state.
#[derive(Clone)]
struct RadioGroupContext {
    value: Signal<String>,
    name: String,
    disabled: bool,
    on_value_change: Option<Callback<String>>,
}

/// Props for RadioGroup.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// The controlled value.
    #[props(default)]
    pub value: Option<Signal<String>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Name attribute for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// Whether all radio items are disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Option<Callback<String>>,

    /// Layout direction.
    #[props(default)]
    pub orientation: RadioGroupOrientation,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Radio items.
    pub children: Element,
}

/// Layout orientation for the radio group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// A group of radio buttons where only one can be selected.
///
/// # Example
///
/// ```rust
/// let selected = use_signal(|| "option1".to_string());
///
/// rsx! {
///     RadioGroup {
///         value: selected,
///         on_value_change: move |v| selected.set(v),
///
///         div { class: "flex items-center space-x-2",
///             RadioGroupItem { value: "option1", id: "r1" }
///             Label { r#for: "r1", "Option 1" }
///         }
///         div { class: "flex items-center space-x-2",
///             RadioGroupItem { value: "option2", id: "r2" }
///             Label { r#for: "r2", "Option 2" }
///         }
///     }
/// }
/// ```
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let group_id = use_unique_id();
    let name = props.name.clone().unwrap_or_else(|| group_id());

    let default = props.default_value.clone().unwrap_or_default();
    let internal_value = use_signal(|| default);
    let value = props.value.unwrap_or(internal_value);

    let context = RadioGroupContext {
        value,
        name,
        disabled: props.disabled,
        on_value_change: props.on_value_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    let orientation_class = match props.orientation {
        RadioGroupOrientation::Vertical => "flex flex-col space-y-2",
        RadioGroupOrientation::Horizontal => "flex flex-row space-x-4",
    };

    let classes = format!("{} {}", orientation_class, custom_class);

    rsx! {
        div {
            class: classes,
            role: "radiogroup",
            "data-slot": "radio-group",
            "data-orientation": match props.orientation {
                RadioGroupOrientation::Vertical => "vertical",
                RadioGroupOrientation::Horizontal => "horizontal",
            },
            aria_disabled: props.disabled.to_string(),
            {props.children}
        }
    }
}

/// Props for RadioGroupItem.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupItemProps {
    /// The value of this radio item.
    pub value: String,

    /// Optional ID for the radio input.
    #[props(default)]
    pub id: Option<String>,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A single radio button within a RadioGroup.
#[component]
pub fn RadioGroupItem(props: RadioGroupItemProps) -> Element {
    let context = use_context::<RadioGroupContext>();
    let item_id = use_unique_id();
    let id = props.id.clone().unwrap_or_else(|| item_id());

    let is_checked = *context.value.read() == props.value;
    let is_disabled = props.disabled || context.disabled;

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "aspect-square h-4 w-4 rounded-full border border-primary text-primary \
         ring-offset-background focus:outline-none focus-visible:ring-2 \
         focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:cursor-not-allowed disabled:opacity-50 {}",
        custom_class
    );

    let handle_change = {
        let value = props.value.clone();
        let on_change = context.on_value_change.clone();
        move |_| {
            if !is_disabled {
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
            role: "radio",
            id: id.clone(),
            class: classes,
            "data-slot": "radio-group-item",
            "data-state": if is_checked { "checked" } else { "unchecked" },
            aria_checked: is_checked.to_string(),
            disabled: is_disabled,
            onclick: handle_change,

            // Inner indicator circle
            if is_checked {
                span {
                    class: "flex items-center justify-center",
                    "data-slot": "radio-group-indicator",
                    span {
                        class: "h-2.5 w-2.5 rounded-full bg-current",
                    }
                }
            }
        }

        // Hidden input for form submission
        input {
            r#type: "radio",
            name: context.name.clone(),
            value: props.value.clone(),
            checked: is_checked,
            disabled: is_disabled,
            class: "sr-only",
            "aria-hidden": "true",
        }
    }
}
