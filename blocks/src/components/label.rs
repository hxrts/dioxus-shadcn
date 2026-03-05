use crate::{use_id_or, use_unique_id};
use dioxus::prelude::*;

/// Label size options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The HTML for attribute that associates the label with a form control
    #[props(default)]
    for_id: ReadSignal<Option<String>>,

    /// The size of the label
    #[props(default)]
    size: ReadSignal<LabelSize>,

    /// Whether the label is for a required field
    #[props(default)]
    required: ReadSignal<bool>,

    /// Optional ID for the label element
    #[props(default)]
    id: ReadSignal<Option<String>>,

    /// Optional additional classes for the label
    #[props(default)]
    class: Option<String>,

    /// Whether to display the label as disabled
    #[props(default)]
    disabled: ReadSignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    // Generate unique ID if not provided
    let label_id = use_unique_id();
    let id_value = use_id_or(label_id, props.id);

    // Determine size classes (dioxus extension, shadcn doesn't have this)
    let size_classes = match (props.size)() {
        LabelSize::Small => "text-xs",
        LabelSize::Medium => "text-sm",
        LabelSize::Large => "text-base",
    };

    // Generate all the classes (matches shadcn label.tsx)
    let label_classes = vec![
        // Base classes matching shadcn
        "flex items-center gap-2 leading-none font-medium select-none",
        // Peer disabled support
        "peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
        // Group disabled support
        "group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
        // Size-specific classes
        size_classes,
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        label {
            id: id_value,
            class: label_classes,
            for: (props.for_id)(),
            "data-slot": "label",
            "data-size": match (props.size)() {
                LabelSize::Small => "sm",
                LabelSize::Medium => "default",
                LabelSize::Large => "lg",
            },

            // Pass through other attributes
            ..props.attributes,

            // Label content
            {props.children}

            // Required indicator
            if (props.required)() {
                span {
                    class: "ml-1 text-destructive",
                    aria_hidden: "true",
                    "*"
                }
            }
        }
    }
}
