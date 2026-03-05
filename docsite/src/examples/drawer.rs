//! Drawer example components.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Drawer {
        DrawerTrigger {
            Button { "Open Drawer" }
        }
        DrawerContent {
            DrawerHeader {
                DrawerTitle { "Edit Profile" }
                DrawerDescription { "Make changes to your profile here." }
            }
            div { class: "p-4",
                p { "Drawer content goes here." }
            }
            DrawerFooter {
                DrawerClose {
                    Button { "Close" }
                }
            }
        }
    }
}"##;

/// Basic drawer example.
#[component]
pub fn DrawerBasicExample() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger {
                Button { "Open Drawer" }
            }
            DrawerContent {
                DrawerHeader {
                    DrawerTitle { "Edit Profile" }
                    DrawerDescription { "Make changes to your profile here." }
                }
                div { class: "p-4",
                    p { "Drawer content goes here." }
                }
                DrawerFooter {
                    DrawerClose {
                        Button { "Close" }
                    }
                }
            }
        }
    }
}
