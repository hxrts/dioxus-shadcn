//! Skeleton example components.

use dioxus::prelude::*;
use lumen_blocks::components::skeleton::Skeleton;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    Skeleton { class: "h-4 w-[250px]" }
}"#;

/// Basic skeleton example.
#[component]
pub fn SkeletonBasicExample() -> Element {
    rsx! {
        Skeleton { class: "h-4 w-[250px]" }
    }
}

/// Source code for the card example.
pub const CARD_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center space-x-4",
        Skeleton { class: "h-12 w-12 rounded-full" }
        div { class: "space-y-2",
            Skeleton { class: "h-4 w-[250px]" }
            Skeleton { class: "h-4 w-[200px]" }
        }
    }
}"#;

/// Card skeleton example.
#[component]
pub fn SkeletonCardExample() -> Element {
    rsx! {
        div { class: "flex items-center space-x-4",
            Skeleton { class: "h-12 w-12 rounded-full" }
            div { class: "space-y-2",
                Skeleton { class: "h-4 w-[250px]" }
                Skeleton { class: "h-4 w-[200px]" }
            }
        }
    }
}

/// Source code for the text block example.
pub const TEXT_BLOCK_SOURCE: &str = r#"rsx! {
    div { class: "space-y-2",
        Skeleton { class: "h-4 w-full" }
        Skeleton { class: "h-4 w-full" }
        Skeleton { class: "h-4 w-3/4" }
    }
}"#;

/// Text block skeleton example.
#[component]
pub fn SkeletonTextBlockExample() -> Element {
    rsx! {
        div { class: "space-y-2",
            Skeleton { class: "h-4 w-full" }
            Skeleton { class: "h-4 w-full" }
            Skeleton { class: "h-4 w-3/4" }
        }
    }
}
