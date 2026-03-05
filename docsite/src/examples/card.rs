//! Card component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::button::Button;
use lumen_blocks::components::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;

pub const BASIC_SOURCE: &str = r#"rsx! {
    Card {
        CardHeader {
            CardTitle { "Card Title" }
            CardDescription { "Card description goes here." }
        }
        CardContent {
            p { "Card content goes here." }
        }
        CardFooter {
            Button { "Action" }
        }
    }
}"#;

#[component]
pub fn CardBasicExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Card Title" }
                CardDescription { "Card description goes here." }
            }
            CardContent {
                p { "Card content goes here." }
            }
            CardFooter {
                Button { "Action" }
            }
        }
    }
}

pub const WITH_FORM_SOURCE: &str = r#"rsx! {
    Card { class: "w-full max-w-sm",
        CardHeader {
            CardTitle { "Create account" }
            CardDescription { "Enter your details to get started." }
        }
        CardContent {
            div { class: "grid gap-4",
                div { class: "grid gap-2",
                    Label { "Name" }
                    Input { placeholder: "John Doe" }
                }
                div { class: "grid gap-2",
                    Label { "Email" }
                    Input { input_type: "email", placeholder: "john@example.com" }
                }
            }
        }
        CardFooter {
            Button { full_width: true, "Create account" }
        }
    }
}"#;

#[component]
pub fn CardWithFormExample() -> Element {
    rsx! {
        Card { class: "w-full max-w-sm",
            CardHeader {
                CardTitle { "Create account" }
                CardDescription { "Enter your details to get started." }
            }
            CardContent {
                div { class: "grid gap-4",
                    div { class: "grid gap-2",
                        Label { "Name" }
                        Input { placeholder: "John Doe" }
                    }
                    div { class: "grid gap-2",
                        Label { "Email" }
                        Input { input_type: "email", placeholder: "john@example.com" }
                    }
                }
            }
            CardFooter {
                Button { full_width: true, "Create account" }
            }
        }
    }
}

pub const WITH_ACTION_SOURCE: &str = r#"rsx! {
    Card {
        CardHeader {
            CardTitle { "Notifications" }
            CardDescription { "You have 3 unread messages." }
            CardAction {
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::IconSm,
                    lucide_dioxus::EllipsisVertical {}
                }
            }
        }
        CardContent {
            p { "View and manage your notifications." }
        }
    }
}"#;

#[component]
pub fn CardWithActionExample() -> Element {
    use lumen_blocks::components::button::{ButtonSize, ButtonVariant};

    rsx! {
        Card {
            CardHeader {
                CardTitle { "Notifications" }
                CardDescription { "You have 3 unread messages." }
                CardAction {
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::IconSm,
                        lucide_dioxus::EllipsisVertical {}
                    }
                }
            }
            CardContent {
                p { "View and manage your notifications." }
            }
        }
    }
}
