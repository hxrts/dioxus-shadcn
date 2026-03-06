//! Resizable example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::resizable::{
    ResizableDirection, ResizableHandle, ResizablePanel, ResizablePanelGroup,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    ResizablePanelGroup { direction: ResizableDirection::Horizontal, class: "min-h-[200px] rounded-lg border",
        ResizablePanel { default_size: 50.0,
            div { class: "flex h-full items-center justify-center p-6",
                "Panel One"
            }
        }
        ResizableHandle { with_handle: true }
        ResizablePanel { default_size: 50.0,
            div { class: "flex h-full items-center justify-center p-6",
                "Panel Two"
            }
        }
    }
}"##;

/// Basic resizable example.
#[component]
pub fn ResizableBasicExample() -> Element {
    rsx! {
        ResizablePanelGroup { direction: ResizableDirection::Horizontal, class: "min-h-[200px] rounded-lg border",
            ResizablePanel { default_size: 50.0,
                div { class: "flex h-full items-center justify-center p-6",
                    "Panel One"
                }
            }
            ResizableHandle { with_handle: true }
            ResizablePanel { default_size: 50.0,
                div { class: "flex h-full items-center justify-center p-6",
                    "Panel Two"
                }
            }
        }
    }
}
