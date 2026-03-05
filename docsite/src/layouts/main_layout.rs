use dioxus::prelude::*;
use dioxus_router::{use_route, Outlet};

use crate::components::navbar::Navbar;
use crate::Route;

#[component]
pub fn MainLayout() -> Element {
    let route = use_route::<Route>();
    let title = match route {
        Route::Home { .. } => "dioxus-shadcn",
        Route::DocsPage { .. } => "dioxus-shadcn - Docs",
        Route::Err404 { .. } => "dioxus-shadcn - Not Found",
    };

    rsx! {
        document::Title { "{title}" }

        div {
            class: "relative flex min-h-screen flex-col bg-background",
            // CSS variable for header height
            style: "--header-height: 56px;",

            // Top navigation bar
            Navbar {}

            // Main content area - child routes render here
            Outlet::<Route> {}
        }
    }
}
