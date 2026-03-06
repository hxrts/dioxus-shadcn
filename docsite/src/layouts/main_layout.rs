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
        Route::BlocksCategory { .. } => "dioxus-shadcn - Blocks",
        Route::Colors { .. } => "dioxus-shadcn - Colors",
        Route::Charts { .. } => "dioxus-shadcn - Charts",
        Route::ChartType { .. } => "dioxus-shadcn - Charts",
        Route::Themes { .. } => "dioxus-shadcn - Themes",
        Route::DashboardExample { .. } => "dioxus-shadcn - Dashboard Example",
        Route::TasksExample { .. } => "dioxus-shadcn - Tasks Example",
        Route::PlaygroundExample { .. } => "dioxus-shadcn - Playground Example",
        Route::AuthenticationExample { .. } => "dioxus-shadcn - Authentication",
        Route::RtlExample { .. } => "dioxus-shadcn - RTL Example",
        Route::DocsPage { .. } => "dioxus-shadcn - Docs",
        Route::Err404 { .. } => "dioxus-shadcn - Not Found",
    };

    rsx! {
        document::Title { "{title}" }

        div {
            "data-slot": "layout",
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
