//! Examples section navigation.

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_route;

const EXAMPLE_LINKS: [(&str, &str); 6] = [
    ("Examples", "/"),
    ("Dashboard", "/examples/dashboard"),
    ("Tasks", "/examples/tasks"),
    ("Playground", "/examples/playground"),
    ("Authentication", "/examples/authentication"),
    ("RTL", "/examples/rtl"),
];

/// Horizontal example navigation links.
#[component]
pub fn ExamplesNav(#[props(default)] class: Option<String>) -> Element {
    let route = use_route::<Route>();
    let class = class.unwrap_or_default();

    let current_path = match route {
        Route::Home { .. } => "/",
        Route::DashboardExample { .. } => "/examples/dashboard",
        Route::TasksExample { .. } => "/examples/tasks",
        Route::PlaygroundExample { .. } => "/examples/playground",
        Route::AuthenticationExample { .. } => "/examples/authentication",
        Route::RtlExample { .. } => "/examples/rtl",
        _ => "",
    };

    rsx! {
        div { class: "flex items-center {class}",
            div { class: "flex max-w-[96%] items-center overflow-x-auto no-scrollbar md:max-w-[600px] lg:max-w-none",
                for (name, href) in EXAMPLE_LINKS {
                    Link {
                        to: href,
                        class: "flex h-7 shrink-0 items-center justify-center gap-2 px-4 text-center text-base font-medium text-muted-foreground transition-colors hover:text-primary data-[active=true]:text-primary",
                        "data-active": if current_path == href { "true" } else { "false" },
                        "{name}"
                        if name == "RTL" {
                            span { class: "size-2 rounded-full bg-blue-500", title: "New" }
                        }
                    }
                }
            }
        }
    }
}
