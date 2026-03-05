use dioxus::document;
use dioxus::prelude::*;

mod components;
mod examples;
mod layouts;
mod pages;

use crate::layouts::MainLayout;
use crate::pages::{Err404, Home};

const FAVICON: Asset = asset!("/assets/lumen-logo-small.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const LOGO: Asset = asset!("/assets/lumen-logo.png");
pub const LOGO_SMALL: Asset = asset!("/assets/lumen-logo-small.png");
const PREVIEW_IMAGE: Asset = asset!("/assets/lumen-blocks-preview.jpg");

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[route("/docs/:..segments")]
    DocsPage { segments: Vec<String> },
    #[end_layout]

    #[layout(MainLayout)]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

/// Docs page wrapper that renders the docs layout.
#[component]
fn DocsPage(segments: Vec<String>) -> Element {
    rsx! {
        layouts::DocsLayout { segments }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "dioxus-shadcn" }
        document::Meta { property: "og:description", content: "shadcn UI components for Dioxus" }
        document::Meta { property: "og:image", content: PREVIEW_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }

        div { class: "min-h-screen bg-background",
            Router::<Route> {}
        }
    }
}
