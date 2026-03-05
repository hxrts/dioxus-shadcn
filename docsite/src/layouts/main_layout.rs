//! Main application layout with header - matches shadcn-ui v4 app layout.

use dioxus::prelude::*;
use dioxus_router::{use_route, Outlet};

use crate::components::footer::SiteFooter;
use crate::components::navbar::Navbar;
use crate::Route;

/// Main layout wrapper with navbar - matches shadcn/ui v4 app layout.
#[component]
pub fn MainLayout() -> Element {
    let route = use_route::<Route>();
    let title = match route {
        Route::Home { .. } => "dioxus-shadcn",
        Route::Blocks { .. } => "dioxus-shadcn - Blocks",
        Route::Themes { .. } => "dioxus-shadcn - Themes",
        Route::AuthenticationExample { .. } => "dioxus-shadcn - Authentication",
        Route::DocsPage { .. } => "dioxus-shadcn - Docs",
        Route::Err404 { .. } => "dioxus-shadcn - Not Found",
    };

    rsx! {
        document::Title { "{title}" }

        div {
            class: "group/body relative z-10 flex min-h-svh flex-col bg-background",

            // Top navigation bar
            Navbar {}

            // Main content area
            main { class: "flex flex-1 flex-col",
                Outlet::<Route> {}
            }

            // Footer
            SiteFooter {}
        }
    }
}
