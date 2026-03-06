//! Form components with validation support.
//!
//! This module provides composable form primitives for building forms with
//! validation, error display, and accessible labeling.

use crate::use_unique_id;
use dioxus::prelude::*;
use std::collections::HashMap;

/// A validation error for a form field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldError {
    /// The error message to display.
    pub message: String,
    /// Optional error code for programmatic handling.
    pub code: Option<String>,
}

impl FieldError {
    /// Create a new field error with just a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }

    /// Create a new field error with a message and code.
    pub fn with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some(code.into()),
        }
    }
}

/// Context for form state management.
#[derive(Clone)]
pub struct FormContext {
    /// Field errors keyed by field name.
    pub errors: Signal<HashMap<String, Vec<FieldError>>>,
    /// Touched fields.
    pub touched: Signal<HashMap<String, bool>>,
    /// Whether the form is currently submitting.
    pub submitting: Signal<bool>,
}

impl FormContext {
    /// Set an error for a field.
    pub fn set_error(&mut self, field: &str, error: FieldError) {
        self.errors.with_mut(|errors| {
            errors
                .entry(field.to_string())
                .or_insert_with(Vec::new)
                .push(error);
        });
    }

    /// Set multiple errors for a field.
    pub fn set_errors(&mut self, field: &str, field_errors: Vec<FieldError>) {
        self.errors.with_mut(|errors| {
            errors.insert(field.to_string(), field_errors);
        });
    }

    /// Clear errors for a field.
    pub fn clear_errors(&mut self, field: &str) {
        self.errors.with_mut(|errors| {
            errors.remove(field);
        });
    }

    /// Clear all errors.
    pub fn clear_all_errors(&mut self) {
        self.errors.set(HashMap::new());
    }

    /// Get errors for a field.
    pub fn get_errors(&self, field: &str) -> Vec<FieldError> {
        self.errors.read().get(field).cloned().unwrap_or_default()
    }

    /// Check if a field has errors.
    pub fn has_errors(&self, field: &str) -> bool {
        self.errors
            .read()
            .get(field)
            .map(|e| !e.is_empty())
            .unwrap_or(false)
    }

    /// Mark a field as touched.
    pub fn touch(&mut self, field: &str) {
        self.touched.with_mut(|touched| {
            touched.insert(field.to_string(), true);
        });
    }

    /// Check if a field has been touched.
    pub fn is_touched(&self, field: &str) -> bool {
        *self.touched.read().get(field).unwrap_or(&false)
    }

    /// Check if the form is valid (no errors).
    pub fn is_valid(&self) -> bool {
        self.errors.read().values().all(|e| e.is_empty())
    }
}

/// Context for a specific form field.
#[derive(Clone)]
pub struct FieldContext {
    /// The field name.
    pub name: String,
    /// The field ID for linking labels.
    pub id: String,
    /// The first error for this field.
    pub error: Option<FieldError>,
    /// Whether the field has been touched.
    pub touched: bool,
}

/// Props for Form.
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    /// Callback when the form is submitted.
    #[props(default)]
    pub on_submit: Option<Callback<FormEvent>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Form content.
    pub children: Element,
}

/// A form container that provides validation context.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Form {
///         on_submit: move |_| {
///             // Handle submission
///         },
///
///         FormField { name: "email",
///             FormLabel { "Email" }
///             FormControl {
///                 Input { placeholder: "Enter your email" }
///             }
///             FormDescription { "We'll never share your email." }
///             FormMessage {}
///         }
///
///         Button { r#type: "submit", "Submit" }
///     }
/// }
/// ```
#[component]
pub fn Form(props: FormProps) -> Element {
    let errors = use_signal(HashMap::new);
    let touched = use_signal(HashMap::new);
    let submitting = use_signal(|| false);

    let context = FormContext {
        errors,
        touched,
        submitting,
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        if let Some(callback) = &props.on_submit {
            callback.call(event);
        }
    };

    rsx! {
        form {
            class: custom_class,
            "data-slot": "form",
            onsubmit: handle_submit,
            {props.children}
        }
    }
}

/// Props for FormField.
#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    /// The name of the field (used for error mapping).
    pub name: String,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Field content (FormLabel, FormControl, etc.).
    pub children: Element,
}

/// A container for a form field with its label, input, and messages.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let form_ctx = use_context::<FormContext>();
    let field_id = use_unique_id();
    let id = format!("field-{}", field_id());

    let errors = form_ctx.get_errors(&props.name);
    let error = errors.first().cloned();
    let touched = form_ctx.is_touched(&props.name);
    let has_error = error.is_some();

    let field_context = FieldContext {
        name: props.name.clone(),
        id,
        error,
        touched,
    };

    use_context_provider(|| field_context);

    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "grid gap-2 {custom_class}",
            "data-slot": "form-item",
            "data-name": props.name.clone(),
            "data-error": has_error.to_string(),
            {props.children}
        }
    }
}

/// Props for FormLabel.
#[derive(Props, Clone, PartialEq)]
pub struct FormLabelProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Label content.
    pub children: Element,
}

/// A label for a form field.
#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    let field_ctx = use_context::<FieldContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let error_class = if field_ctx.error.is_some() && field_ctx.touched {
        "text-destructive"
    } else {
        ""
    };

    let has_error = field_ctx.error.is_some() && field_ctx.touched;

    let classes = format!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none \
         group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 \
         {} {}",
        error_class, custom_class
    );

    rsx! {
        label {
            class: classes,
            r#for: field_ctx.id.clone(),
            "data-slot": "form-label",
            "data-error": has_error.to_string(),
            {props.children}
        }
    }
}

/// Props for FormControl.
#[derive(Props, Clone, PartialEq)]
pub struct FormControlProps {
    /// The form control (input, select, etc.).
    pub children: Element,
}

/// A wrapper for the form control element.
#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    let field_ctx = use_context::<FieldContext>();

    rsx! {
        div {
            "data-slot": "form-control",
            id: field_ctx.id.clone(),
            aria_describedby: "{field_ctx.id}-message",
            aria_invalid: if field_ctx.error.is_some() { "true" } else { "false" },
            {props.children}
        }
    }
}

/// Props for FormDescription.
#[derive(Props, Clone, PartialEq)]
pub struct FormDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Helper text describing a form field.
#[component]
pub fn FormDescription(props: FormDescriptionProps) -> Element {
    let field_ctx = use_context::<FieldContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-[0.8rem] text-muted-foreground {}", custom_class);

    rsx! {
        p {
            class: classes,
            id: "{field_ctx.id}-description",
            "data-slot": "form-description",
            {props.children}
        }
    }
}

/// Props for FormMessage.
#[derive(Props, Clone, PartialEq)]
pub struct FormMessageProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Optional custom message (overrides error message).
    #[props(default)]
    pub children: Option<Element>,
}

/// Displays validation error messages for a form field.
#[component]
pub fn FormMessage(props: FormMessageProps) -> Element {
    let field_ctx = use_context::<FieldContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let message = field_ctx.error.as_ref().map(|e| e.message.clone());

    // Don't render if no message and no children
    if message.is_none() && props.children.is_none() {
        return rsx! {};
    }

    let classes = format!(
        "text-[0.8rem] font-medium text-destructive {}",
        custom_class
    );

    rsx! {
        p {
            class: classes,
            id: "{field_ctx.id}-message",
            role: "alert",
            "data-slot": "form-message",

            if let Some(msg) = message {
                "{msg}"
            } else if let Some(children) = props.children {
                {children}
            }
        }
    }
}

/// Hook to access the form context.
pub fn use_form() -> FormContext {
    use_context::<FormContext>()
}

/// Hook to access the current field context.
pub fn use_field() -> FieldContext {
    use_context::<FieldContext>()
}
