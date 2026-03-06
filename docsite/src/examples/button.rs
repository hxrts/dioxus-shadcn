//! Button component examples with embedded source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::button::{Button, ButtonSize, ButtonVariant};
use lucide_dioxus::{ArrowLeft, ArrowRight, Pencil, Plus, Search, Trash, X};

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const VARIANTS_SOURCE: &str = r#"use dioxus_shadcn::components::button::{Button, ButtonVariant};

rsx! {
    div { class: "flex flex-wrap gap-2.5 items-center",
        Button { variant: ButtonVariant::Default, "Default" }
        Button { variant: ButtonVariant::Secondary, "Secondary" }
        Button { variant: ButtonVariant::Outline, "Outline" }
        Button { variant: ButtonVariant::Ghost, "Ghost" }
        Button { variant: ButtonVariant::Link, "Link" }
        Button { variant: ButtonVariant::Destructive, "Destructive" }
    }
}"#;

pub const SIZES_SOURCE: &str = r#"use dioxus_shadcn::components::button::{Button, ButtonSize, ButtonVariant};

rsx! {
    div { class: "flex flex-wrap gap-2.5 items-center",
        Button { variant: ButtonVariant::Default, size: ButtonSize::Small, "Small" }
        Button { variant: ButtonVariant::Default, size: ButtonSize::Medium, "Medium" }
        Button { variant: ButtonVariant::Default, size: ButtonSize::Large, "Large" }
    }
}"#;

pub const STATES_SOURCE: &str = r#"use dioxus_shadcn::components::button::{Button, ButtonVariant};

let mut loading = use_signal(|| false);

rsx! {
    div { class: "flex flex-wrap gap-2.5 items-center",
        Button { variant: ButtonVariant::Default, disabled: true, "Disabled" }
        Button { variant: ButtonVariant::Default, loading: loading(), "Loading" }
        Button {
            variant: ButtonVariant::Secondary,
            on_click: move |_| loading.toggle(),
            "Toggle Loading"
        }
    }
}"#;

pub const ICONS_SOURCE: &str = r#"use lucide_dioxus::{ArrowLeft, ArrowRight};
use dioxus_shadcn::components::button::{Button, ButtonVariant};

rsx! {
    div { class: "flex flex-wrap gap-2.5 items-center",
        Button {
            variant: ButtonVariant::Default,
            icon_left: rsx! { ArrowLeft { size: 16 } },
            "Left Icon"
        }
        Button {
            variant: ButtonVariant::Default,
            icon_right: rsx! { ArrowRight { size: 16 } },
            "Right Icon"
        }
    }
}"#;

pub const ICON_BUTTONS_SOURCE: &str = r#"use lucide_dioxus::{Plus, Pencil, Trash, Search, X};
use dioxus_shadcn::components::button::{Button, ButtonVariant};

rsx! {
    div { class: "flex flex-wrap gap-2.5 items-center",
        Button {
            variant: ButtonVariant::Default,
            is_icon_button: true,
            aria_label: Some("Add item".to_string()),
            Plus { size: 20 }
        }
        Button {
            variant: ButtonVariant::Secondary,
            is_icon_button: true,
            aria_label: Some("Edit".to_string()),
            Pencil { size: 20 }
        }
        // ... more icon buttons
    }
}"#;

pub const FULL_WIDTH_SOURCE: &str = r#"use dioxus_shadcn::components::button::{Button, ButtonVariant};

rsx! {
    Button {
        variant: ButtonVariant::Default,
        full_width: true,
        "Full Width Button"
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn ButtonVariantsExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button { variant: ButtonVariant::Default, "Default" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Link, "Link" }
            Button { variant: ButtonVariant::Destructive, "Destructive" }
        }
    }
}

#[component]
pub fn ButtonSizesExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button { variant: ButtonVariant::Default, size: ButtonSize::Small, "Small" }
            Button { variant: ButtonVariant::Default, size: ButtonSize::Medium, "Medium" }
            Button { variant: ButtonVariant::Default, size: ButtonSize::Large, "Large" }
        }
    }
}

#[component]
pub fn ButtonStatesExample() -> Element {
    let mut loading = use_signal(|| false);

    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button { variant: ButtonVariant::Default, disabled: true, "Disabled" }
            Button { variant: ButtonVariant::Default, loading: loading(), "Loading" }
            Button {
                variant: ButtonVariant::Secondary,
                on_click: move |_| loading.toggle(),
                "Toggle Loading"
            }
        }
    }
}

#[component]
pub fn ButtonWithIconsExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button {
                variant: ButtonVariant::Default,
                icon_left: rsx! { ArrowLeft { size: 16 } },
                "Left Icon"
            }
            Button {
                variant: ButtonVariant::Default,
                icon_right: rsx! { ArrowRight { size: 16 } },
                "Right Icon"
            }
        }
    }
}

#[component]
pub fn IconButtonsExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2.5 items-center",
            Button {
                variant: ButtonVariant::Default,
                is_icon_button: true,
                aria_label: Some("Add item".to_string()),
                Plus { size: 20 }
            }
            Button {
                variant: ButtonVariant::Secondary,
                is_icon_button: true,
                aria_label: Some("Edit item".to_string()),
                Pencil { size: 20 }
            }
            Button {
                variant: ButtonVariant::Outline,
                is_icon_button: true,
                aria_label: Some("Delete item".to_string()),
                Trash { size: 20 }
            }
            Button {
                variant: ButtonVariant::Ghost,
                is_icon_button: true,
                aria_label: Some("Search".to_string()),
                Search { size: 20 }
            }
            Button {
                variant: ButtonVariant::Destructive,
                is_icon_button: true,
                aria_label: Some("Close".to_string()),
                X { size: 20 }
            }
        }
    }
}

#[component]
pub fn FullWidthButtonExample() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            Button {
                variant: ButtonVariant::Default,
                full_width: true,
                "Full Width Button"
            }
        }
    }
}
