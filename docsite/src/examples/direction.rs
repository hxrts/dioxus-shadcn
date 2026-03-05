//! Direction example components.

use dioxus::prelude::*;
use lumen_blocks::components::direction::{Direction, DirectionProvider};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    div { class: "flex gap-8",
        DirectionProvider { direction: Direction::Ltr,
            div { class: "p-4 border rounded",
                p { "Left to right text" }
                p { class: "text-muted-foreground", "This is LTR content" }
            }
        }

        DirectionProvider { direction: Direction::Rtl,
            div { class: "p-4 border rounded",
                p { "Right to left text" }
                p { class: "text-muted-foreground", "This is RTL content" }
            }
        }
    }
}"##;

/// Basic direction example.
#[component]
pub fn DirectionBasicExample() -> Element {
    rsx! {
        div { class: "flex gap-8",
            DirectionProvider { direction: Direction::Ltr,
                div { class: "p-4 border rounded",
                    p { "Left to right text" }
                    p { class: "text-muted-foreground", "This is LTR content" }
                }
            }

            DirectionProvider { direction: Direction::Rtl,
                div { class: "p-4 border rounded",
                    p { "Right to left text" }
                    p { class: "text-muted-foreground", "This is RTL content" }
                }
            }
        }
    }
}
