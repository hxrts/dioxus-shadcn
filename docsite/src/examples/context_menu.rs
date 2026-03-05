//! ContextMenu example components.

use dioxus::prelude::*;
use lumen_blocks::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuShortcut,
    ContextMenuTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    ContextMenu {
        ContextMenuTrigger {
            div {
                class: "flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm",
                "Right click here"
            }
        }
        ContextMenuContent {
            ContextMenuItem { value: "back", index: 0,
                "Back"
                ContextMenuShortcut { "Cmd+[" }
            }
            ContextMenuItem { value: "forward", index: 1, disabled: true,
                "Forward"
                ContextMenuShortcut { "Cmd+]" }
            }
            ContextMenuItem { value: "reload", index: 2,
                "Reload"
                ContextMenuShortcut { "Cmd+R" }
            }
            ContextMenuSeparator {}
            ContextMenuItem { value: "save", index: 3,
                "Save Page As..."
                ContextMenuShortcut { "Cmd+S" }
            }
            ContextMenuItem { value: "print", index: 4,
                "Print..."
                ContextMenuShortcut { "Cmd+P" }
            }
        }
    }
}"##;

/// Basic context menu example.
#[component]
pub fn ContextMenuBasicExample() -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                div {
                    class: "flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm",
                    "Right click here"
                }
            }
            ContextMenuContent {
                ContextMenuItem { value: "back", index: 0,
                    "Back"
                    ContextMenuShortcut { "Cmd+[" }
                }
                ContextMenuItem { value: "forward", index: 1, disabled: true,
                    "Forward"
                    ContextMenuShortcut { "Cmd+]" }
                }
                ContextMenuItem { value: "reload", index: 2,
                    "Reload"
                    ContextMenuShortcut { "Cmd+R" }
                }
                ContextMenuSeparator {}
                ContextMenuItem { value: "save", index: 3,
                    "Save Page As..."
                    ContextMenuShortcut { "Cmd+S" }
                }
                ContextMenuItem { value: "print", index: 4,
                    "Print..."
                    ContextMenuShortcut { "Cmd+P" }
                }
            }
        }
    }
}
