//! Toggle component for binary states.
//!
//! A two-state button that can be either "on" or "off".

use dioxus::prelude::*;

/// Variant styles for the toggle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Size variants for the toggle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

/// Props for Toggle.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Controlled pressed state.
    #[props(default)]
    pub pressed: Option<Signal<bool>>,

    /// Default pressed state for uncontrolled mode.
    #[props(default)]
    pub default_pressed: bool,

    /// Callback when pressed state changes.
    #[props(default)]
    pub on_pressed_change: Option<Callback<bool>>,

    /// Visual variant.
    #[props(default)]
    pub variant: ToggleVariant,

    /// Size variant.
    #[props(default)]
    pub size: ToggleSize,

    /// Whether the toggle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Toggle content (usually an icon).
    pub children: Element,
}

/// A toggle button component.
///
/// # Example
///
/// ```rust
/// let bold = use_signal(|| false);
///
/// rsx! {
///     Toggle {
///         pressed: bold,
///         on_pressed_change: move |v| bold.set(v),
///         aria_label: "Toggle bold",
///
///         // Bold icon here
///     }
/// }
/// ```
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    // Internal state for uncontrolled mode
    let internal_pressed = use_signal(|| props.default_pressed);

    // Use controlled or internal state
    let pressed = props.pressed.unwrap_or(internal_pressed);

    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        ToggleVariant::Default => "bg-transparent",
        ToggleVariant::Outline => {
            "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground"
        }
    };

    let size_class = match props.size {
        ToggleSize::Sm => "h-8 px-2 min-w-8",
        ToggleSize::Default => "h-9 px-3 min-w-9",
        ToggleSize::Lg => "h-10 px-4 min-w-10",
    };

    let pressed_class = if *pressed.read() {
        "bg-accent text-accent-foreground"
    } else {
        "hover:bg-muted hover:text-muted-foreground"
    };

    let classes = format!(
        "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium \
         ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 \
         {} {} {} {}",
        variant_class, size_class, pressed_class, custom_class
    );

    let handle_click = {
        let mut pressed = pressed;
        let on_pressed_change = props.on_pressed_change.clone();
        let disabled = props.disabled;
        move |_| {
            if !disabled {
                let new_value = !*pressed.read();
                pressed.set(new_value);
                if let Some(callback) = &on_pressed_change {
                    callback.call(new_value);
                }
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "toggle",
            "data-state": if *pressed.read() { "on" } else { "off" },
            aria_pressed: pressed.read().to_string(),
            disabled: props.disabled,
            onclick: handle_click,

            {props.children}
        }
    }
}
