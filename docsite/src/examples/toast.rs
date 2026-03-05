//! Toast component examples with embedded source code.

use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};
use lumen_blocks::components::toast::{use_toast, ToastOptions};
use std::time::Duration;

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use lumen_blocks::components::toast::{ToastProvider, use_toast};
use lumen_blocks::components::button::{Button, ButtonVariant};

// Wrap your app in ToastProvider
rsx! {
    ToastProvider {
        ToastDemo {}
    }
}

// Then use the toast hook
fn ToastDemo() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            on_click: move |_| {
                toast.success("Success!", None);
            },
            "Show Toast"
        }
    }
}"#;

pub const VARIANTS_SOURCE: &str = r#"use lumen_blocks::components::toast::{use_toast, ToastOptions};

let toast = use_toast();

// Success toast
toast.success("Operation completed!", None);

// Error toast
toast.error("Something went wrong", None);

// Warning toast
toast.warning("Please review your input", None);

// Info toast
toast.info("Here's some information", None);"#;

pub const WITH_DESCRIPTION_SOURCE: &str = r#"use lumen_blocks::components::toast::{use_toast, ToastOptions};

let toast = use_toast();

toast.success(
    "Profile updated",
    Some(ToastOptions::with_description("Your changes have been saved."))
);"#;

pub const CUSTOM_DURATION_SOURCE: &str = r#"use lumen_blocks::components::toast::{use_toast, ToastOptions};
use std::time::Duration;

let toast = use_toast();

// Show for 10 seconds
toast.info(
    "Extended toast",
    Some(ToastOptions::with_description("This will stay longer.")
        .duration(Duration::from_secs(10)))
);

// Permanent toast (no auto-dismiss)
toast.warning(
    "Action required",
    Some(ToastOptions::with_description("Please complete your profile.")
        .permanent())
);"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn ToastBasicExample() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            on_click: move |_| {
                toast.success("Success!", None);
            },
            "Show Toast"
        }
    }
}

#[component]
pub fn ToastVariantsExample() -> Element {
    let toast = use_toast();
    let toast_success = toast.clone();
    let toast_error = toast.clone();
    let toast_warning = toast.clone();
    let toast_info = toast.clone();

    rsx! {
        div { class: "flex flex-wrap gap-2",
            Button {
                variant: ButtonVariant::Primary,
                on_click: move |_| {
                    toast_success.success("Success!", Some(ToastOptions::with_description("Your action was successful.")));
                },
                "Success"
            }
            Button {
                variant: ButtonVariant::Destructive,
                on_click: move |_| {
                    toast_error.error("Error!", Some(ToastOptions::with_description("Something went wrong.")));
                },
                "Error"
            }
            Button {
                variant: ButtonVariant::Outline,
                on_click: move |_| {
                    toast_warning.warning("Warning!", Some(ToastOptions::with_description("Please review your input.")));
                },
                "Warning"
            }
            Button {
                variant: ButtonVariant::Secondary,
                on_click: move |_| {
                    toast_info.info("Info", Some(ToastOptions::with_description("Here's some helpful information.")));
                },
                "Info"
            }
        }
    }
}

#[component]
pub fn ToastWithDescriptionExample() -> Element {
    let toast = use_toast();

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            on_click: move |_| {
                toast.success(
                    "Profile updated",
                    Some(ToastOptions::with_description("Your changes have been saved successfully."))
                );
            },
            "Show with Description"
        }
    }
}

#[component]
pub fn ToastDurationExample() -> Element {
    let toast = use_toast();
    let toast_quick = toast.clone();
    let toast_extended = toast.clone();
    let toast_permanent = toast.clone();

    rsx! {
        div { class: "flex flex-wrap gap-2",
            Button {
                variant: ButtonVariant::Outline,
                on_click: move |_| {
                    toast_quick.info(
                        "Quick toast",
                        Some(ToastOptions::with_description("This disappears quickly.")
                            .duration(Duration::from_secs(2)))
                    );
                },
                "Quick (2s)"
            }
            Button {
                variant: ButtonVariant::Outline,
                on_click: move |_| {
                    toast_extended.info(
                        "Extended toast",
                        Some(ToastOptions::with_description("This stays longer.")
                            .duration(Duration::from_secs(10)))
                    );
                },
                "Extended (10s)"
            }
            Button {
                variant: ButtonVariant::Outline,
                on_click: move |_| {
                    toast_permanent.warning(
                        "Permanent toast",
                        Some(ToastOptions::with_description("Close this manually.")
                            .permanent())
                    );
                },
                "Permanent"
            }
        }
    }
}
