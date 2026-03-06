//! Colors section navigation.

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_route;

const COLOR_LINKS: [(&str, &str); 6] = [
    ("Neutral", "neutral"),
    ("Blue", "blue"),
    ("Green", "green"),
    ("Amber", "amber"),
    ("Rose", "rose"),
    ("Purple", "purple"),
];

/// Horizontal color navigation links.
#[component]
pub fn ColorsNav(#[props(default)] class: Option<String>) -> Element {
    let route = use_route::<Route>();
    let class = class.unwrap_or_default();

    let is_colors = matches!(route, Route::Colors { .. });

    rsx! {
        div { class: "flex items-center {class}",
            div { class: "flex max-w-full items-center overflow-x-auto no-scrollbar",
                for (name, slug) in COLOR_LINKS {
                    Link {
                        to: "/colors#{slug}",
                        class: "flex h-7 shrink-0 items-center justify-center px-4 text-center text-base font-medium text-muted-foreground capitalize transition-colors hover:text-primary data-[active=true]:text-primary",
                        "data-active": if is_colors && slug == "neutral" { "true" } else { "false" },
                        "{name}"
                    }
                }
            }
        }
    }
}
