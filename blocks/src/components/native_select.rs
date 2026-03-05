//! NativeSelect component for native HTML select elements.
//!
//! A styled native HTML select element with proper accessibility and theming.

use dioxus::prelude::*;

/// Size variants for the native select.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum NativeSelectSize {
    /// Small size (h-8)
    Sm,
    /// Default size (h-9)
    #[default]
    Default,
}

impl NativeSelectSize {
    fn class(&self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "h-8 py-1 text-xs",
            NativeSelectSize::Default => "h-9 py-2 text-sm",
        }
    }
}

/// Props for NativeSelect.
#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectProps {
    /// Size variant.
    #[props(default)]
    pub size: NativeSelectSize,

    /// Whether the select is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the select has an error.
    #[props(default)]
    pub invalid: bool,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// The id attribute.
    #[props(default)]
    pub id: Option<String>,

    /// The current value.
    #[props(default)]
    pub value: Option<String>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Callback when the value changes.
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Select options.
    pub children: Element,
}

/// A styled native HTML select element.
///
/// # Example
///
/// ```rust
/// rsx! {
///     NativeSelect {
///         on_change: move |value| println!("Selected: {value}"),
///
///         NativeSelectOption { value: "", "Select an option" }
///         NativeSelectOption { value: "apple", "Apple" }
///         NativeSelectOption { value: "banana", "Banana" }
///         NativeSelectOption { value: "orange", "Orange" }
///     }
/// }
/// ```
#[component]
pub fn NativeSelect(props: NativeSelectProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let size_class = props.size.class();

    let classes = format!(
        "w-full min-w-0 appearance-none rounded-md border border-input bg-transparent px-3 pr-9 \
         shadow-xs transition-[color,box-shadow] outline-none \
         focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:cursor-not-allowed disabled:opacity-50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 \
         dark:aria-invalid:ring-destructive/40 \
         dark:bg-input/30 dark:hover:bg-input/50 \
         {} {}",
        size_class, custom_class
    );

    let handle_change = {
        let on_change = props.on_change.clone();
        move |event: Event<FormData>| {
            if let Some(handler) = &on_change {
                handler.call(event.value());
            }
        }
    };

    rsx! {
        div {
            class: "relative",
            "data-slot": "native-select",

            select {
                class: classes,
                "data-slot": "native-select-trigger",
                disabled: props.disabled,
                "aria-invalid": props.invalid.to_string(),
                name: props.name.clone(),
                id: props.id.clone(),
                value: props.value.clone().or(props.default_value.clone()),
                onchange: handle_change,

                {props.children}
            }

            // Chevron down icon
            div {
                class: "pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground",
                "aria-hidden": "true",

                svg {
                    class: "size-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "m6 9 6 6 6-6" }
                }
            }
        }
    }
}

/// Props for NativeSelectOption.
#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptionProps {
    /// The value of this option.
    #[props(default)]
    pub value: Option<String>,

    /// Whether this option is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Option label.
    pub children: Element,
}

/// An option within a native select.
#[component]
pub fn NativeSelectOption(props: NativeSelectOptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        option {
            class: custom_class,
            "data-slot": "native-select-option",
            value: props.value.clone(),
            disabled: props.disabled,
            {props.children}
        }
    }
}

/// Props for NativeSelectOptGroup.
#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptGroupProps {
    /// The label for this group.
    pub label: String,

    /// Whether this group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Grouped options.
    pub children: Element,
}

/// An option group within a native select.
#[component]
pub fn NativeSelectOptGroup(props: NativeSelectOptGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        optgroup {
            class: custom_class,
            "data-slot": "native-select-optgroup",
            label: props.label.clone(),
            disabled: props.disabled,
            {props.children}
        }
    }
}
