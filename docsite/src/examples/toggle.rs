//! Toggle example components.

use dioxus::prelude::*;
use lumen_blocks::components::toggle::{Toggle, ToggleVariant, ToggleSize};
use lucide_dioxus::{Bold, Italic, Underline};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let pressed = use_signal(|| false);

rsx! {
    Toggle {
        pressed: pressed,
        on_pressed_change: move |v| pressed.set(v),

        Bold { class: "size-4" }
    }
}"#;

/// Basic toggle example.
#[component]
pub fn ToggleBasicExample() -> Element {
    let mut pressed = use_signal(|| false);

    rsx! {
        Toggle {
            pressed: pressed,
            on_pressed_change: move |v| pressed.set(v),

            Bold { class: "size-4" }
        }
    }
}

/// Source code for the variants example.
pub const VARIANTS_SOURCE: &str = r#"rsx! {
    div { class: "flex gap-2",
        Toggle {
            variant: ToggleVariant::Default,
            Bold { class: "size-4" }
        }
        Toggle {
            variant: ToggleVariant::Outline,
            Italic { class: "size-4" }
        }
    }
}"#;

/// Toggle variants example.
#[component]
pub fn ToggleVariantsExample() -> Element {
    rsx! {
        div { class: "flex gap-2",
            Toggle {
                variant: ToggleVariant::Default,
                Bold { class: "size-4" }
            }
            Toggle {
                variant: ToggleVariant::Outline,
                Italic { class: "size-4" }
            }
        }
    }
}

/// Source code for the sizes example.
pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-2",
        Toggle {
            size: ToggleSize::Sm,
            Bold { class: "size-4" }
        }
        Toggle {
            size: ToggleSize::Default,
            Bold { class: "size-4" }
        }
        Toggle {
            size: ToggleSize::Lg,
            Bold { class: "size-4" }
        }
    }
}"#;

/// Toggle sizes example.
#[component]
pub fn ToggleSizesExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Toggle {
                size: ToggleSize::Sm,
                Bold { class: "size-4" }
            }
            Toggle {
                size: ToggleSize::Default,
                Bold { class: "size-4" }
            }
            Toggle {
                size: ToggleSize::Lg,
                Bold { class: "size-4" }
            }
        }
    }
}

/// Source code for the with text example.
pub const WITH_TEXT_SOURCE: &str = r#"rsx! {
    Toggle {
        Underline { class: "size-4" }
        "Underline"
    }
}"#;

/// Toggle with text example.
#[component]
pub fn ToggleWithTextExample() -> Element {
    rsx! {
        Toggle {
            Underline { class: "size-4" }
            "Underline"
        }
    }
}

/// Source code for the disabled example.
pub const DISABLED_SOURCE: &str = r#"rsx! {
    Toggle {
        disabled: true,
        Bold { class: "size-4" }
    }
}"#;

/// Disabled toggle example.
#[component]
pub fn ToggleDisabledExample() -> Element {
    rsx! {
        Toggle {
            disabled: true,
            Bold { class: "size-4" }
        }
    }
}
