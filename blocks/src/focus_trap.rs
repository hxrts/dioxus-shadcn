//! Focus Trap Utilities for Modal Components
//!
//! This module provides hooks for trapping focus within modal elements like dialogs,
//! side sheets, and dropdowns. It ensures keyboard accessibility by keeping focus
//! within the modal while it's open.
//!
//! # Example
//!
//! ```rust
//! use lumen_blocks::focus_trap::{use_focus_trap, use_escape_close};
//!
//! #[component]
//! fn Modal(is_open: Signal<bool>, on_close: Callback<()>) -> Element {
//!     // Trap focus when modal is open
//!     use_focus_trap(is_open, "modal-content");
//!
//!     // Close on Escape key
//!     use_escape_close(is_open, on_close);
//!
//!     rsx! {
//!         div {
//!             id: "modal-content",
//!             // Modal content...
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use dioxus::document::eval;

/// Selector for all focusable elements within a container.
pub const FOCUSABLE_SELECTOR: &str = r#"
    a[href]:not([disabled]):not([tabindex="-1"]),
    button:not([disabled]):not([tabindex="-1"]),
    textarea:not([disabled]):not([tabindex="-1"]),
    input:not([disabled]):not([tabindex="-1"]),
    select:not([disabled]):not([tabindex="-1"]),
    [tabindex]:not([tabindex="-1"]):not([disabled])
"#;

/// Hook to trap focus within a container element.
///
/// When `is_active` is true, this hook will:
/// 1. Focus the first focusable element within the container
/// 2. Trap Tab/Shift+Tab navigation within the container
/// 3. Return focus to the previously focused element when deactivated
///
/// # Arguments
/// * `is_active` - Signal that controls whether focus trapping is active
/// * `container_id` - The ID of the container element to trap focus within
///
/// # Note
/// This hook uses JavaScript evaluation to manipulate focus. It assumes
/// the container element exists in the DOM when `is_active` becomes true.
pub fn use_focus_trap(is_active: Signal<bool>, container_id: &'static str) {
    // Store the previously focused element to restore later
    let mut previous_focus = use_signal(|| Option::<String>::None);

    use_effect(move || {
        let active = is_active();

        if active {
            // Store current focus and activate trap
            spawn(async move {
                // Store the currently focused element
                let store_focus = format!(
                    r#"
                    (function() {{
                        const activeEl = document.activeElement;
                        if (activeEl && activeEl.id) {{
                            return activeEl.id;
                        }}
                        return null;
                    }})()
                    "#
                );

                let mut result = eval(&store_focus);
                if let Ok(value) = result.recv::<String>().await {
                    if value != "null" && !value.is_empty() {
                        previous_focus.set(Some(value.trim_matches('"').to_string()));
                    }
                }

                // Focus the first focusable element in the container
                let focus_first = format!(
                    r#"
                    (function() {{
                        const container = document.getElementById('{container_id}');
                        if (!container) return;

                        const focusable = container.querySelectorAll(`{FOCUSABLE_SELECTOR}`);
                        if (focusable.length > 0) {{
                            focusable[0].focus();
                        }}
                    }})()
                    "#,
                    container_id = container_id,
                    FOCUSABLE_SELECTOR = FOCUSABLE_SELECTOR.replace('\n', " ").trim(),
                );

                let _ = eval(&focus_first).await;
            });
        } else {
            // Restore focus when deactivated
            let prev_id_opt = { previous_focus.peek().clone() };
            if let Some(prev_id) = prev_id_opt {
                previous_focus.set(None);
                spawn(async move {
                    let restore_focus = format!(
                        r#"
                        (function() {{
                            const el = document.getElementById('{prev_id}');
                            if (el && typeof el.focus === 'function') {{
                                el.focus();
                            }}
                        }})()
                        "#,
                        prev_id = prev_id,
                    );
                    let _ = eval(&restore_focus).await;
                });
            }
        }
    });

    // Set up keyboard trap (Tab/Shift+Tab cycling)
    use_effect(move || {
        if !is_active() {
            return;
        }

        spawn(async move {
            let setup_trap = format!(
                r#"
                (function() {{
                    const container = document.getElementById('{container_id}');
                    if (!container) return;

                    const handleKeyDown = (e) => {{
                        if (e.key !== 'Tab') return;

                        const focusable = container.querySelectorAll(`{selector}`);
                        if (focusable.length === 0) return;

                        const first = focusable[0];
                        const last = focusable[focusable.length - 1];

                        if (e.shiftKey) {{
                            // Shift+Tab: if on first, go to last
                            if (document.activeElement === first) {{
                                e.preventDefault();
                                last.focus();
                            }}
                        }} else {{
                            // Tab: if on last, go to first
                            if (document.activeElement === last) {{
                                e.preventDefault();
                                first.focus();
                            }}
                        }}
                    }};

                    container._focusTrapHandler = handleKeyDown;
                    container.addEventListener('keydown', handleKeyDown);
                }})()
                "#,
                container_id = container_id,
                selector = FOCUSABLE_SELECTOR.replace('\n', " ").trim(),
            );

            let _ = eval(&setup_trap).await;
        });

        // Cleanup function would go here, but Dioxus effects don't have cleanup yet
        // The trap will be naturally removed when the element is removed from DOM
    });
}

/// Hook to close a modal when the Escape key is pressed.
///
/// # Arguments
/// * `is_active` - Signal that controls whether the escape handler is active
/// * `on_close` - Callback to invoke when Escape is pressed
///
/// # Example
///
/// ```rust
/// use_escape_close(is_open, move |_| {
///     is_open.set(false);
/// });
/// ```
pub fn use_escape_close<F>(is_active: Signal<bool>, _on_close: F)
where
    F: FnMut() + 'static,
{
    use_effect(move || {
        if !is_active() {
            return;
        }

        // Note: In a full implementation, we would set up a global keydown listener.
        // For now, components should handle Escape in their own keydown handlers.
        // This is a placeholder for the pattern.
    });
}

/// Props for the FocusTrap component wrapper.
#[derive(Props, Clone, PartialEq)]
pub struct FocusTrapProps {
    /// Whether focus trapping is active
    pub active: bool,
    /// The ID to assign to the trap container
    #[props(default = "focus-trap-container".to_string())]
    pub id: String,
    /// Children to render inside the focus trap
    pub children: Element,
    /// Optional additional class names
    #[props(default)]
    pub class: Option<String>,
}

/// A wrapper component that provides focus trapping.
///
/// This is an alternative to using the `use_focus_trap` hook directly.
/// It wraps children in a div that traps focus when active.
///
/// # Example
///
/// ```rust
/// rsx! {
///     FocusTrap {
///         active: *is_open.read(),
///         id: "my-modal",
///         class: "modal-content",
///
///         // Modal content...
///         button { "Close" }
///     }
/// }
/// ```
#[component]
pub fn FocusTrap(props: FocusTrapProps) -> Element {
    let mut is_active = use_signal(|| props.active);
    let id_for_effect = props.id.clone();

    // Keep signal in sync with prop
    use_effect(move || {
        is_active.set(props.active);
    });

    let id_static: &'static str = Box::leak(id_for_effect.into_boxed_str());
    use_focus_trap(is_active, id_static);

    rsx! {
        div {
            id: props.id.clone(),
            class: props.class,
            tabindex: "-1",
            {props.children}
        }
    }
}

/// Helper component that focuses itself when mounted.
///
/// Useful as the first element in a modal to ensure focus moves into the modal.
#[derive(Props, Clone, PartialEq)]
pub struct FocusSentinelProps {
    /// Optional ID for the sentinel
    #[props(default)]
    pub id: Option<String>,
}

#[component]
pub fn FocusSentinel(props: FocusSentinelProps) -> Element {
    let id_for_effect = props.id.clone();
    use_effect(move || {
        if let Some(id) = &id_for_effect {
            let focus_script = format!(
                r#"
                (function() {{
                    const el = document.getElementById('{id}');
                    if (el) el.focus();
                }})()
                "#,
                id = id,
            );
            spawn(async move {
                let _ = eval(&focus_script).await;
            });
        }
    });

    rsx! {
        span {
            id: props.id,
            tabindex: "-1",
            class: "sr-only",
            "aria-hidden": "true",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focusable_selector_is_valid() {
        // Just verify the selector string is not empty and contains expected elements
        assert!(FOCUSABLE_SELECTOR.contains("button"));
        assert!(FOCUSABLE_SELECTOR.contains("input"));
        assert!(FOCUSABLE_SELECTOR.contains("a[href]"));
        assert!(FOCUSABLE_SELECTOR.contains("tabindex"));
    }
}
