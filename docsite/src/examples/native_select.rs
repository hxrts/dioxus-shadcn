//! NativeSelect example components.

use dioxus::prelude::*;
use lumen_blocks::components::native_select::{NativeSelect, NativeSelectOptGroup, NativeSelectOption};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    NativeSelect {
        on_change: move |value| println!("Selected: {value}"),

        NativeSelectOption { value: "", "Select a fruit" }
        NativeSelectOption { value: "apple", "Apple" }
        NativeSelectOption { value: "banana", "Banana" }
        NativeSelectOption { value: "orange", "Orange" }
    }
}"##;

/// Source code for groups example.
pub const GROUPS_SOURCE: &str = r##"rsx! {
    NativeSelect {
        NativeSelectOption { value: "", "Select a food" }
        NativeSelectOptGroup { label: "Fruits",
            NativeSelectOption { value: "apple", "Apple" }
            NativeSelectOption { value: "banana", "Banana" }
        }
        NativeSelectOptGroup { label: "Vegetables",
            NativeSelectOption { value: "carrot", "Carrot" }
            NativeSelectOption { value: "broccoli", "Broccoli" }
        }
    }
}"##;

/// Basic native select example.
#[component]
pub fn NativeSelectBasicExample() -> Element {
    rsx! {
        div { class: "max-w-xs",
            NativeSelect {
                NativeSelectOption { value: "", "Select a fruit" }
                NativeSelectOption { value: "apple", "Apple" }
                NativeSelectOption { value: "banana", "Banana" }
                NativeSelectOption { value: "orange", "Orange" }
            }
        }
    }
}

/// Native select with groups example.
#[component]
pub fn NativeSelectGroupsExample() -> Element {
    rsx! {
        div { class: "max-w-xs",
            NativeSelect {
                NativeSelectOption { value: "", "Select a food" }
                NativeSelectOptGroup { label: "Fruits",
                    NativeSelectOption { value: "apple", "Apple" }
                    NativeSelectOption { value: "banana", "Banana" }
                }
                NativeSelectOptGroup { label: "Vegetables",
                    NativeSelectOption { value: "carrot", "Carrot" }
                    NativeSelectOption { value: "broccoli", "Broccoli" }
                }
            }
        }
    }
}
