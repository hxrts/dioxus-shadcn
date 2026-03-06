//! Menubar example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut,
    MenubarTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Menubar {
        MenubarMenu { index: ReadSignal::new(Signal::new(0)),
            MenubarTrigger { "File" }
            MenubarContent {
                MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "new-tab",
                    "New Tab"
                    MenubarShortcut { "Cmd+T" }
                }
                MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "new-window",
                    "New Window"
                    MenubarShortcut { "Cmd+N" }
                }
                MenubarSeparator {}
                MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "share",
                    "Share"
                }
                MenubarSeparator {}
                MenubarItem { index: ReadSignal::new(Signal::new(3)), value: "print",
                    "Print"
                    MenubarShortcut { "Cmd+P" }
                }
            }
        }
        MenubarMenu { index: ReadSignal::new(Signal::new(1)),
            MenubarTrigger { "Edit" }
            MenubarContent {
                MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "undo",
                    "Undo"
                    MenubarShortcut { "Cmd+Z" }
                }
                MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "redo",
                    "Redo"
                    MenubarShortcut { "Shift+Cmd+Z" }
                }
                MenubarSeparator {}
                MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "cut",
                    "Cut"
                }
                MenubarItem { index: ReadSignal::new(Signal::new(3)), value: "copy",
                    "Copy"
                }
                MenubarItem { index: ReadSignal::new(Signal::new(4)), value: "paste",
                    "Paste"
                }
            }
        }
        MenubarMenu { index: ReadSignal::new(Signal::new(2)),
            MenubarTrigger { "View" }
            MenubarContent {
                MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "zoom-in",
                    "Zoom In"
                }
                MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "zoom-out",
                    "Zoom Out"
                }
                MenubarSeparator {}
                MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "fullscreen",
                    "Toggle Fullscreen"
                }
            }
        }
    }
}"##;

/// Basic menubar example.
#[component]
pub fn MenubarBasicExample() -> Element {
    rsx! {
        Menubar {
            MenubarMenu { index: ReadSignal::new(Signal::new(0)),
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "new-tab",
                        "New Tab"
                        MenubarShortcut { "Cmd+T" }
                    }
                    MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "new-window",
                        "New Window"
                        MenubarShortcut { "Cmd+N" }
                    }
                    MenubarSeparator {}
                    MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "share",
                        "Share"
                    }
                    MenubarSeparator {}
                    MenubarItem { index: ReadSignal::new(Signal::new(3)), value: "print",
                        "Print"
                        MenubarShortcut { "Cmd+P" }
                    }
                }
            }
            MenubarMenu { index: ReadSignal::new(Signal::new(1)),
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "undo",
                        "Undo"
                        MenubarShortcut { "Cmd+Z" }
                    }
                    MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "redo",
                        "Redo"
                        MenubarShortcut { "Shift+Cmd+Z" }
                    }
                    MenubarSeparator {}
                    MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "cut",
                        "Cut"
                    }
                    MenubarItem { index: ReadSignal::new(Signal::new(3)), value: "copy",
                        "Copy"
                    }
                    MenubarItem { index: ReadSignal::new(Signal::new(4)), value: "paste",
                        "Paste"
                    }
                }
            }
            MenubarMenu { index: ReadSignal::new(Signal::new(2)),
                MenubarTrigger { "View" }
                MenubarContent {
                    MenubarItem { index: ReadSignal::new(Signal::new(0)), value: "zoom-in",
                        "Zoom In"
                    }
                    MenubarItem { index: ReadSignal::new(Signal::new(1)), value: "zoom-out",
                        "Zoom Out"
                    }
                    MenubarSeparator {}
                    MenubarItem { index: ReadSignal::new(Signal::new(2)), value: "fullscreen",
                        "Toggle Fullscreen"
                    }
                }
            }
        }
    }
}
