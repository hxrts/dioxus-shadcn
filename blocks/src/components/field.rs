//! Field components for form layouts.
//!
//! A comprehensive form field component system for building forms with
//! flexible layouts, descriptions, and error handling.

use dioxus::prelude::*;

/// Orientation for field layout.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FieldOrientation {
    /// Vertical layout (label above input).
    #[default]
    Vertical,
    /// Horizontal layout (label beside input).
    Horizontal,
    /// Responsive layout (vertical on small screens, horizontal on larger).
    Responsive,
}

impl FieldOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "vertical",
            FieldOrientation::Horizontal => "horizontal",
            FieldOrientation::Responsive => "responsive",
        }
    }

    fn classes(&self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
            FieldOrientation::Horizontal => {
                "flex-row items-center \
                 [&>[data-slot=field-label]]:flex-auto \
                 has-[>[data-slot=field-content]]:items-start"
            }
            FieldOrientation::Responsive => {
                "flex-col @md/field-group:flex-row @md/field-group:items-center \
                 [&>*]:w-full @md/field-group:[&>*]:w-auto [&>.sr-only]:w-auto \
                 @md/field-group:[&>[data-slot=field-label]]:flex-auto \
                 @md/field-group:has-[>[data-slot=field-content]]:items-start"
            }
        }
    }
}

/// Variant for field legend.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FieldLegendVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FieldLegendVariant::Legend => "legend",
            FieldLegendVariant::Label => "label",
        }
    }
}

/// Props for FieldSet.
#[derive(Props, Clone, PartialEq)]
pub struct FieldSetProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Fieldset content.
    pub children: Element,
}

/// A container for grouped form fields.
///
/// # Example
///
/// ```rust
/// rsx! {
///     FieldSet {
///         FieldLegend { "Personal Information" }
///         FieldGroup {
///             Field {
///                 FieldLabel { "Name" }
///                 Input { placeholder: "Enter your name" }
///             }
///             Field {
///                 FieldLabel { "Email" }
///                 Input { r#type: "email", placeholder: "Enter your email" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn FieldSet(props: FieldSetProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-col gap-6 \
         has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3 \
         {}",
        custom_class
    );

    rsx! {
        fieldset {
            class: classes,
            "data-slot": "field-set",
            {props.children}
        }
    }
}

/// Props for FieldLegend.
#[derive(Props, Clone, PartialEq)]
pub struct FieldLegendProps {
    /// Visual variant.
    #[props(default)]
    pub variant: FieldLegendVariant,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Legend content.
    pub children: Element,
}

/// A legend for a fieldset.
#[component]
pub fn FieldLegend(props: FieldLegendProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "mb-3 font-medium \
         data-[variant=legend]:text-base \
         data-[variant=label]:text-sm \
         {}",
        custom_class
    );

    rsx! {
        legend {
            class: classes,
            "data-slot": "field-legend",
            "data-variant": props.variant.as_str(),
            {props.children}
        }
    }
}

/// Props for FieldGroup.
#[derive(Props, Clone, PartialEq)]
pub struct FieldGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group content.
    pub children: Element,
}

/// A container for grouping multiple fields.
#[component]
pub fn FieldGroup(props: FieldGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/field-group @container/field-group flex w-full flex-col gap-7 \
         data-[slot=checkbox-group]:gap-3 \
         [&>[data-slot=field-group]]:gap-4 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "field-group",
            {props.children}
        }
    }
}

/// Props for Field.
#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    /// Layout orientation.
    #[props(default)]
    pub orientation: FieldOrientation,

    /// Whether the field has an invalid state.
    #[props(default)]
    pub invalid: bool,

    /// Whether the field is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Field content.
    pub children: Element,
}

/// A field wrapper with flexible layout options.
#[component]
pub fn Field(props: FieldProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let orientation_class = props.orientation.classes();

    let classes = format!(
        "group/field flex w-full gap-3 data-[invalid=true]:text-destructive \
         {} {}",
        orientation_class, custom_class
    );

    rsx! {
        div {
            role: "group",
            class: classes,
            "data-slot": "field",
            "data-orientation": props.orientation.as_str(),
            "data-invalid": props.invalid.to_string(),
            "data-disabled": props.disabled.to_string(),
            {props.children}
        }
    }
}

/// Props for FieldContent.
#[derive(Props, Clone, PartialEq)]
pub struct FieldContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content.
    pub children: Element,
}

/// Content container for a field.
#[component]
pub fn FieldContent(props: FieldContentProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/field-content flex flex-1 flex-col gap-1.5 leading-snug \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "field-content",
            {props.children}
        }
    }
}

/// Props for FieldLabel.
#[derive(Props, Clone, PartialEq)]
pub struct FieldLabelProps {
    /// The ID of the input this label is for.
    #[props(default)]
    pub for_id: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Label content.
    pub children: Element,
}

/// A label for a field with enhanced styling.
#[component]
pub fn FieldLabel(props: FieldLabelProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/field-label peer/field-label flex w-fit gap-2 leading-snug \
         text-sm font-medium select-none \
         group-data-[disabled=true]/field:opacity-50 \
         has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col \
         has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border \
         [&>*]:data-[slot=field]:p-4 \
         has-data-[state=checked]:border-primary has-data-[state=checked]:bg-primary/5 \
         dark:has-data-[state=checked]:bg-primary/10 \
         {}",
        custom_class
    );

    rsx! {
        label {
            class: classes,
            "data-slot": "field-label",
            r#for: props.for_id.clone(),
            {props.children}
        }
    }
}

/// Props for FieldTitle.
#[derive(Props, Clone, PartialEq)]
pub struct FieldTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// A lightweight title for a field section.
#[component]
pub fn FieldTitle(props: FieldTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex w-fit items-center gap-2 text-sm leading-snug font-medium \
         group-data-[disabled=true]/field:opacity-50 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "field-label",
            {props.children}
        }
    }
}

/// Props for FieldDescription.
#[derive(Props, Clone, PartialEq)]
pub struct FieldDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Helper text describing a field.
#[component]
pub fn FieldDescription(props: FieldDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "text-sm leading-normal font-normal text-muted-foreground \
         group-has-[[data-orientation=horizontal]]/field:text-balance \
         last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5 \
         [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary \
         {}",
        custom_class
    );

    rsx! {
        p {
            class: classes,
            "data-slot": "field-description",
            {props.children}
        }
    }
}

/// Props for FieldSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct FieldSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Optional content to display in the separator.
    #[props(default)]
    pub children: Option<Element>,
}

/// A visual separator between fields.
#[component]
pub fn FieldSeparator(props: FieldSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let has_content = props.children.is_some();

    let classes = format!(
        "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "field-separator",
            "data-content": has_content.to_string(),

            // Separator line
            div {
                class: "absolute inset-0 top-1/2 -translate-y-1/2 h-px bg-border",
                "data-slot": "separator",
            }

            if let Some(children) = props.children {
                span {
                    class: "relative mx-auto block w-fit bg-background px-2 text-muted-foreground",
                    "data-slot": "field-separator-content",
                    {children}
                }
            }
        }
    }
}

/// A field error.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldErrorItem {
    /// The error message.
    pub message: String,
}

impl FieldErrorItem {
    /// Create a new field error.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Props for FieldError.
#[derive(Props, Clone, PartialEq)]
pub struct FieldErrorProps {
    /// Error items to display.
    #[props(default)]
    pub errors: Vec<FieldErrorItem>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Custom error content (overrides errors).
    #[props(default)]
    pub children: Option<Element>,
}

/// Displays validation error messages for a field.
#[component]
pub fn FieldError(props: FieldErrorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    // If children provided, use them
    if let Some(children) = props.children {
        let classes = format!("text-sm font-normal text-destructive {}", custom_class);
        return rsx! {
            div {
                role: "alert",
                class: classes,
                "data-slot": "field-error",
                {children}
            }
        };
    }

    // No errors, don't render
    if props.errors.is_empty() {
        return rsx! {};
    }

    // Deduplicate errors
    let unique_errors: Vec<_> = {
        let mut seen = std::collections::HashSet::new();
        props
            .errors
            .iter()
            .filter(|e| seen.insert(e.message.clone()))
            .collect()
    };

    let classes = format!("text-sm font-normal text-destructive {}", custom_class);

    rsx! {
        div {
            role: "alert",
            class: classes,
            "data-slot": "field-error",

            if unique_errors.len() == 1 {
                "{unique_errors[0].message}"
            } else {
                ul {
                    class: "ml-4 flex list-disc flex-col gap-1",
                    for error in unique_errors {
                        li { "{error.message}" }
                    }
                }
            }
        }
    }
}
