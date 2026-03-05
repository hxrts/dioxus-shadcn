//! Separator component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::separator::{Separator, SeparatorOrientation};

pub const HORIZONTAL_SOURCE: &str = r#"rsx! {
    div { class: "space-y-4",
        div { "Content above" }
        Separator {}
        div { "Content below" }
    }
}"#;

#[component]
pub fn SeparatorHorizontalExample() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { "Content above" }
            Separator {}
            div { "Content below" }
        }
    }
}

pub const VERTICAL_SOURCE: &str = r#"rsx! {
    div { class: "flex h-5 items-center space-x-4",
        div { "Left" }
        Separator { orientation: SeparatorOrientation::Vertical }
        div { "Center" }
        Separator { orientation: SeparatorOrientation::Vertical }
        div { "Right" }
    }
}"#;

#[component]
pub fn SeparatorVerticalExample() -> Element {
    rsx! {
        div { class: "flex h-5 items-center space-x-4",
            div { "Left" }
            Separator { orientation: SeparatorOrientation::Vertical }
            div { "Center" }
            Separator { orientation: SeparatorOrientation::Vertical }
            div { "Right" }
        }
    }
}

pub const IN_CARD_SOURCE: &str = r#"rsx! {
    Card {
        CardHeader {
            CardTitle { "Account" }
            CardDescription { "Manage your account settings." }
        }
        Separator {}
        CardContent {
            p { "Your preferences will appear here." }
        }
    }
}"#;

#[component]
pub fn SeparatorInCardExample() -> Element {
    use lumen_blocks::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

    rsx! {
        Card {
            CardHeader {
                CardTitle { "Account" }
                CardDescription { "Manage your account settings." }
            }
            Separator {}
            CardContent {
                p { "Your preferences will appear here." }
            }
        }
    }
}
