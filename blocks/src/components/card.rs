//! Card component for displaying content in a contained box.

use dioxus::prelude::*;

/// Props for the Card component.
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Card content.
    pub children: Element,
}

/// A container component for grouping related content.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Card {
///         CardHeader {
///             CardTitle { "Card Title" }
///             CardDescription { "Card description goes here." }
///         }
///         CardContent {
///             p { "Card content goes here." }
///         }
///         CardFooter {
///             Button { "Action" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "rounded-lg border border-border bg-card text-card-foreground shadow-sm {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "card",
            {props.children}
        }
    }
}

/// Props for CardHeader.
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section of a Card.
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 p-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "card-header",
            {props.children}
        }
    }
}

/// Props for CardTitle.
#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title element within a CardHeader.
#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "text-2xl font-semibold leading-none tracking-tight {}",
        custom_class
    );

    rsx! {
        h3 {
            class: classes,
            "data-slot": "card-title",
            {props.children}
        }
    }
}

/// Props for CardDescription.
#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text within a CardHeader.
#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-sm text-muted-foreground {}", custom_class);

    rsx! {
        p {
            class: classes,
            "data-slot": "card-description",
            {props.children}
        }
    }
}

/// Props for CardContent.
#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content.
    pub children: Element,
}

/// Main content area of a Card.
#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("p-6 pt-0 {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "card-content",
            {props.children}
        }
    }
}

/// Props for CardFooter.
#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content.
    pub children: Element,
}

/// Footer section of a Card, typically containing actions.
#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex items-center p-6 pt-0 {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "card-footer",
            {props.children}
        }
    }
}

/// Props for CardAction.
#[derive(Props, Clone, PartialEq)]
pub struct CardActionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Action content (typically a button or dropdown).
    pub children: Element,
}

/// Action element within a CardHeader, positioned in the top-right.
#[component]
pub fn CardAction(props: CardActionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "card-action",
            {props.children}
        }
    }
}
