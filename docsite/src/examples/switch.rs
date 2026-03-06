//! Switch component examples and source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::label::Label;
use dioxus_shadcn::components::switch::{Switch, SwitchSize};

pub const BASIC_SOURCE: &str = r#"rsx! {
    Switch {
        checked: enabled,
        on_checked_change: move |v| enabled.set(v),
    }
}"#;

#[component]
pub fn SwitchBasicExample() -> Element {
    let mut enabled = use_signal(|| false);

    rsx! {
        Switch {
            checked: enabled,
            on_checked_change: move |v| enabled.set(v),
        }
    }
}

pub const WITH_LABEL_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-2",
        Switch {
            id: "airplane-mode",
            checked: enabled,
            on_checked_change: move |v| enabled.set(v),
        }
        Label { for_id: "airplane-mode", "Airplane Mode" }
    }
}"#;

#[component]
pub fn SwitchWithLabelExample() -> Element {
    let mut enabled = use_signal(|| false);

    rsx! {
        div { class: "flex items-center gap-2",
            Switch {
                id: "airplane-mode",
                checked: enabled,
                on_checked_change: move |v| enabled.set(v),
            }
            Label { for_id: "airplane-mode", "Airplane Mode" }
        }
    }
}

pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-4",
        Switch { size: SwitchSize::Sm, checked: small }
        Switch { size: SwitchSize::Default, checked: medium }
    }
}"#;

#[component]
pub fn SwitchSizesExample() -> Element {
    let small = use_signal(|| true);
    let medium = use_signal(|| true);

    rsx! {
        div { class: "flex items-center gap-4",
            Switch { size: SwitchSize::Sm, checked: small }
            Switch { size: SwitchSize::Default, checked: medium }
        }
    }
}

pub const DISABLED_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-4",
        Switch { disabled: true, checked: use_signal(|| false) }
        Switch { disabled: true, checked: use_signal(|| true) }
    }
}"#;

#[component]
pub fn SwitchDisabledExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Switch { disabled: true, checked: use_signal(|| false) }
            Switch { disabled: true, checked: use_signal(|| true) }
        }
    }
}
