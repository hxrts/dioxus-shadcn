#![allow(
    clippy::clone_on_copy,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_closure,
    clippy::redundant_locals,
    clippy::useless_format
)]

use dioxus::prelude::*;

// Re-export log crate for use in components
pub use log;

pub mod components;
pub mod focus_trap;
pub mod patterns;
pub mod theme;
pub mod variants;

// Re-export commonly used items
pub use focus_trap::{FocusSentinel, FocusTrap, use_focus_trap};
pub use patterns::{CallbackExt, ControlledState, EventHandlerExt};
pub use theme::{
    ColorScheme, OklchColor, Theme, ThemeColors, ThemeContext, ThemeProvider, themes, use_theme,
};
pub use variants::{CompoundVariant, VariantConfig, class_if, class_switch, cn, cva};

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    // Use hook to capture a unique ID once during first render
    use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Signal::new(format!("dxc-{id}"))
    })
}

// Elements can only have one id so if the user provides their own, we must use it as the aria id.
fn use_id_or(
    mut gen_id: Signal<String>,
    user_id: ReadSignal<Option<String>>,
) -> Memo<Option<String>> {
    // If we have a user ID, update the gen_id in an effect
    use_effect(move || {
        if let Some(id) = user_id() {
            gen_id.set(id);
        }
    });

    // Return the appropriate ID
    use_memo(move || match user_id() {
        Some(user_id) => Some(user_id),
        None => Some(gen_id.peek().clone()),
    })
}
