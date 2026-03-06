//! Announcement badge used in page headers.

use dioxus::prelude::*;
use dioxus_shadcn::components::badge::{Badge, BadgeVariant};
use lucide_dioxus::ArrowRight;

/// Compact announcement link used by top-level pages.
#[component]
pub fn Announcement() -> Element {
    rsx! {
        Badge { variant: BadgeVariant::Secondary, class: "bg-muted",
            Link {
                to: "/docs/changelog/2026-01-rtl",
                class: "inline-flex items-center gap-1.5",
                "RTL Support"
                ArrowRight { class: "size-3" }
            }
        }
    }
}
