//! ScrollArea example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::scroll_area::{
    ScrollArea, ScrollAreaViewport, ScrollBar, ScrollbarOrientation,
};
use dioxus_shadcn::components::separator::Separator;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    ScrollArea {
        height: "200px",
        class: "w-48 rounded-md border",

        ScrollAreaViewport {
            div { class: "p-4",
                h4 { class: "mb-4 text-sm font-medium leading-none", "Tags" }
                for tag in TAGS {
                    div {
                        div { class: "text-sm", "{tag}" }
                        Separator { class: "my-2" }
                    }
                }
            }
        }
        ScrollBar { orientation: ScrollbarOrientation::Vertical }
    }
}"#;

const TAGS: &[&str] = &[
    "v1.0.0", "v0.9.0", "v0.8.5", "v0.8.0", "v0.7.3", "v0.7.0", "v0.6.0", "v0.5.2", "v0.5.0",
    "v0.4.0",
];

/// Basic scroll area example.
#[component]
pub fn ScrollAreaBasicExample() -> Element {
    rsx! {
        ScrollArea {
            height: "200px".to_string(),
            class: "w-48 rounded-md border",

            ScrollAreaViewport {
                div { class: "p-4",
                    h4 { class: "mb-4 text-sm font-medium leading-none", "Tags" }
                    for tag in TAGS {
                        div {
                            div { class: "text-sm", "{tag}" }
                            Separator { class: "my-2" }
                        }
                    }
                }
            }
            ScrollBar { orientation: ScrollbarOrientation::Vertical }
        }
    }
}

/// Source code for the horizontal example.
pub const HORIZONTAL_SOURCE: &str = r#"rsx! {
    ScrollArea {
        width: "300px",
        class: "rounded-md border",

        ScrollAreaViewport {
            div { class: "flex gap-4 p-4",
                for i in 1..=10 {
                    div {
                        class: "flex h-[150px] w-[200px] shrink-0 items-center justify-center rounded-md border bg-muted",
                        "Image {i}"
                    }
                }
            }
        }
        ScrollBar { orientation: ScrollbarOrientation::Horizontal }
    }
}"#;

/// Horizontal scroll area example.
#[component]
pub fn ScrollAreaHorizontalExample() -> Element {
    rsx! {
        ScrollArea {
            width: "300px".to_string(),
            class: "rounded-md border",

            ScrollAreaViewport {
                class: "whitespace-nowrap",
                div { class: "flex gap-4 p-4",
                    for i in 1..=10 {
                        div {
                            class: "flex h-[150px] w-[200px] shrink-0 items-center justify-center rounded-md border bg-muted",
                            "Image {i}"
                        }
                    }
                }
            }
            ScrollBar { orientation: ScrollbarOrientation::Horizontal }
        }
    }
}
