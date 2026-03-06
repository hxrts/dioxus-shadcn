//! Theme toggle component for dark/light mode switching.

use dioxus::prelude::*;
use lucide_dioxus::{Moon, Sun};

/// Global signal for theme state.
/// "light" | "dark" | "system"
static THEME: GlobalSignal<String> = Signal::global(|| "system".to_string());

/// Theme toggle button component - matches shadcn ModeSwitcher styling.
#[component]
pub fn ThemeToggle() -> Element {
    use_effect(move || {
        let initial = read_saved_theme().unwrap_or_else(|| "system".to_string());
        *THEME.write() = initial.clone();
        apply_theme(&initial);
    });

    let current_theme = THEME.read().clone();
    let is_dark = is_effectively_dark(&current_theme);

    rsx! {
        button {
            r#type: "button",
            class: "inline-flex items-center justify-center size-8 rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors",
            onclick: move |_| {
                let next = if is_dark {
                    "light".to_string()
                } else {
                    "dark".to_string()
                };
                *THEME.write() = next.clone();
                save_theme(&next);
                apply_theme(&next);
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

/// Returns true when the current effective theme is dark.
fn is_effectively_dark(theme: &str) -> bool {
    if theme == "dark" {
        return true;
    }
    if theme == "light" {
        return false;
    }

    system_prefers_dark()
}

/// Read saved theme from local storage.
fn read_saved_theme() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let value = storage.get_item("theme").ok()??;
        if value == "dark" || value == "light" || value == "system" {
            Some(value)
        } else {
            None
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Persist theme choice to local storage.
fn save_theme(theme: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", theme);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = theme;
    }
}

/// Returns true if the OS/browser preference is dark.
fn system_prefers_dark() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media)) = window.match_media("(prefers-color-scheme: dark)") {
                return media.matches();
            }
        }
        false
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Apply a theme value to document classes.
fn apply_theme(theme: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let class_list = html.class_list();
                    let _ = class_list.remove_1("dark");
                    let _ = class_list.remove_1("light");

                    if theme == "dark" {
                        let _ = class_list.add_1("dark");
                    } else if theme == "light" {
                        let _ = class_list.add_1("light");
                    } else if system_prefers_dark() {
                        let _ = class_list.add_1("dark");
                    }
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = theme;
    }
}
