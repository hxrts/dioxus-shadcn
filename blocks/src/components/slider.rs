//! Slider component for selecting values from a range.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Props for the Slider component.
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// The controlled value.
    #[props(default)]
    pub value: Option<Signal<f64>>,

    /// Default value for uncontrolled mode.
    #[props(default = 50.0)]
    pub default_value: f64,

    /// Minimum value.
    #[props(default = 0.0)]
    pub min: f64,

    /// Maximum value.
    #[props(default = 100.0)]
    pub max: f64,

    /// Step increment.
    #[props(default = 1.0)]
    pub step: f64,

    /// Whether the slider is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Option<Callback<f64>>,

    /// Optional ID.
    #[props(default)]
    pub id: Option<String>,

    /// Accessible label.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A slider input for selecting numeric values.
///
/// # Example
///
/// ```rust
/// let volume = use_signal(|| 50.0);
///
/// rsx! {
///     Slider {
///         value: volume,
///         min: 0.0,
///         max: 100.0,
///         on_value_change: move |v| volume.set(v),
///     }
/// }
/// ```
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let slider_id = use_unique_id();
    let id = props.id.clone().unwrap_or_else(|| slider_id());

    let mut internal_value = use_signal(|| props.default_value);
    let current_value = props
        .value
        .map(|s| *s.read())
        .unwrap_or(*internal_value.read());

    // Calculate percentage for styling
    let percentage =
        ((current_value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0);

    let custom_class = props.class.as_deref().unwrap_or("");

    let track_classes = format!(
        "group/slider relative flex w-full touch-none items-center select-none data-[disabled=true]:opacity-50 {}",
        custom_class
    );

    let handle_change = {
        let value_signal = props.value;
        let on_change = props.on_value_change.clone();
        let disabled = props.disabled;

        move |event: FormEvent| {
            if disabled {
                return;
            }

            if let Ok(new_value) = event.value().parse::<f64>() {
                // Update internal state only in uncontrolled mode
                if value_signal.is_none() {
                    internal_value.set(new_value);
                }

                if let Some(callback) = &on_change {
                    callback.call(new_value);
                }
            }
        }
    };

    rsx! {
        div {
            class: track_classes,
            "data-slot": "slider",
            "data-disabled": props.disabled.to_string(),

            // Track
            div {
                class: "relative h-1.5 w-full grow overflow-hidden rounded-full bg-muted",
                "data-slot": "slider-track",

                // Range (filled portion)
                div {
                    class: "absolute h-full bg-primary",
                    "data-slot": "slider-range",
                    style: "width: {percentage}%",
                }
            }

            // Native input for accessibility
            input {
                r#type: "range",
                id: id,
                class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed",
                min: props.min,
                max: props.max,
                step: props.step,
                value: current_value,
                disabled: props.disabled,
                aria_label: props.aria_label.clone(),
                aria_valuemin: props.min,
                aria_valuemax: props.max,
                aria_valuenow: current_value,
                oninput: handle_change,
            }

            // Visual thumb
            div {
                class: "block size-4 shrink-0 rounded-full border border-primary bg-white shadow-sm \
                        ring-ring/50 transition-[color,box-shadow] hover:ring-4 \
                        focus-visible:ring-4 focus-visible:outline-hidden \
                        disabled:pointer-events-none disabled:opacity-50",
                "data-slot": "slider-thumb",
                style: "position: absolute; left: calc({percentage}% - 0.5rem)",
                "data-disabled": props.disabled.to_string(),
            }
        }
    }
}
