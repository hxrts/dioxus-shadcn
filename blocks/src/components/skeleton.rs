//! Skeleton component for loading state placeholders.

use dioxus::prelude::*;

/// Props for the Skeleton component.
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Additional CSS classes. Use width/height utilities to size the skeleton.
    #[props(default)]
    pub class: Option<String>,
}

/// A placeholder component for loading states.
///
/// Use Tailwind width and height utilities to size the skeleton.
///
/// # Example
///
/// ```rust
/// rsx! {
///     // Basic skeleton
///     Skeleton { class: "h-4 w-[250px]" }
///
///     // Card skeleton
///     div { class: "flex items-center space-x-4",
///         Skeleton { class: "h-12 w-12 rounded-full" }
///         div { class: "space-y-2",
///             Skeleton { class: "h-4 w-[250px]" }
///             Skeleton { class: "h-4 w-[200px]" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("animate-pulse rounded-md bg-accent {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "skeleton",
            aria_hidden: "true",
        }
    }
}
