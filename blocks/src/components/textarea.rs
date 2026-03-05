//! Textarea component for multi-line text input.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Props for Textarea.
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    /// The controlled value.
    #[props(default)]
    pub value: Option<Signal<String>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Number of visible text rows.
    #[props(default = 3)]
    pub rows: u32,

    /// Whether the textarea is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the textarea is read-only.
    #[props(default)]
    pub readonly: bool,

    /// Whether the textarea is required.
    #[props(default)]
    pub required: bool,

    /// Maximum character length.
    #[props(default)]
    pub max_length: Option<u32>,

    /// Minimum character length.
    #[props(default)]
    pub min_length: Option<u32>,

    /// Name attribute for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// Optional ID.
    #[props(default)]
    pub id: Option<String>,

    /// Callback when value changes.
    #[props(default)]
    pub on_change: Option<Callback<String>>,

    /// Callback on input.
    #[props(default)]
    pub on_input: Option<Callback<String>>,

    /// Whether the field has an error.
    #[props(default)]
    pub error: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A multi-line text input component.
///
/// # Example
///
/// ```rust
/// let bio = use_signal(String::new);
///
/// rsx! {
///     Textarea {
///         value: bio,
///         placeholder: "Tell us about yourself...",
///         rows: 4,
///         on_change: move |v| bio.set(v),
///     }
/// }
/// ```
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let textarea_id = use_unique_id();
    let id = props.id.clone().unwrap_or_else(|| textarea_id());

    let mut internal_value = use_signal(|| props.default_value.clone().unwrap_or_default());
    let current_value = props
        .value
        .map(|s| s.read().clone())
        .unwrap_or_else(|| internal_value.read().clone());

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent shadow-xs px-3 py-2 text-base md:text-sm \
         placeholder:text-muted-foreground transition-[color,box-shadow] outline-none \
         focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:cursor-not-allowed disabled:opacity-50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 dark:bg-input/30 \
         {} {}",
        if props.error {
            "border-destructive"
        } else {
            "border-input"
        },
        custom_class
    );

    let handle_input = {
        let value_signal = props.value;
        let on_input = props.on_input.clone();
        let on_change = props.on_change.clone();

        move |event: FormEvent| {
            let new_value = event.value();

            // Update internal state only in uncontrolled mode
            if value_signal.is_none() {
                internal_value.set(new_value.clone());
            }

            if let Some(callback) = &on_input {
                callback.call(new_value.clone());
            }
            if let Some(callback) = &on_change {
                callback.call(new_value);
            }
        }
    };

    rsx! {
        textarea {
            id: id,
            class: classes,
            "data-slot": "textarea",
            "data-error": props.error.to_string(),
            "data-variant": if props.error { "error" } else { "default" },
            rows: props.rows,
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            required: props.required,
            maxlength: props.max_length,
            minlength: props.min_length,
            name: props.name.clone(),
            value: current_value,
            aria_invalid: props.error.to_string(),
            oninput: handle_input,
        }
    }
}
