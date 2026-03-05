//! # Component Prop Conventions for Lumen Blocks
//!
//! This module documents and provides utilities for consistent prop patterns
//! across all Lumen Blocks components.
//!
//! ## Controlled vs Uncontrolled Components
//!
//! Stateful components should support both controlled and uncontrolled usage patterns:
//!
//! ### Uncontrolled (Default)
//! The component manages its own state internally. Use `default_*` props to set initial values.
//!
//! ```rust
//! // Uncontrolled checkbox - component manages checked state internally
//! Checkbox {
//!     default_checked: true,
//!     on_checked_change: move |checked| {
//!         println!("Checkbox is now: {}", checked);
//!     }
//! }
//! ```
//!
//! ### Controlled
//! The parent manages state via signals. The component reflects the signal value
//! and calls callbacks to request changes.
//!
//! ```rust
//! // Controlled checkbox - parent manages checked state
//! let is_checked = use_signal(|| false);
//!
//! Checkbox {
//!     checked: is_checked,
//!     on_checked_change: move |new_value| {
//!         is_checked.set(new_value);
//!     }
//! }
//! ```
//!
//! ## Callback Naming Conventions
//!
//! Use consistent callback names across similar components:
//!
//! | Callback Name | Use Case | Type |
//! |--------------|----------|------|
//! | `on_click` | Generic click events | `Callback<MouseEvent>` |
//! | `on_change` | Form value changes | `Callback<FormEvent>` |
//! | `on_input` | Text input changes | `Callback<FormEvent>` |
//! | `on_checked_change` | Boolean toggle changes | `Callback<bool>` |
//! | `on_value_change` | Value selection changes | `Callback<T>` |
//! | `on_open_change` | Open/close state changes | `Callback<bool>` |
//! | `on_select` | Item selection | `Callback<T>` |
//! | `on_focus` | Focus gained | `Callback<FocusEvent>` |
//! | `on_blur` | Focus lost | `Callback<FocusEvent>` |
//!
//! ## ID Handling
//!
//! Components should:
//! 1. Accept an optional `id: Option<String>` prop
//! 2. Generate a unique ID if not provided using `use_unique_id()`
//! 3. Use `use_id_or()` to merge user-provided and generated IDs
//!
//! ```rust
//! #[component]
//! fn MyComponent(props: MyComponentProps) -> Element {
//!     let generated_id = use_unique_id();
//!     let props_id = use_signal(|| props.id.clone());
//!     let id = use_id_or(generated_id, props_id.into());
//!
//!     rsx! {
//!         div { id: id }
//!     }
//! }
//! ```
//!
//! ## data-slot Attributes
//!
//! All components should include `data-slot` attributes for styling hooks:
//!
//! ```rust
//! rsx! {
//!     button {
//!         "data-slot": "button",
//!         // ...
//!     }
//! }
//! ```
//!
//! Common data-slot values:
//! - `button`, `input`, `label`, `checkbox`, `switch`
//! - `card`, `card-header`, `card-title`, `card-content`, `card-footer`
//! - `dialog`, `dialog-content`, `dialog-header`, `dialog-footer`
//! - `dropdown`, `dropdown-trigger`, `dropdown-content`, `dropdown-item`
//!
//! ## data-state Attributes
//!
//! Use `data-state` for component states that affect styling:
//!
//! ```rust
//! rsx! {
//!     div {
//!         "data-state": if is_open { "open" } else { "closed" },
//!     }
//! }
//! ```
//!
//! Common data-state values:
//! - `open` / `closed` - for disclosure components
//! - `checked` / `unchecked` - for checkboxes, switches
//! - `active` / `inactive` - for toggles, tabs
//! - `valid` / `invalid` - for form validation
//! - `loading` - for async states
//!
//! ## Standard Props Template
//!
//! Use this template for new components:
//!
//! ```rust
//! #[derive(Props, Clone, PartialEq)]
//! pub struct MyComponentProps {
//!     // Content
//!     pub children: Element,
//!
//!     // Identity
//!     #[props(default)]
//!     pub id: Option<String>,
//!
//!     // Styling
//!     #[props(default)]
//!     pub class: Option<String>,
//!
//!     // State (for controlled mode)
//!     #[props(default)]
//!     pub value: Option<Signal<T>>,
//!
//!     // Default state (for uncontrolled mode)
//!     #[props(default)]
//!     pub default_value: T,
//!
//!     // Callbacks
//!     #[props(default)]
//!     pub on_value_change: Option<Callback<T>>,
//!
//!     // Disabled state
//!     #[props(default = false)]
//!     pub disabled: bool,
//!
//!     // Pass-through HTML attributes
//!     #[props(extends = GlobalAttributes)]
//!     pub attributes: Vec<Attribute>,
//! }
//! ```

use dioxus::prelude::*;

/// Helper trait for components that support controlled/uncontrolled patterns.
///
/// This trait helps implement the dual-mode pattern consistently.
pub trait Controllable<T: Clone + PartialEq> {
    /// Get the current value, preferring controlled value over internal state.
    fn value(&self) -> T;

    /// Update the value, respecting controlled/uncontrolled mode.
    fn set_value(&mut self, value: T);

    /// Check if the component is in controlled mode.
    fn is_controlled(&self) -> bool;
}

/// A helper struct for implementing controlled/uncontrolled components.
///
/// # Example
///
/// ```rust
/// #[component]
/// fn Toggle(props: ToggleProps) -> Element {
///     let state = ControlledState::new(
///         props.checked,           // Option<Signal<bool>>
///         props.default_checked,   // bool
///     );
///
///     let is_checked = state.value();
///
///     let handle_click = move |_| {
///         let new_value = !state.value();
///         state.set_value(new_value);
///
///         if let Some(callback) = &props.on_checked_change {
///             callback.call(new_value);
///         }
///     };
///
///     // ...
/// }
/// ```
#[derive(Clone)]
pub struct ControlledState<T: Clone + PartialEq + 'static> {
    /// The controlled signal (if provided by parent)
    controlled: Option<Signal<T>>,
    /// The internal signal (for uncontrolled mode)
    internal: Signal<T>,
}

impl<T: Clone + PartialEq + 'static> ControlledState<T> {
    /// Create a new controlled state helper.
    ///
    /// # Arguments
    /// * `controlled` - Optional signal from parent for controlled mode
    /// * `default_value` - Default value for uncontrolled mode
    pub fn new(controlled: Option<Signal<T>>, default_value: T) -> Self {
        let internal = use_signal(|| default_value);
        Self {
            controlled,
            internal,
        }
    }

    /// Get the current value.
    pub fn value(&self) -> T {
        match &self.controlled {
            Some(signal) => signal.read().clone(),
            None => self.internal.read().clone(),
        }
    }

    /// Set the value. Only updates internal state in uncontrolled mode.
    /// In controlled mode, the parent is responsible for updating the signal.
    pub fn set_value(&self, value: T) {
        if self.controlled.is_none() {
            self.internal.set(value);
        }
    }

    /// Check if in controlled mode.
    pub fn is_controlled(&self) -> bool {
        self.controlled.is_some()
    }

    /// Get a readable signal that reflects the current value.
    pub fn signal(&self) -> Signal<T> {
        match self.controlled {
            Some(signal) => signal,
            None => self.internal,
        }
    }
}

/// Extension trait for working with optional callbacks.
pub trait CallbackExt<T> {
    /// Call the callback if it exists with the given value.
    fn call_if_some(&self, value: T);
}

impl<T: 'static> CallbackExt<T> for Option<Callback<T>> {
    fn call_if_some(&self, value: T) {
        if let Some(callback) = self {
            callback.call(value);
        }
    }
}

/// Extension trait for optional event handlers.
pub trait EventHandlerExt<T> {
    /// Call the event handler if it exists.
    fn call_if_some(&self, event: T);
}

impl<T: 'static> EventHandlerExt<T> for Option<EventHandler<T>> {
    fn call_if_some(&self, event: T) {
        if let Some(handler) = self {
            handler.call(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests would go here once we have a proper test harness
}
