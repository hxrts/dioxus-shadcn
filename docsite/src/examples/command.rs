//! Command example components.

use dioxus::prelude::*;
use lumen_blocks::components::command::{
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Command {
        class: "rounded-lg border shadow-md",
        CommandInput { placeholder: "Type a command or search..." }
        CommandList {
            CommandEmpty { "No results found." }
            CommandGroup { heading: "Suggestions",
                CommandItem { value: "calendar", "Calendar" }
                CommandItem { value: "search", "Search Emoji" }
                CommandItem { value: "calculator", "Calculator" }
            }
            CommandGroup { heading: "Settings",
                CommandItem { value: "profile",
                    "Profile"
                    CommandShortcut { "Cmd+P" }
                }
                CommandItem { value: "billing",
                    "Billing"
                    CommandShortcut { "Cmd+B" }
                }
                CommandItem { value: "settings",
                    "Settings"
                    CommandShortcut { "Cmd+S" }
                }
            }
        }
    }
}"##;

/// Basic command example.
#[component]
pub fn CommandBasicExample() -> Element {
    rsx! {
        Command {
            class: "rounded-lg border shadow-md",
            CommandInput { placeholder: "Type a command or search..." }
            CommandList {
                CommandEmpty { "No results found." }
                CommandGroup { heading: "Suggestions",
                    CommandItem { value: "calendar", "Calendar" }
                    CommandItem { value: "search", "Search Emoji" }
                    CommandItem { value: "calculator", "Calculator" }
                }
                CommandGroup { heading: "Settings",
                    CommandItem { value: "profile",
                        "Profile"
                        CommandShortcut { "Cmd+P" }
                    }
                    CommandItem { value: "billing",
                        "Billing"
                        CommandShortcut { "Cmd+B" }
                    }
                    CommandItem { value: "settings",
                        "Settings"
                        CommandShortcut { "Cmd+S" }
                    }
                }
            }
        }
    }
}
