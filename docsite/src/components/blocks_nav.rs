//! Blocks section navigation.

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_route;

const BLOCK_CATEGORIES: [(&str, &str); 4] = [
    ("Featured", "/blocks"),
    ("Dashboard", "/blocks/dashboard"),
    ("Forms", "/blocks/forms"),
    ("People", "/blocks/people"),
];

/// Horizontal category navigation for blocks.
#[component]
pub fn BlocksNav() -> Element {
    let route = use_route::<Route>();

    let current_path = match route {
        Route::Blocks { .. } => "/blocks",
        Route::BlocksCategory { ref category } => {
            if category == "dashboard" {
                "/blocks/dashboard"
            } else if category == "forms" {
                "/blocks/forms"
            } else if category == "people" {
                "/blocks/people"
            } else {
                "/blocks"
            }
        }
        _ => "",
    };

    rsx! {
        div { class: "relative overflow-hidden",
            div { class: "flex max-w-full items-center overflow-x-auto no-scrollbar",
                for (name, href) in BLOCK_CATEGORIES {
                    Link {
                        to: href,
                        class: "flex h-7 shrink-0 items-center justify-center px-4 text-center text-base font-medium text-muted-foreground transition-colors hover:text-primary data-[active=true]:text-primary",
                        "data-active": if current_path == href { "true" } else { "false" },
                        "{name}"
                    }
                }
            }
        }
    }
}
