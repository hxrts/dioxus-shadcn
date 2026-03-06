//! ToggleGroup example components.

use dioxus::prelude::*;
use lucide_dioxus::{Bold, Italic, Underline};
use dioxus_shadcn::components::toggle::{ToggleSize, ToggleVariant};
use dioxus_shadcn::components::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupType};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let alignment = use_signal(|| vec!["center".to_string()]);

rsx! {
    ToggleGroup {
        toggle_type: ToggleGroupType::Single,
        value: alignment,
        on_value_change: move |v| alignment.set(v),

        ToggleGroupItem { value: "left", "L" }
        ToggleGroupItem { value: "center", "C" }
        ToggleGroupItem { value: "right", "R" }
    }
}"#;

/// Basic toggle group example.
#[component]
pub fn ToggleGroupBasicExample() -> Element {
    let mut alignment = use_signal(|| vec!["center".to_string()]);

    rsx! {
        ToggleGroup {
            toggle_type: ToggleGroupType::Single,
            value: alignment,
            on_value_change: move |v| alignment.set(v),

            ToggleGroupItem { value: "left", "L" }
            ToggleGroupItem { value: "center", "C" }
            ToggleGroupItem { value: "right", "R" }
        }
    }
}

/// Source code for the multiple selection example.
pub const MULTIPLE_SOURCE: &str = r#"let formatting = use_signal(|| vec!["bold".to_string()]);

rsx! {
    ToggleGroup {
        toggle_type: ToggleGroupType::Multiple,
        value: formatting,
        on_value_change: move |v| formatting.set(v),

        ToggleGroupItem { value: "bold",
            Bold { class: "size-4" }
        }
        ToggleGroupItem { value: "italic",
            Italic { class: "size-4" }
        }
        ToggleGroupItem { value: "underline",
            Underline { class: "size-4" }
        }
    }
}"#;

/// Multiple selection toggle group example.
#[component]
pub fn ToggleGroupMultipleExample() -> Element {
    let mut formatting = use_signal(|| vec!["bold".to_string()]);

    rsx! {
        ToggleGroup {
            toggle_type: ToggleGroupType::Multiple,
            value: formatting,
            on_value_change: move |v| formatting.set(v),

            ToggleGroupItem { value: "bold",
                Bold { class: "size-4" }
            }
            ToggleGroupItem { value: "italic",
                Italic { class: "size-4" }
            }
            ToggleGroupItem { value: "underline",
                Underline { class: "size-4" }
            }
        }
    }
}

/// Source code for the outline variant example.
pub const OUTLINE_SOURCE: &str = r#"rsx! {
    ToggleGroup {
        toggle_type: ToggleGroupType::Single,
        variant: ToggleVariant::Outline,

        ToggleGroupItem { value: "left", "L" }
        ToggleGroupItem { value: "center", "C" }
        ToggleGroupItem { value: "right", "R" }
    }
}"#;

/// Outline variant toggle group example.
#[component]
pub fn ToggleGroupOutlineExample() -> Element {
    rsx! {
        ToggleGroup {
            toggle_type: ToggleGroupType::Single,
            variant: ToggleVariant::Outline,

            ToggleGroupItem { value: "left", "L" }
            ToggleGroupItem { value: "center", "C" }
            ToggleGroupItem { value: "right", "R" }
        }
    }
}

/// Source code for the sizes example.
pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "flex flex-col gap-4",
        ToggleGroup {
            toggle_type: ToggleGroupType::Single,
            size: ToggleSize::Sm,
            ToggleGroupItem { value: "a", "A" }
            ToggleGroupItem { value: "b", "B" }
            ToggleGroupItem { value: "c", "C" }
        }
        ToggleGroup {
            toggle_type: ToggleGroupType::Single,
            size: ToggleSize::Default,
            ToggleGroupItem { value: "a", "A" }
            ToggleGroupItem { value: "b", "B" }
            ToggleGroupItem { value: "c", "C" }
        }
        ToggleGroup {
            toggle_type: ToggleGroupType::Single,
            size: ToggleSize::Lg,
            ToggleGroupItem { value: "a", "A" }
            ToggleGroupItem { value: "b", "B" }
            ToggleGroupItem { value: "c", "C" }
        }
    }
}"#;

/// Toggle group sizes example.
#[component]
pub fn ToggleGroupSizesExample() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            ToggleGroup {
                toggle_type: ToggleGroupType::Single,
                size: ToggleSize::Sm,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
                ToggleGroupItem { value: "c", "C" }
            }
            ToggleGroup {
                toggle_type: ToggleGroupType::Single,
                size: ToggleSize::Default,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
                ToggleGroupItem { value: "c", "C" }
            }
            ToggleGroup {
                toggle_type: ToggleGroupType::Single,
                size: ToggleSize::Lg,
                ToggleGroupItem { value: "a", "A" }
                ToggleGroupItem { value: "b", "B" }
                ToggleGroupItem { value: "c", "C" }
            }
        }
    }
}
