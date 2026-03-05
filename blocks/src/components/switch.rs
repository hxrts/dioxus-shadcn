//! Switch component for toggling between on/off states.
//!
//! A toggle switch component with smooth animations and accessibility support.

use crate::{use_id_or, use_unique_id};
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch as PrimitiveSwitch, SwitchThumb};

/// Switch size options matching shadcn-ui.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SwitchSize {
    /// Small size
    Sm,
    /// Default size
    #[default]
    Default,
}

/// Props for the Switch component
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is checked
    #[props(default)]
    pub checked: Signal<bool>,

    /// Callback for when the switch is toggled
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,

    /// Whether the switch is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Size of the switch
    #[props(default)]
    pub size: SwitchSize,

    /// Optional ID for the switch
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Accessible label for the switch
    #[props(default)]
    pub aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A styled switch component that can be toggled on or off.
///
/// # Example
///
/// ```rust
/// let enabled = use_signal(|| false);
///
/// rsx! {
///     Switch {
///         checked: enabled,
///         on_checked_change: move |checked| enabled.set(checked),
///     }
/// }
/// ```
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    // Generate unique ID if not provided
    let switch_id = use_unique_id();
    let id_value = use_id_or(switch_id, props.id);
    let inner_checked_state = use_memo(move || Some((props.checked)()));

    // Determine size-specific classes matching shadcn-ui
    let (switch_size_class, thumb_size_class, thumb_translate) = match props.size {
        SwitchSize::Sm => (
            "h-3.5 w-6",              // data-[size=sm]:h-3.5 data-[size=sm]:w-6
            "size-3",                  // group-data-[size=sm]/switch:size-3
            "data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0",
        ),
        SwitchSize::Default => (
            "h-[1.15rem] w-8",        // data-[size=default]:h-[1.15rem] data-[size=default]:w-8
            "size-4",                  // group-data-[size=default]/switch:size-4
            "data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0",
        ),
    };

    let data_size = match props.size {
        SwitchSize::Sm => "sm",
        SwitchSize::Default => "default",
    };

    // Build full switch classes matching shadcn-ui styling
    let full_switch_classes = vec![
        // Base classes
        "peer group/switch inline-flex shrink-0 items-center rounded-full",
        "border border-transparent shadow-xs transition-all outline-none",
        // Focus styles
        "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        // Disabled styles
        "disabled:cursor-not-allowed disabled:opacity-50",
        // State-based colors
        "data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
        "dark:data-[state=unchecked]:bg-input/80",
        // Size class
        switch_size_class,
    ]
    .into_iter()
    .collect::<Vec<_>>()
    .join(" ");

    // Build thumb classes
    let full_thumb_classes = vec![
        // Base classes
        "pointer-events-none block rounded-full bg-background ring-0 transition-transform",
        // Dark mode colors
        "dark:data-[state=checked]:bg-primary-foreground dark:data-[state=unchecked]:bg-foreground",
        // Size class
        thumb_size_class,
        // Translation
        thumb_translate,
    ]
    .into_iter()
    .collect::<Vec<_>>()
    .join(" ");

    // Handler for change events
    let on_change = move |checked: bool| {
        if let Some(handler) = &props.on_checked_change {
            handler.call(checked);
        }
    };

    rsx! {
        PrimitiveSwitch {
            id: id_value,
            class: full_switch_classes,
            "data-slot": "switch",
            "data-size": data_size,
            checked: inner_checked_state,
            on_checked_change: on_change,
            disabled: (props.disabled)(),
            aria_label: props.aria_label.clone(),

            SwitchThumb {
                class: full_thumb_classes,
                "data-slot": "switch-thumb",
                aria_hidden: "true".to_string(),
            }
        }
    }
}
