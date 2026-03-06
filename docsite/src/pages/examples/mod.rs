//! Example pages module.

use crate::components::{
    Announcement, ExamplesNav, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
    PageNav, ThemeSelector,
};
use dioxus::prelude::*;

pub mod authentication;
pub mod dashboard;
pub mod playground;
pub mod rtl;
pub mod tasks;

pub use authentication::AuthenticationExample;
pub use dashboard::DashboardExample;
pub use playground::PlaygroundExample;
pub use rtl::RtlExample;
pub use tasks::TasksExample;

const TITLE: &str = "The Foundation for your Design System";
const DESCRIPTION: &str = "A set of beautifully designed components that you can customize, extend, and build on. Start here then make it your own. Open Source. Open Code.";

/// Shared layout shell for all `/examples/*` routes.
#[component]
pub fn ExamplesShell(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col",
            PageHeader {
                Announcement {}
                PageHeaderHeading { class: "max-w-4xl", "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    Link {
                        to: "/docs/installation",
                        class: "inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Get Started"
                    }
                    Link {
                        to: "/docs/components",
                        class: "inline-flex h-8 items-center justify-center rounded-md px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "View Components"
                    }
                }
            }

            PageNav { id: "examples", class: "hidden md:flex",
                ExamplesNav { class: "flex-1 overflow-hidden [&>a:first-child]:text-primary" }
                ThemeSelector { class: "mr-4 hidden md:flex" }
            }

            div { class: "container-wrapper flex flex-1 flex-col section-soft pb-6",
                div { class: "container flex flex-1 scroll-mt-20 flex-col theme-container",
                    div {
                        class: "flex flex-col overflow-hidden rounded-lg border bg-background bg-clip-padding has-[[data-slot=rtl-components]]:overflow-visible has-[[data-slot=rtl-components]]:border-0 has-[[data-slot=rtl-components]]:bg-transparent md:flex-1 xl:rounded-xl",
                        {children}
                    }
                }
            }
        }
    }
}
