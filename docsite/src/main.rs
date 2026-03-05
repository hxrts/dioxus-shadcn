use dioxus::document;
use dioxus::prelude::*;

mod components;
mod layouts;
mod pages;
use crate::layouts::{DocsLayout, MainLayout};
use crate::pages::{Err404, Home};
use docs::docs;

const FAVICON: Asset = asset!("/assets/lumen-logo-small.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/lumen-logo.png");
const LOGO_SMALL: Asset = asset!("/assets/lumen-logo-small.png");
const PREVIEW_IMAGE: Asset = asset!("/assets/lumen-blocks-preview.jpg");

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[layout(DocsLayout)]
    #[nest("/docs")]
    #[redirect("/", || Route::Docs { child: docs::router::BookRoute::Index { section: Default::default() } })]
    #[child("/")]
    Docs { child: docs::router::BookRoute },
    #[end_nest]
    #[end_layout]
    #[end_layout]
    #[layout(MainLayout)]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Analytics disabled for dioxus-shadcn
    let analytics_script = rsx! {};

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "dioxus-shadcn" }
        document::Meta { property: "og:description", content: "shadcn UI components for Dioxus" }
        document::Meta { property: "og:image", content: PREVIEW_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }

        // Include the analytics script (will be empty if feature is disabled)
        {analytics_script}

        div { class: "min-h-screen bg-background",
            Router::<Route> {}
        }
    }
}
