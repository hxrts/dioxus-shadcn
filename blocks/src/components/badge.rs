//! Badge component for displaying status indicators and labels.

use dioxus::prelude::*;

/// Badge variant styles.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "border-transparent bg-primary text-primary-foreground hover:bg-primary/80"
            }
            BadgeVariant::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            BadgeVariant::Destructive => {
                "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80"
            }
            BadgeVariant::Outline => "text-foreground",
        }
    }
}

/// Props for the Badge component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// The visual variant of the badge.
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Badge content.
    pub children: Element,
}

/// A small status indicator or label component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Badge { "New" }
///     Badge { variant: BadgeVariant::Secondary, "Draft" }
///     Badge { variant: BadgeVariant::Destructive, "Error" }
///     Badge { variant: BadgeVariant::Outline, "v1.0.0" }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let variant_classes = props.variant.classes();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold \
         transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
         {} {}",
        variant_classes, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "badge",
            "data-variant": match props.variant {
                BadgeVariant::Default => "default",
                BadgeVariant::Secondary => "secondary",
                BadgeVariant::Destructive => "destructive",
                BadgeVariant::Outline => "outline",
            },
            {props.children}
        }
    }
}
