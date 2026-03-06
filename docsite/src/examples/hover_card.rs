//! HoverCard example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::avatar::{Avatar, AvatarFallback, AvatarImage};
use dioxus_shadcn::components::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    HoverCard {
        HoverCardTrigger {
            a {
                class: "underline",
                href: "#",
                "dioxus"
            }
        }
        HoverCardContent {
            div { class: "flex justify-between space-x-4",
                Avatar {
                    AvatarImage { src: "https://github.com/dioxuslabs.png", alt: "Dioxus" }
                    AvatarFallback { "DX" }
                }
                div { class: "space-y-1",
                    h4 { class: "text-sm font-semibold", "dioxus" }
                    p { class: "text-sm",
                        "A Rust framework for building cross-platform user interfaces."
                    }
                    div { class: "flex items-center pt-2",
                        span { class: "text-xs text-muted-foreground", "Joined December 2021" }
                    }
                }
            }
        }
    }
}"##;

/// Basic hover card example.
#[component]
pub fn HoverCardBasicExample() -> Element {
    rsx! {
        HoverCard {
            HoverCardTrigger {
                a {
                    class: "underline",
                    href: "#",
                    "dioxus"
                }
            }
            HoverCardContent {
                div { class: "flex justify-between space-x-4",
                    Avatar {
                        AvatarImage { src: "https://github.com/dioxuslabs.png", alt: "Dioxus" }
                        AvatarFallback { "DX" }
                    }
                    div { class: "space-y-1",
                        h4 { class: "text-sm font-semibold", "dioxus" }
                        p { class: "text-sm",
                            "A Rust framework for building cross-platform user interfaces."
                        }
                        div { class: "flex items-center pt-2",
                            span { class: "text-xs text-muted-foreground", "Joined December 2021" }
                        }
                    }
                }
            }
        }
    }
}
