//! Item example components.

use dioxus::prelude::*;
use lucide_dioxus::File;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemMedia, ItemMediaVariant,
    ItemSeparator, ItemTitle, ItemVariant,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    ItemGroup {
        Item { variant: ItemVariant::Outline,
            ItemMedia { variant: ItemMediaVariant::Icon,
                File {}
            }
            ItemContent {
                ItemTitle { "Document.pdf" }
                ItemDescription { "Uploaded on March 5, 2026" }
            }
            ItemActions {
                Button { "Download" }
            }
        }
    }
}"##;

/// Basic item example.
#[component]
pub fn ItemBasicExample() -> Element {
    rsx! {
        ItemGroup { class: "max-w-md",
            Item { variant: ItemVariant::Outline,
                ItemMedia { variant: ItemMediaVariant::Icon,
                    File {}
                }
                ItemContent {
                    ItemTitle { "Document.pdf" }
                    ItemDescription { "Uploaded on March 5, 2026" }
                }
                ItemActions {
                    Button { "Download" }
                }
            }
            ItemSeparator {}
            Item { variant: ItemVariant::Outline,
                ItemMedia { variant: ItemMediaVariant::Icon,
                    File {}
                }
                ItemContent {
                    ItemTitle { "Report.xlsx" }
                    ItemDescription { "Uploaded on March 4, 2026" }
                }
                ItemActions {
                    Button { "Download" }
                }
            }
        }
    }
}
