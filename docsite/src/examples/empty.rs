//! Empty example components.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use lucide_dioxus::Inbox;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Empty {
        EmptyMedia { variant: EmptyMediaVariant::Icon,
            Inbox {}
        }
        EmptyHeader {
            EmptyTitle { "No results found" }
            EmptyDescription {
                "Try adjusting your search or filter to find what you're looking for."
            }
        }
        EmptyContent {
            Button { "Clear filters" }
        }
    }
}"##;

/// Basic empty state example.
#[component]
pub fn EmptyBasicExample() -> Element {
    rsx! {
        Empty {
            EmptyMedia { variant: EmptyMediaVariant::Icon,
                Inbox {}
            }
            EmptyHeader {
                EmptyTitle { "No results found" }
                EmptyDescription {
                    "Try adjusting your search or filter to find what you're looking for."
                }
            }
            EmptyContent {
                Button { "Clear filters" }
            }
        }
    }
}
