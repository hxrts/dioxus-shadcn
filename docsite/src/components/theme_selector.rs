//! Compact theme selector used in page nav rows.

use crate::components::ThemeToggle;
use dioxus::prelude::*;
use lumen_blocks::components::{
    native_select::{NativeSelect, NativeSelectOption, NativeSelectSize},
    separator::{Separator, SeparatorOrientation},
};

const THEMES: [(&str, &str); 8] = [
    ("neutral", "Neutral"),
    ("zinc", "Zinc"),
    ("slate", "Slate"),
    ("stone", "Stone"),
    ("gray", "Gray"),
    ("blue", "Blue"),
    ("green", "Green"),
    ("rose", "Rose"),
];

/// Lightweight theme selector aligned with v4 page nav actions.
#[component]
pub fn ThemeSelector(#[props(default)] class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    let mut selected_theme = use_signal(|| "neutral".to_string());

    rsx! {
        div { class: "flex items-center gap-2 {class}",
            label { class: "sr-only", r#for: "theme-selector", "Theme" }

            NativeSelect {
                id: "theme-selector",
                size: NativeSelectSize::Sm,
                class: Some("w-36".to_string()),
                value: Some(selected_theme()),
                on_change: move |value| selected_theme.set(value),
                for (value, label) in THEMES {
                    NativeSelectOption { value: value, "{label}" }
                }
            }

            a {
                href: "/themes",
                class: "inline-flex h-8 items-center justify-center rounded-md border bg-transparent px-3 text-xs font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                "Open"
            }

            Separator { orientation: SeparatorOrientation::Vertical, class: "h-4" }
            ThemeToggle {}
        }
    }
}
