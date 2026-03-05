//! Alert component for displaying important messages.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, TriangleAlert, CircleCheck, Info};

/// Alert variant styles.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
}

impl AlertVariant {
    fn classes(&self) -> &'static str {
        match self {
            AlertVariant::Default => "bg-background text-foreground",
            AlertVariant::Destructive => {
                "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
            }
            AlertVariant::Success => {
                "border-green-500/50 text-green-700 dark:text-green-400 [&>svg]:text-green-600"
            }
            AlertVariant::Warning => {
                "border-yellow-500/50 text-yellow-700 dark:text-yellow-400 [&>svg]:text-yellow-600"
            }
        }
    }

    fn icon(&self) -> Element {
        match self {
            AlertVariant::Default => rsx! { Info { class: "h-4 w-4" } },
            AlertVariant::Destructive => rsx! { CircleAlert { class: "h-4 w-4" } },
            AlertVariant::Success => rsx! { CircleCheck { class: "h-4 w-4" } },
            AlertVariant::Warning => rsx! { TriangleAlert { class: "h-4 w-4" } },
        }
    }
}

/// Props for the Alert component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// The visual variant of the alert.
    #[props(default)]
    pub variant: AlertVariant,

    /// Whether to show the default icon for the variant.
    #[props(default = true)]
    pub show_icon: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Alert content (typically AlertTitle and AlertDescription).
    pub children: Element,
}

/// A component for displaying important messages to users.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Alert {
///         AlertTitle { "Heads up!" }
///         AlertDescription { "You can add components to your app using the CLI." }
///     }
///
///     Alert { variant: AlertVariant::Destructive,
///         AlertTitle { "Error" }
///         AlertDescription { "Your session has expired. Please log in again." }
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let variant_classes = props.variant.classes();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "relative w-full rounded-lg border p-4 [&>svg+div]:translate-y-[-3px] \
         [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&:has(svg)]:pl-11 {} {}",
        variant_classes, custom_class
    );

    rsx! {
        div {
            class: classes,
            role: "alert",
            "data-slot": "alert",
            "data-variant": match props.variant {
                AlertVariant::Default => "default",
                AlertVariant::Destructive => "destructive",
                AlertVariant::Success => "success",
                AlertVariant::Warning => "warning",
            },

            if props.show_icon {
                {props.variant.icon()}
            }

            {props.children}
        }
    }
}

/// Props for AlertTitle.
#[derive(Props, Clone, PartialEq)]
pub struct AlertTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for an Alert.
#[component]
pub fn AlertTitle(props: AlertTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "mb-1 font-medium leading-none tracking-tight {}",
        custom_class
    );

    rsx! {
        h5 {
            class: classes,
            "data-slot": "alert-title",
            {props.children}
        }
    }
}

/// Props for AlertDescription.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDescriptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for an Alert.
#[component]
pub fn AlertDescription(props: AlertDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("text-sm [&_p]:leading-relaxed {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "alert-description",
            {props.children}
        }
    }
}
