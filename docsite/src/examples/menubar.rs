//! Menubar example components.

use dioxus::prelude::*;
use lumen_blocks::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut,
    MenubarTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"let file_index = use_signal(|| 0_usize);
let edit_index = use_signal(|| 1_usize);

rsx! {
    Menubar {
        MenubarMenu { index: file_index.into(),
            MenubarTrigger { "File" }
            MenubarContent {
                MenubarItem { index: use_signal(|| 0_usize).into(), value: "new-tab",
                    "New Tab"
                    MenubarShortcut { "Cmd+T" }
                }
                MenubarItem { index: use_signal(|| 1_usize).into(), value: "new-window",
                    "New Window"
                    MenubarShortcut { "Cmd+N" }
                }
                MenubarSeparator {}
                MenubarItem { index: use_signal(|| 2_usize).into(), value: "print",
                    "Print..."
                    MenubarShortcut { "Cmd+P" }
                }
            }
        }
        MenubarMenu { index: edit_index.into(),
            MenubarTrigger { "Edit" }
            MenubarContent {
                MenubarItem { index: use_signal(|| 0_usize).into(), value: "undo",
                    "Undo"
                    MenubarShortcut { "Cmd+Z" }
                }
                MenubarItem { index: use_signal(|| 1_usize).into(), value: "redo",
                    "Redo"
                    MenubarShortcut { "Shift+Cmd+Z" }
                }
            }
        }
    }
}"##;

/// Basic menubar example.
#[component]
pub fn MenubarBasicExample() -> Element {
    let file_index = use_signal(|| 0_usize);
    let edit_index = use_signal(|| 1_usize);

    rsx! {
        Menubar {
            MenubarMenu { index: file_index.into(),
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarItem { index: use_signal(|| 0_usize).into(), value: "new-tab",
                        "New Tab"
                        MenubarShortcut { "Cmd+T" }
                    }
                    MenubarItem { index: use_signal(|| 1_usize).into(), value: "new-window",
                        "New Window"
                        MenubarShortcut { "Cmd+N" }
                    }
                    MenubarSeparator {}
                    MenubarItem { index: use_signal(|| 2_usize).into(), value: "print",
                        "Print..."
                        MenubarShortcut { "Cmd+P" }
                    }
                }
            }
            MenubarMenu { index: edit_index.into(),
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarItem { index: use_signal(|| 0_usize).into(), value: "undo",
                        "Undo"
                        MenubarShortcut { "Cmd+Z" }
                    }
                    MenubarItem { index: use_signal(|| 1_usize).into(), value: "redo",
                        "Redo"
                        MenubarShortcut { "Shift+Cmd+Z" }
                    }
                }
            }
        }
    }
}
