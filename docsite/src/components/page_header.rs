//! Shared page header primitives matching the v4 reference structure.

use dioxus::prelude::*;

/// Header section wrapper.
#[component]
pub fn PageHeader(#[props(default)] class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        section { class: "border-grid {class}",
            div { class: "container-wrapper",
                div {
                    class: "container flex flex-col items-center gap-2 px-6 py-8 text-center md:py-16 lg:py-20 xl:gap-4",
                    {children}
                }
            }
        }
    }
}

/// Main heading used in page headers.
#[component]
pub fn PageHeaderHeading(#[props(default)] class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        h1 {
            class: "leading-tighter max-w-3xl text-3xl font-semibold tracking-tight text-balance text-primary lg:leading-[1.1] lg:font-semibold xl:text-5xl xl:tracking-tighter {class}",
            {children}
        }
    }
}

/// Supporting description text.
#[component]
pub fn PageHeaderDescription(
    #[props(default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        p {
            class: "max-w-4xl text-base text-balance text-foreground sm:text-lg {class}",
            {children}
        }
    }
}

/// Action row used under page heading/description.
#[component]
pub fn PageActions(#[props(default)] class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        div { class: "flex w-full items-center justify-center gap-2 pt-2 **:data-[slot=button]:shadow-none {class}",
            {children}
        }
    }
}
