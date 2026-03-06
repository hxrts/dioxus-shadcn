//! Announcement badge used in page headers.

use dioxus::prelude::*;
use lucide_dioxus::ArrowRight;
use lumen_blocks::components::badge::{Badge, BadgeVariant};

/// Compact announcement link used by top-level pages.
#[component]
pub fn Announcement() -> Element {
    rsx! {
        Badge { variant: BadgeVariant::Secondary, class: "bg-muted p-0",
            a {
                href: "/docs/changelog/2026-01-rtl",
                class: "inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium text-foreground transition-colors hover:bg-muted/80",
                "RTL Support"
                ArrowRight { class: "size-3" }
            }
        }
    }
}
