//! Kbd example components.

use dioxus::prelude::*;
use lumen_blocks::components::kbd::{Kbd, KbdGroup};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    div { class: "flex items-center gap-4",
        Kbd { "K" }

        KbdGroup {
            Kbd { "Cmd" }
            Kbd { "K" }
        }

        KbdGroup {
            Kbd { "Ctrl" }
            Kbd { "Shift" }
            Kbd { "P" }
        }
    }
}"##;

/// Basic kbd example.
#[component]
pub fn KbdBasicExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Kbd { "K" }

            KbdGroup {
                Kbd { "Cmd" }
                Kbd { "K" }
            }

            KbdGroup {
                Kbd { "Ctrl" }
                Kbd { "Shift" }
                Kbd { "P" }
            }
        }
    }
}
