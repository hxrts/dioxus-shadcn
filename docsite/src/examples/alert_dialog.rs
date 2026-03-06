//! AlertDialog example components.

use dioxus::prelude::*;
use lumen_blocks::components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogButtonVariant, AlertDialogCancel,
    AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader,
    AlertDialogTitle, AlertDialogTrigger,
};
use lumen_blocks::components::button::Button;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let open = use_signal(|| false);

rsx! {
    AlertDialog {
        open: open,
        on_open_change: move |v| open.set(v),

        AlertDialogTrigger {
            Button { "Show Dialog" }
        }
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Are you absolutely sure?" }
                AlertDialogDescription {
                    "This action cannot be undone. This will permanently delete your
                    account and remove your data from our servers."
                }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { "Continue" }
            }
        }
    }
}"#;

/// Basic alert dialog example.
#[component]
pub fn AlertDialogBasicExample() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        AlertDialog {
            open: open,
            on_open_change: move |v| open.set(v),

            AlertDialogTrigger {
                Button { "Show Dialog" }
            }
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Are you absolutely sure?" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your
                        account and remove your data from our servers."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction { "Continue" }
                }
            }
        }
    }
}

/// Source code for the destructive example.
pub const DESTRUCTIVE_SOURCE: &str = r#"let open = use_signal(|| false);

rsx! {
    AlertDialog {
        open: open,
        on_open_change: move |v| open.set(v),

        AlertDialogTrigger {
            Button { variant: ButtonVariant::Destructive, "Delete Account" }
        }
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Delete Account" }
                AlertDialogDescription {
                    "Are you sure you want to delete your account? All of your data
                    will be permanently removed. This action cannot be undone."
                }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction {
                    variant: AlertDialogButtonVariant::Destructive,
                    "Yes, delete account"
                }
            }
        }
    }
}"#;

/// Destructive action alert dialog example.
#[component]
pub fn AlertDialogDestructiveExample() -> Element {
    use lumen_blocks::components::button::ButtonVariant;
    let mut open = use_signal(|| false);

    rsx! {
        AlertDialog {
            open: open,
            on_open_change: move |v| open.set(v),

            AlertDialogTrigger {
                Button { variant: ButtonVariant::Destructive, "Delete Account" }
            }
            AlertDialogContent {
                AlertDialogHeader {
                    AlertDialogTitle { "Delete Account" }
                    AlertDialogDescription {
                        "Are you sure you want to delete your account? All of your data
                        will be permanently removed. This action cannot be undone."
                    }
                }
                AlertDialogFooter {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        variant: AlertDialogButtonVariant::Destructive,
                        "Yes, delete account"
                    }
                }
            }
        }
    }
}
