//! Theme toggle component for dark/light mode switching.

use dioxus::prelude::*;
use lucide_dioxus::{Moon, Sun};

/// Global signal for theme state.
/// "light" | "dark" | "system"
static THEME: GlobalSignal<&'static str> = Signal::global(|| "system");

/// Theme toggle button component - matches shadcn ModeSwitcher styling.
#[component]
pub fn ThemeToggle() -> Element {
    let current_theme = *THEME.read();
    let is_dark = current_theme == "dark";

    rsx! {
        button {
            r#type: "button",
            class: "inline-flex items-center justify-center size-8 rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors",
            onclick: move |_| {
                let new_theme = if is_dark { "light" } else { "dark" };
                *THEME.write() = new_theme;
                // Update document class for dark mode
                spawn(async move {
                    toggle_theme_class(new_theme == "dark");
                });
            },
            if is_dark {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
            span { class: "sr-only", "Toggle theme" }
        }
    }
}

/// Toggle dark class on document element.
fn toggle_theme_class(is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let class_list = html.class_list();
                    if is_dark {
                        let _ = class_list.add_1("dark");
                    } else {
                        let _ = class_list.remove_1("dark");
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = is_dark; // Suppress unused warning on non-wasm targets
    }
}
