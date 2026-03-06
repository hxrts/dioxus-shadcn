//! Badge component examples and source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::badge::{Badge, BadgeVariant};

pub const VARIANTS_SOURCE: &str = r#"rsx! {
    div { class: "flex flex-wrap gap-2",
        Badge { "Default" }
        Badge { variant: BadgeVariant::Secondary, "Secondary" }
        Badge { variant: BadgeVariant::Destructive, "Destructive" }
        Badge { variant: BadgeVariant::Outline, "Outline" }
        Badge { variant: BadgeVariant::Ghost, "Ghost" }
        Badge { variant: BadgeVariant::Link, "Link" }
    }
}"#;

#[component]
pub fn BadgeVariantsExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            Badge { "Default" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
            Badge { variant: BadgeVariant::Ghost, "Ghost" }
            Badge { variant: BadgeVariant::Link, "Link" }
        }
    }
}

pub const WITH_ICON_SOURCE: &str = r#"rsx! {
    div { class: "flex flex-wrap gap-2",
        Badge {
            lucide_dioxus::Check {}
            "Verified"
        }
        Badge { variant: BadgeVariant::Secondary,
            lucide_dioxus::Clock {}
            "Pending"
        }
        Badge { variant: BadgeVariant::Destructive,
            lucide_dioxus::X {}
            "Failed"
        }
    }
}"#;

#[component]
pub fn BadgeWithIconExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2",
            Badge {
                lucide_dioxus::Check {}
                "Verified"
            }
            Badge { variant: BadgeVariant::Secondary,
                lucide_dioxus::Clock {}
                "Pending"
            }
            Badge { variant: BadgeVariant::Destructive,
                lucide_dioxus::X {}
                "Failed"
            }
        }
    }
}
