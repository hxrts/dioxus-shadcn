//! Alert component for displaying important messages.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, CircleCheck, Info, TriangleAlert};

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
            AlertVariant::Default => "bg-card text-card-foreground",
            AlertVariant::Destructive => {
                "bg-card text-destructive *:data-[slot=alert-description]:text-destructive/90 [&>svg]:text-current"
            }
            AlertVariant::Success => {
                "bg-card text-green-700 dark:text-green-400 *:data-[slot=alert-description]:text-green-600/90 [&>svg]:text-current"
            }
            AlertVariant::Warning => {
                "bg-card text-yellow-700 dark:text-yellow-400 *:data-[slot=alert-description]:text-yellow-600/90 [&>svg]:text-current"
            }
        }
    }

    fn icon(&self) -> Element {
        // Note: size and translate-y are applied via parent's [&>svg] selectors
        match self {
            AlertVariant::Default => rsx! { Info {} },
            AlertVariant::Destructive => rsx! { CircleAlert {} },
            AlertVariant::Success => rsx! { CircleCheck {} },
            AlertVariant::Warning => rsx! { TriangleAlert {} },
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
        "relative grid w-full grid-cols-[0_1fr] items-start gap-y-0.5 rounded-lg border px-4 py-3 text-sm \
         has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-3 \
         [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current {} {}",
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
        "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight {}",
        custom_class
    );

    rsx! {
        div {
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

    let classes = format!(
        "col-start-2 grid justify-items-start gap-1 text-sm text-muted-foreground [&_p]:leading-relaxed {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "alert-description",
            {props.children}
        }
    }
}
