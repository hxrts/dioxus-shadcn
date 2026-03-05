//! Direction component for RTL/LTR support.
//!
//! A context provider for managing text direction across your application,
//! supporting both left-to-right and right-to-left layouts.

use dioxus::prelude::*;

/// Text direction options.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Direction {
    /// Left-to-right (default for most languages)
    #[default]
    Ltr,
    /// Right-to-left (for Arabic, Hebrew, etc.)
    Rtl,
}

impl Direction {
    /// Get the direction as a string value for the `dir` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }
}

/// Context for managing direction state.
#[derive(Clone, Copy)]
pub struct DirectionContext {
    /// The current text direction.
    pub direction: Direction,
}

/// Props for DirectionProvider.
#[derive(Props, Clone, PartialEq)]
pub struct DirectionProviderProps {
    /// The text direction to use.
    #[props(default)]
    pub direction: Direction,

    /// Children to render within the direction context.
    pub children: Element,
}

/// A provider component for managing text direction.
///
/// Wraps children in a context that provides the current direction,
/// and sets the `dir` attribute on the wrapper element.
///
/// # Example
///
/// ```rust
/// rsx! {
///     DirectionProvider {
///         direction: Direction::Rtl,
///
///         // All children will be RTL
///         div { "This text will be right-to-left" }
///     }
/// }
/// ```
#[component]
pub fn DirectionProvider(props: DirectionProviderProps) -> Element {
    let context = DirectionContext {
        direction: props.direction,
    };

    use_context_provider(|| context);

    rsx! {
        div {
            dir: props.direction.as_str(),
            "data-slot": "direction-provider",
            "data-direction": props.direction.as_str(),
            {props.children}
        }
    }
}

/// Hook to access the current direction context.
///
/// Returns the current direction from the nearest DirectionProvider,
/// or defaults to LTR if no provider is found.
///
/// # Example
///
/// ```rust
/// fn MyComponent() -> Element {
///     let direction = use_direction();
///
///     let align_class = match direction {
///         Direction::Ltr => "text-left",
///         Direction::Rtl => "text-right",
///     };
///
///     rsx! {
///         div { class: align_class, "Aligned text" }
///     }
/// }
/// ```
pub fn use_direction() -> Direction {
    try_use_context::<DirectionContext>()
        .map(|ctx| ctx.direction)
        .unwrap_or_default()
}

/// Hook to access the full direction context.
///
/// Returns the DirectionContext from the nearest DirectionProvider,
/// or a default LTR context if no provider is found.
pub fn use_direction_context() -> DirectionContext {
    try_use_context::<DirectionContext>().unwrap_or(DirectionContext {
        direction: Direction::Ltr,
    })
}
