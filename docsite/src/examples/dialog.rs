//! Dialog component examples with embedded source code.

use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};
use lumen_blocks::components::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogOverlay, DialogTitle,
};
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use lumen_blocks::components::button::{Button, ButtonVariant};
use lumen_blocks::components::dialog::{
    Dialog, DialogContent, DialogDescription, DialogHeader,
    DialogOverlay, DialogTitle,
};

let mut open = use_signal(|| false);

rsx! {
    Button {
        variant: ButtonVariant::Outline,
        on_click: move |_| open.set(true),
        "Open Dialog"
    }

    Dialog {
        open: ReadSignal::new(Signal::new(Some(*open.read()))),
        on_open_change: move |new_open| open.set(new_open),

        DialogOverlay {}
        DialogContent {
            DialogHeader {
                DialogTitle { "Dialog Title" }
                DialogDescription { "This is a dialog description." }
            }
        }
    }
}"#;

pub const WITH_FORM_SOURCE: &str = r#"use lumen_blocks::components::dialog::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
};
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;
use lumen_blocks::components::button::{Button, ButtonVariant};

rsx! {
    Dialog {
        DialogContent {
            DialogHeader {
                DialogTitle { "Edit Profile" }
                DialogDescription { "Make changes to your profile here." }
            }
            div { class: "space-y-4 py-4",
                div { class: "space-y-2",
                    Label { for_id: "name", "Name" }
                    Input { id: "name", placeholder: "Enter your name" }
                }
            }
            DialogFooter {
                Button { variant: ButtonVariant::Default, "Save changes" }
            }
        }
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn DialogBasicExample() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            on_click: move |_| open.set(true),
            "Open Dialog"
        }

        Dialog {
            open: ReadSignal::new(Signal::new(Some(*open.read()))),
            on_open_change: move |new_open| open.set(new_open),

            DialogOverlay {}
            DialogContent {
                DialogHeader {
                    DialogTitle { "Are you sure?" }
                    DialogDescription {
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    }
                }
                DialogFooter {
                    DialogClose {
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                    }
                    Button { variant: ButtonVariant::Destructive, "Delete" }
                }
            }
        }
    }
}

#[component]
pub fn DialogWithFormExample() -> Element {
    let mut open = use_signal(|| false);
    let mut name = use_signal(|| String::new());

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            on_click: move |_| open.set(true),
            "Edit Profile"
        }

        Dialog {
            open: ReadSignal::new(Signal::new(Some(*open.read()))),
            on_open_change: move |new_open| open.set(new_open),

            DialogOverlay {}
            DialogContent {
                DialogHeader {
                    DialogTitle { "Edit Profile" }
                    DialogDescription { "Make changes to your profile here. Click save when you're done." }
                }
                div { class: "grid gap-4 py-4",
                    div { class: "grid grid-cols-4 items-center gap-4",
                        Label { for_id: "name", class: "text-right", "Name" }
                        Input {
                            id: "name",
                            class: "col-span-3",
                            placeholder: "Enter your name",
                            value: name,
                            on_input: move |e: FormEvent| name.set(e.value()),
                        }
                    }
                }
                DialogFooter {
                    DialogClose {
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                    }
                    DialogClose {
                        Button { variant: ButtonVariant::Default, "Save changes" }
                    }
                }
            }
        }
    }
}
