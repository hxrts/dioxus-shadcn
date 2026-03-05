//! Select component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue,
};

pub const BASIC_SOURCE: &str = r#"rsx! {
    Select {
        SelectTrigger {
            SelectValue { placeholder: "Select a fruit..." }
        }
        SelectContent {
            SelectItem { value: "apple", "Apple" }
            SelectItem { value: "banana", "Banana" }
            SelectItem { value: "orange", "Orange" }
        }
    }
}"#;

#[component]
pub fn SelectBasicExample() -> Element {
    rsx! {
        Select {
            SelectTrigger {
                SelectValue { placeholder: "Select a fruit..." }
            }
            SelectContent {
                SelectItem { value: "apple", "Apple" }
                SelectItem { value: "banana", "Banana" }
                SelectItem { value: "orange", "Orange" }
            }
        }
    }
}

pub const WITH_GROUPS_SOURCE: &str = r#"rsx! {
    Select {
        SelectTrigger {
            SelectValue { placeholder: "Select a food..." }
        }
        SelectContent {
            SelectGroup {
                SelectLabel { "Fruits" }
                SelectItem { value: "apple", "Apple" }
                SelectItem { value: "banana", "Banana" }
            }
            SelectGroup {
                SelectLabel { "Vegetables" }
                SelectItem { value: "carrot", "Carrot" }
                SelectItem { value: "broccoli", "Broccoli" }
            }
        }
    }
}"#;

#[component]
pub fn SelectWithGroupsExample() -> Element {
    rsx! {
        Select {
            SelectTrigger {
                SelectValue { placeholder: "Select a food..." }
            }
            SelectContent {
                SelectGroup {
                    SelectLabel { "Fruits" }
                    SelectItem { value: "apple", "Apple" }
                    SelectItem { value: "banana", "Banana" }
                }
                SelectGroup {
                    SelectLabel { "Vegetables" }
                    SelectItem { value: "carrot", "Carrot" }
                    SelectItem { value: "broccoli", "Broccoli" }
                }
            }
        }
    }
}

pub const DISABLED_SOURCE: &str = r#"rsx! {
    div { class: "flex gap-4",
        Select { disabled: true,
            SelectTrigger {
                SelectValue { placeholder: "Disabled" }
            }
            SelectContent {
                SelectItem { value: "1", "Option 1" }
            }
        }
        Select {
            SelectTrigger {
                SelectValue { placeholder: "With disabled item" }
            }
            SelectContent {
                SelectItem { value: "1", "Option 1" }
                SelectItem { value: "2", disabled: true, "Option 2 (disabled)" }
                SelectItem { value: "3", "Option 3" }
            }
        }
    }
}"#;

#[component]
pub fn SelectDisabledExample() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            Select { disabled: true,
                SelectTrigger {
                    SelectValue { placeholder: "Disabled" }
                }
                SelectContent {
                    SelectItem { value: "1", "Option 1" }
                }
            }
            Select {
                SelectTrigger {
                    SelectValue { placeholder: "With disabled item" }
                }
                SelectContent {
                    SelectItem { value: "1", "Option 1" }
                    SelectItem { value: "2", disabled: true, "Option 2 (disabled)" }
                    SelectItem { value: "3", "Option 3" }
                }
            }
        }
    }
}
