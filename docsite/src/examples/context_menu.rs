//! ContextMenu example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuLabel, ContextMenuSeparator,
    ContextMenuShortcut, ContextMenuTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    ContextMenu {
        ContextMenuTrigger {
            div { class: "flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm",
                "Right click here"
            }
        }
        ContextMenuContent {
            ContextMenuLabel { "My Account" }
            ContextMenuSeparator {}
            ContextMenuItem { value: "profile",
                "Profile"
                ContextMenuShortcut { "Shift+Cmd+P" }
            }
            ContextMenuItem { value: "billing",
                "Billing"
                ContextMenuShortcut { "Cmd+B" }
            }
            ContextMenuItem { value: "settings",
                "Settings"
                ContextMenuShortcut { "Cmd+," }
            }
            ContextMenuSeparator {}
            ContextMenuItem { value: "logout",
                "Log out"
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
                div { class: "flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm",
                    "Right click here"
                }
            }
            ContextMenuContent {
                ContextMenuLabel { "My Account" }
                ContextMenuSeparator {}
                ContextMenuItem { value: "profile",
                    "Profile"
                    ContextMenuShortcut { "Shift+Cmd+P" }
                }
                ContextMenuItem { value: "billing",
                    "Billing"
                    ContextMenuShortcut { "Cmd+B" }
                }
                ContextMenuItem { value: "settings",
                    "Settings"
                    ContextMenuShortcut { "Cmd+," }
                }
                ContextMenuSeparator {}
                ContextMenuItem { value: "logout",
                    "Log out"
                }
            }
        }
    }
}
