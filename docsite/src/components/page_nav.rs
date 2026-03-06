//! Shared page navigation row.

use dioxus::prelude::*;

/// Secondary navigation container shown below page headers.
#[component]
pub fn PageNav(
    #[props(default)] id: Option<String>,
    #[props(default)] class: Option<String>,
    children: Element,
) -> Element {
    let id = id.unwrap_or_default();
    let class = class.unwrap_or_default();

    rsx! {
        div { id: "{id}", class: "container-wrapper scroll-mt-24 {class}",
            div { class: "container flex items-center justify-between gap-4 py-4",
                {children}
            }
        }
    }
}
