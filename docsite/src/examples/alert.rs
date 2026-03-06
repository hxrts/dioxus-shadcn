//! Alert component examples and source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

pub const VARIANTS_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Alert {
            AlertTitle { "Default Alert" }
            AlertDescription { "This is a default informational alert." }
        }
        Alert { variant: AlertVariant::Destructive,
            AlertTitle { "Error" }
            AlertDescription { "Something went wrong. Please try again." }
        }
        Alert { variant: AlertVariant::Success,
            AlertTitle { "Success" }
            AlertDescription { "Your changes have been saved successfully." }
        }
        Alert { variant: AlertVariant::Warning,
            AlertTitle { "Warning" }
            AlertDescription { "This action cannot be undone." }
        }
    }
}"#;

#[component]
pub fn AlertVariantsExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Alert {
                AlertTitle { "Default Alert" }
                AlertDescription { "This is a default informational alert." }
            }
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Error" }
                AlertDescription { "Something went wrong. Please try again." }
            }
            Alert { variant: AlertVariant::Success,
                AlertTitle { "Success" }
                AlertDescription { "Your changes have been saved successfully." }
            }
            Alert { variant: AlertVariant::Warning,
                AlertTitle { "Warning" }
                AlertDescription { "This action cannot be undone." }
            }
        }
    }
}

pub const NO_ICON_SOURCE: &str = r#"rsx! {
    Alert { show_icon: false,
        AlertTitle { "Without Icon" }
        AlertDescription { "This alert does not display the default icon." }
    }
}"#;

#[component]
pub fn AlertNoIconExample() -> Element {
    rsx! {
        Alert { show_icon: false,
            AlertTitle { "Without Icon" }
            AlertDescription { "This alert does not display the default icon." }
        }
    }
}
