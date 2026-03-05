//! ButtonGroup example components.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::button_group::{ButtonGroup, ButtonGroupOrientation};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    ButtonGroup {
        Button { "Left" }
        Button { "Center" }
        Button { "Right" }
    }
}"##;

/// Source code for vertical example.
pub const VERTICAL_SOURCE: &str = r##"rsx! {
    ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
        Button { "Top" }
        Button { "Middle" }
        Button { "Bottom" }
    }
}"##;

/// Basic button group example.
#[component]
pub fn ButtonGroupBasicExample() -> Element {
    rsx! {
        ButtonGroup {
            Button { "Left" }
            Button { "Center" }
            Button { "Right" }
        }
    }
}

/// Vertical button group example.
#[component]
pub fn ButtonGroupVerticalExample() -> Element {
    rsx! {
        ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
            Button { "Top" }
            Button { "Middle" }
            Button { "Bottom" }
        }
    }
}
