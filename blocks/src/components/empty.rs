//! Empty component for empty state displays.
//!
//! A set of components for displaying empty states with icons, titles,
//! descriptions, and action buttons.

use dioxus::prelude::*;

/// Media variant for EmptyMedia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum EmptyMediaVariant {
    /// Default variant (no special styling)
    #[default]
    Default,
    /// Icon variant with background
    Icon,
}

/// Props for Empty.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Empty state content.
    pub children: Element,
}

/// A container for empty state content.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Empty {
///         EmptyMedia { variant: EmptyMediaVariant::Icon,
///             // Icon here
///         }
///         EmptyHeader {
///             EmptyTitle { "No results found" }
///             EmptyDescription {
///                 "Try adjusting your search or filter to find what you're looking for."
///             }
///         }
///         EmptyContent {
///             Button { "Clear filters" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Empty(props: EmptyProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-col items-center justify-center rounded-lg border border-dashed p-6 text-center \
         md:p-12 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "empty",
            {props.children}
        }
    }
}

/// Props for EmptyHeader.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section for empty state.
#[component]
pub fn EmptyHeader(props: EmptyHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex max-w-md flex-col items-center gap-1 {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "empty-header",
            {props.children}
        }
    }
}

/// Props for EmptyMedia.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyMediaProps {
    /// Visual variant.
    #[props(default)]
    pub variant: EmptyMediaVariant,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Media content (icon or image).
    pub children: Element,
}

/// Media section for empty state (icon or image).
#[component]
pub fn EmptyMedia(props: EmptyMediaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        EmptyMediaVariant::Default => "mb-2",
        EmptyMediaVariant::Icon => {
            "mb-2 flex size-10 shrink-0 items-center justify-center rounded-lg bg-muted text-foreground \
             [&_svg:not([class*='size-'])]:size-5"
        }
    };

    let classes = format!("{} {}", variant_class, custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "empty-media",
            "data-variant": match props.variant {
                EmptyMediaVariant::Default => "default",
                EmptyMediaVariant::Icon => "icon",
            },
            {props.children}
        }
    }
}

/// Props for EmptyTitle.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for empty state.
#[component]
pub fn EmptyTitle(props: EmptyTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-lg font-medium tracking-tight {}", custom_class);

    rsx! {
        h3 {
            class: classes,
            "data-slot": "empty-title",
            {props.children}
        }
    }
}

/// Props for EmptyDescription.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for empty state.
#[component]
pub fn EmptyDescription(props: EmptyDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "text-sm/relaxed text-muted-foreground \
         [&_a]:text-foreground [&_a]:underline [&_a]:underline-offset-4 [&_a]:hover:text-primary \
         {}",
        custom_class
    );

    rsx! {
        p {
            class: classes,
            "data-slot": "empty-description",
            {props.children}
        }
    }
}

/// Props for EmptyContent.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content (actions, buttons, etc.).
    pub children: Element,
}

/// Content section for empty state actions.
#[component]
pub fn EmptyContent(props: EmptyContentProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("mt-4 flex max-w-md flex-col gap-2 {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "empty-content",
            {props.children}
        }
    }
}
