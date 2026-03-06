use crate::use_unique_id;
use dioxus::prelude::*;
use lucide_dioxus::{Check, Minus};

/// Checkbox size options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CheckboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Props for the Checkbox component
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Controlled checked state (optional).
    /// When provided, the checkbox is in controlled mode and won't manage its own state.
    #[props(default)]
    pub checked: Option<Signal<bool>>,

    /// Default checked state for uncontrolled mode.
    /// Only used when `checked` prop is not provided.
    #[props(default = false)]
    pub default_checked: bool,

    /// Callback for when the checkbox is toggled.
    /// Always called with the new checked state, regardless of controlled/uncontrolled mode.
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,

    /// Whether the checkbox is in an indeterminate state.
    /// This represents a "partially checked" state, typically used for parent checkboxes
    /// that have some but not all children selected.
    #[props(default)]
    pub indeterminate: bool,

    /// Whether the checkbox is disabled
    #[props(default)]
    pub disabled: bool,

    /// Size of the checkbox
    #[props(default)]
    pub size: CheckboxSize,

    /// Optional ID for the checkbox
    #[props(default)]
    pub id: Option<String>,

    /// Name attribute for form submission
    #[props(default)]
    pub name: Option<String>,

    /// Accessible label for the checkbox
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Optional children (usually the indicator)
    #[props(default)]
    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A styled checkbox component that can be toggled on or off.
///
/// Supports both controlled and uncontrolled usage patterns, as well as
/// an indeterminate state for parent checkboxes with partial selection.
///
/// ## Uncontrolled (default)
/// ```rust
/// Checkbox {
///     default_checked: true,
///     on_checked_change: move |checked| {
///         println!("Checked: {}", checked);
///     }
/// }
/// ```
///
/// ## Controlled
/// ```rust
/// let is_checked = use_signal(|| false);
/// Checkbox {
///     checked: is_checked,
///     on_checked_change: move |new_value| {
///         is_checked.set(new_value);
///     }
/// }
/// ```
///
/// ## Indeterminate
/// ```rust
/// Checkbox {
///     indeterminate: true,
///     on_checked_change: move |_| {
///         // Handle click - typically sets to checked
///     }
/// }
/// ```
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    // Generate unique ID if not provided
    let checkbox_id = use_unique_id();
    let id = props.id.clone().unwrap_or(checkbox_id());

    // Internal state for uncontrolled mode
    let mut internal_checked = use_signal(|| props.default_checked);

    // Determine current checked state: prefer controlled, fall back to internal
    let is_checked = match &props.checked {
        Some(controlled) => controlled.read().clone(),
        None => *internal_checked.read(),
    };

    // Determine size-specific classes (shadcn uses size-4 as default)
    let (size_class, icon_size) = match props.size {
        CheckboxSize::Small => ("size-3.5", "size-3"),
        CheckboxSize::Medium => ("size-4", "size-3.5"),
        CheckboxSize::Large => ("size-5", "size-4"),
    };

    // Determine the checkbox state
    let state = if props.indeterminate {
        "indeterminate"
    } else if is_checked {
        "checked"
    } else {
        "unchecked"
    };

    // Build checkbox wrapper classes matching shadcn-ui
    let custom_class = props.class.as_deref().unwrap_or("");
    let checkbox_class = format!(
        "peer shrink-0 rounded-[4px] border border-input shadow-xs \
         transition-shadow outline-none \
         focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:cursor-not-allowed disabled:opacity-50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 \
         data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground \
         data-[state=indeterminate]:border-primary data-[state=indeterminate]:bg-primary data-[state=indeterminate]:text-primary-foreground \
         dark:bg-input/30 dark:data-[state=checked]:bg-primary dark:data-[state=indeterminate]:bg-primary \
         {} {} {}",
        size_class,
        if props.disabled {
            "cursor-not-allowed opacity-50"
        } else {
            "cursor-pointer"
        },
        custom_class
    );

    // Handle checkbox change
    let toggle_checked = {
        let checked_prop = props.checked.clone();
        let on_checked_change = props.on_checked_change.clone();
        let disabled = props.disabled;

        move || {
            if disabled {
                return;
            }

            let new_state = !is_checked;

            // Only update internal state in uncontrolled mode
            if checked_prop.is_none() {
                internal_checked.set(new_state);
            }

            // Always call the callback if provided
            if let Some(handler) = &on_checked_change {
                handler.call(new_state);
            }
        }
    };

    let on_click = {
        let mut toggle = toggle_checked.clone();
        move |_: MouseEvent| {
            toggle();
        }
    };

    // Handle keyboard activation
    let on_keydown = {
        let mut toggle = toggle_checked.clone();
        move |event: KeyboardEvent| match event.key() {
            Key::Character(ref s) if s == " " => {
                event.prevent_default();
                toggle();
            }
            Key::Enter => {
                event.prevent_default();
                toggle();
            }
            _ => {}
        }
    };

    // Determine aria-checked value (supports "mixed" for indeterminate)
    let aria_checked = if props.indeterminate {
        "mixed"
    } else if is_checked {
        "true"
    } else {
        "false"
    };

    rsx! {
        div {
            class: checkbox_class,
            role: "checkbox",
            "aria-checked": aria_checked,
            "aria-disabled": props.disabled.to_string(),
            "data-slot": "checkbox",
            "data-state": state,
            id: id.clone(),
            onclick: on_click,
            onkeydown: on_keydown,
            tabindex: if !props.disabled { "0" } else { "-1" },

            // Render indicator when checked or indeterminate (matching shadcn grid layout)
            if props.indeterminate {
                div {
                    class: "grid place-content-center text-current transition-none",
                    "data-slot": "checkbox-indicator",
                    Minus { class: "{icon_size}" }
                }
            } else if is_checked {
                div {
                    class: "grid place-content-center text-current transition-none",
                    "data-slot": "checkbox-indicator",
                    Check { class: "{icon_size}" }
                }
            }

            // Hidden input for form submission
            if let Some(name) = &props.name {
                input {
                    r#type: "checkbox",
                    id: "{id}-input",
                    name: name.clone(),
                    checked: is_checked,
                    disabled: props.disabled,
                    class: "sr-only",
                    "aria-hidden": "true",
                }
            }
        }
    }
}
