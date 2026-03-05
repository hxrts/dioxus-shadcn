//! Tooltip component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::tooltip::{Tooltip, TooltipProvider, TooltipSide};

pub const BASIC_SOURCE: &str = r#"rsx! {
    Tooltip {
        content: "Add to library",
        Button { "Hover me" }
    }
}"#;

#[component]
pub fn TooltipBasicExample() -> Element {
    rsx! {
        Tooltip {
            content: "Add to library",
            Button { "Hover me" }
        }
    }
}

pub const SIDES_SOURCE: &str = r#"rsx! {
    div { class: "flex gap-4",
        Tooltip { content: "Top", side: TooltipSide::Top,
            Button { "Top" }
        }
        Tooltip { content: "Right", side: TooltipSide::Right,
            Button { "Right" }
        }
        Tooltip { content: "Bottom", side: TooltipSide::Bottom,
            Button { "Bottom" }
        }
        Tooltip { content: "Left", side: TooltipSide::Left,
            Button { "Left" }
        }
    }
}"#;

#[component]
pub fn TooltipSidesExample() -> Element {
    rsx! {
        div { class: "flex gap-4",
            Tooltip { content: "Top", side: TooltipSide::Top,
                Button { "Top" }
            }
            Tooltip { content: "Right", side: TooltipSide::Right,
                Button { "Right" }
            }
            Tooltip { content: "Bottom", side: TooltipSide::Bottom,
                Button { "Bottom" }
            }
            Tooltip { content: "Left", side: TooltipSide::Left,
                Button { "Left" }
            }
        }
    }
}

pub const WITH_PROVIDER_SOURCE: &str = r#"rsx! {
    TooltipProvider { delay_ms: 400,
        div { class: "flex gap-4",
            Tooltip { content: "First", Button { "One" } }
            Tooltip { content: "Second", Button { "Two" } }
        }
    }
}"#;

#[component]
pub fn TooltipWithProviderExample() -> Element {
    rsx! {
        TooltipProvider { delay_ms: 400,
            div { class: "flex gap-4",
                Tooltip { content: "First", Button { "One" } }
                Tooltip { content: "Second", Button { "Two" } }
            }
        }
    }
}

pub const NO_ARROW_SOURCE: &str = r#"rsx! {
    Tooltip {
        content: "No arrow tooltip",
        show_arrow: false,
        Button { "No Arrow" }
    }
}"#;

#[component]
pub fn TooltipNoArrowExample() -> Element {
    rsx! {
        Tooltip {
            content: "No arrow tooltip",
            show_arrow: false,
            Button { "No Arrow" }
        }
    }
}
