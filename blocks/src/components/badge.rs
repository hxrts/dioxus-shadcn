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
    Ghost,
    Link,
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "bg-primary text-primary-foreground [a&]:hover:bg-primary/90"
            }
            BadgeVariant::Secondary => {
                "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90"
            }
            BadgeVariant::Destructive => {
                "bg-destructive text-white focus-visible:ring-destructive/20 dark:bg-destructive/60 dark:focus-visible:ring-destructive/40 [a&]:hover:bg-destructive/90"
            }
            BadgeVariant::Outline => {
                "border-border text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground"
            }
            BadgeVariant::Ghost => {
                "[a&]:hover:bg-accent [a&]:hover:text-accent-foreground"
            }
            BadgeVariant::Link => {
                "text-primary underline-offset-4 [a&]:hover:underline"
            }
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
        "inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-full border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap \
         transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 \
         [&>svg]:pointer-events-none [&>svg]:size-3 \
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
                BadgeVariant::Ghost => "ghost",
                BadgeVariant::Link => "link",
            },
            {props.children}
        }
    }
}
