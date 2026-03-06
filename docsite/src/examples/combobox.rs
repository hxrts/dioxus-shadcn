//! Combobox example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::combobox::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Combobox {
        ComboboxInput { placeholder: "Select a framework..." }
        ComboboxContent {
            ComboboxList {
                ComboboxEmpty { "No framework found." }
                ComboboxItem { value: "react", label: "React", "React" }
                ComboboxItem { value: "vue", label: "Vue", "Vue" }
                ComboboxItem { value: "angular", label: "Angular", "Angular" }
                ComboboxItem { value: "svelte", label: "Svelte", "Svelte" }
            }
        }
    }
}"##;

/// Basic combobox example.
#[component]
pub fn ComboboxBasicExample() -> Element {
    rsx! {
        div { class: "max-w-xs",
            Combobox {
                ComboboxInput { placeholder: "Select a framework..." }
                ComboboxContent {
                    ComboboxList {
                        ComboboxEmpty { "No framework found." }
                        ComboboxItem { value: "react", label: "React", "React" }
                        ComboboxItem { value: "vue", label: "Vue", "Vue" }
                        ComboboxItem { value: "angular", label: "Angular", "Angular" }
                        ComboboxItem { value: "svelte", label: "Svelte", "Svelte" }
                    }
                }
            }
        }
    }
}
