use crate::{use_id_or, use_unique_id};
use dioxus::prelude::*;
use lucide_dioxus::LoaderCircle;

/// Button variant types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonVariant {
    /// Default/primary button style (renamed from Primary to match shadcn-ui)
    #[default]
    Default,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

/// Alias for backward compatibility
#[deprecated(since = "0.4.0", note = "Use ButtonVariant::Default instead")]
pub const PRIMARY: ButtonVariant = ButtonVariant::Default;

/// Button size options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonSize {
    Xs,
    Small,
    #[default]
    Medium,
    Large,
    /// Extra small icon button
    IconXs,
    /// Small icon button
    IconSm,
    /// Default icon button
    Icon,
    /// Large icon button
    IconLg,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// The button type (submit, reset, button)
    #[props(default = String::from("button"))]
    button_type: String,

    /// The variant of the button
    #[props(default)]
    variant: ButtonVariant,

    /// The size of the button
    #[props(default)]
    size: ButtonSize,

    /// Whether the button is disabled
    #[props(default)]
    disabled: bool,

    /// Whether the button is in a loading state
    #[props(default)]
    loading: bool,

    /// Whether the button is displayed as a full width block
    #[props(default)]
    full_width: bool,

    /// Whether the button is an icon-only button (square with centered icon)
    /// Note: When using icon-only buttons, providing an aria-label is strongly recommended
    /// for accessibility purposes as there is no visible text to identify the button.
    #[props(default)]
    is_icon_button: bool,

    /// Callback when the button is clicked
    #[props(default)]
    on_click: Option<Callback<MouseEvent>>,

    /// Name of the button for form submission
    #[props(default)]
    name: String,

    /// Value of the button for form submission
    #[props(default)]
    value: String,

    /// Optional ID for the button
    #[props(default)]
    id: Option<String>,

    /// Optional icon to display before the button text
    #[props(default)]
    icon_left: Option<Element>,

    /// Optional icon to display after the button text
    #[props(default)]
    icon_right: Option<Element>,

    /// Optional aria-label for the button (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional ID of the element that labels this button (for accessibility)
    #[props(default)]
    aria_labelledby: Option<String>,

    /// Optional ID of the element that describes this button (for accessibility)
    #[props(default)]
    aria_describedby: Option<String>,

    /// Optional aria-controls attribute
    #[props(default)]
    aria_controls: Option<String>,

    /// Optional aria-expanded attribute
    #[props(default)]
    aria_expanded: Option<bool>,

    /// Optional aria-pressed attribute
    #[props(default)]
    aria_pressed: Option<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Generate unique ID if not provided
    let button_id = use_unique_id();
    let props_id_signal = use_signal(|| props.id);
    let id_value = use_id_or(button_id, props_id_signal.into());

    // Check if icon button has aria label for accessibility
    #[cfg(debug_assertions)]
    {
        if props.is_icon_button && props.aria_label.is_none() && props.aria_labelledby.is_none() {
            log::warn!(
                "Icon button missing aria-label or aria-labelledby attribute. This may cause accessibility issues."
            );
        }
    }

    // Determine base classes for button based on variant
    let variant_classes = match props.variant {
        ButtonVariant::Default => {
            "bg-primary text-primary-foreground hover:bg-primary/90"
        }
        ButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }
        ButtonVariant::Outline => {
            "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
        }
        ButtonVariant::Ghost => {
            "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50"
        }
        ButtonVariant::Link => {
            "text-primary underline-offset-4 hover:underline"
        }
        ButtonVariant::Destructive => {
            "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40"
        }
    };

    // Determine size classes based on whether it's an icon button or regular button
    let size_classes = if props.is_icon_button {
        match props.size {
            ButtonSize::IconXs | ButtonSize::Xs => "size-6 rounded-md [&_svg:not([class*='size-'])]:size-3",
            ButtonSize::IconSm | ButtonSize::Small => "size-8",
            ButtonSize::Icon | ButtonSize::Medium => "size-9",
            ButtonSize::IconLg | ButtonSize::Large => "size-10",
        }
    } else {
        match props.size {
            ButtonSize::Xs | ButtonSize::IconXs => "h-6 gap-1 rounded-md px-2 text-xs has-[>svg]:px-1.5 [&_svg:not([class*='size-'])]:size-3",
            ButtonSize::Small | ButtonSize::IconSm => "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
            ButtonSize::Medium | ButtonSize::Icon => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Large | ButtonSize::IconLg => "h-10 rounded-md px-6 has-[>svg]:px-4",
        }
    };

    // Determine if the button should be full width (only for non-icon buttons)
    let width_class = if props.is_icon_button {
        "w-auto" // Icon buttons should never be full width
    } else if props.full_width {
        "w-full"
    } else {
        "w-auto"
    };

    // Determine disabled and loading state classes
    let state_class = if props.disabled || props.loading {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    // Generate all the classes in a more maintainable way
    let button_classes = vec![
        // Base classes that apply to all buttons
        "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap",
        "transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40",
        // Icon sizing
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        // Variant-specific classes
        variant_classes,
        // Size-specific classes
        size_classes,
        // Width class
        width_class,
        // Icon button gets aspect-square class
        if props.is_icon_button {
            "aspect-square"
        } else {
            ""
        },
        // State class (disabled/loading)
        state_class,
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    // Handle click event
    let handle_click = move |event: MouseEvent| {
        if let Some(callback) = &props.on_click {
            callback.call(event);
        }
    };

    rsx! {
        button {
            // Standard HTML attributes
            id: id_value,
            r#type: props.button_type.clone(),
            name: props.name,
            value: props.value,
            disabled: props.disabled || props.loading,
            class: button_classes,
            "data-slot": "button",
            "data-loading": props.loading.to_string(),
            "data-variant": match props.variant {
                ButtonVariant::Default => "default",
                ButtonVariant::Secondary => "secondary",
                ButtonVariant::Outline => "outline",
                ButtonVariant::Ghost => "ghost",
                ButtonVariant::Link => "link",
                ButtonVariant::Destructive => "destructive",
            },
            "data-size": match props.size {
                ButtonSize::Xs | ButtonSize::IconXs => "xs",
                ButtonSize::Small | ButtonSize::IconSm => "sm",
                ButtonSize::Medium | ButtonSize::Icon => "default",
                ButtonSize::Large | ButtonSize::IconLg => "lg",
            },
            onclick: handle_click,

            // ARIA attributes
            aria_label: if props.is_icon_button && props.aria_label.is_none() {
                // Fallback for icon buttons without aria-label
                Some("Button".to_string())
            } else {
                props.aria_label.clone()
            },
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_controls: props.aria_controls.clone(),
            aria_expanded: props.aria_expanded.map(|v| v.to_string()),
            aria_pressed: props.aria_pressed.map(|v| v.to_string()),
            aria_disabled: (props.disabled || props.loading).to_string(),

            // Pass through other attributes
            ..props.attributes,

            if props.is_icon_button {
                // Icon button content
                if props.loading {
                    // Loading spinner for icon button
                    span {
                        class: "animate-spin inline-block",
                        aria_hidden: "true",
                        LoaderCircle {
                            class: "h-4 w-4",
                        }
                    }
                } else {
                    // Icon only when not loading
                    {props.children}
                }
            } else {
                // Standard button content
                if props.loading {
                    // Loading spinner for standard button
                    span {
                        LoaderCircle {
                            class: "mr-1 inline-block animate-spin h-4",
                        }
                    }
                }

                // Left icon if provided
                if let Some(icon) = &props.icon_left {
                    span {
                        class: "mr-2",
                        aria_hidden: "true",
                        {icon.clone()}
                    }
                }

                // Button content (always shown for standard buttons)
                {props.children}

                // Right icon if provided
                if let Some(icon) = &props.icon_right {
                    span {
                        class: "ml-2",
                        aria_hidden: "true",
                        {icon.clone()}
                    }
                }
            }
        }
    }
}
