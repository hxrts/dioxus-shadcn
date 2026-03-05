//! Avatar component examples and source code.

use dioxus::prelude::*;
use lumen_blocks::components::avatar::{
    Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};

pub const BASIC_SOURCE: &str = r#"rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/shadcn.png", alt: "User" }
        AvatarFallback { "CN" }
    }
}"#;

#[component]
pub fn AvatarBasicExample() -> Element {
    rsx! {
        Avatar {
            AvatarImage { src: "https://github.com/shadcn.png", alt: "User" }
            AvatarFallback { "CN" }
        }
    }
}

pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-4",
        Avatar { size: AvatarSize::Sm,
            AvatarFallback { "SM" }
        }
        Avatar { size: AvatarSize::Default,
            AvatarFallback { "MD" }
        }
        Avatar { size: AvatarSize::Lg,
            AvatarFallback { "LG" }
        }
    }
}"#;

#[component]
pub fn AvatarSizesExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Avatar { size: AvatarSize::Sm,
                AvatarFallback { "SM" }
            }
            Avatar { size: AvatarSize::Default,
                AvatarFallback { "MD" }
            }
            Avatar { size: AvatarSize::Lg,
                AvatarFallback { "LG" }
            }
        }
    }
}

pub const GROUP_SOURCE: &str = r#"rsx! {
    AvatarGroup {
        Avatar {
            AvatarImage { src: "https://github.com/shadcn.png", alt: "User 1" }
            AvatarFallback { "U1" }
        }
        Avatar {
            AvatarFallback { "U2" }
        }
        Avatar {
            AvatarFallback { "U3" }
        }
        AvatarGroupCount { "+5" }
    }
}"#;

#[component]
pub fn AvatarGroupExample() -> Element {
    rsx! {
        AvatarGroup {
            Avatar {
                AvatarImage { src: "https://github.com/shadcn.png", alt: "User 1" }
                AvatarFallback { "U1" }
            }
            Avatar {
                AvatarFallback { "U2" }
            }
            Avatar {
                AvatarFallback { "U3" }
            }
            AvatarGroupCount { "+5" }
        }
    }
}

pub const FALLBACK_SOURCE: &str = r#"rsx! {
    div { class: "flex items-center gap-4",
        Avatar {
            AvatarImage { src: "invalid-url.jpg", alt: "User" }
            AvatarFallback { "JD" }
        }
        Avatar {
            AvatarFallback {
                lucide_dioxus::User { class: "size-4" }
            }
        }
    }
}"#;

#[component]
pub fn AvatarFallbackExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Avatar {
                AvatarImage { src: "invalid-url.jpg", alt: "User" }
                AvatarFallback { "JD" }
            }
            Avatar {
                AvatarFallback {
                    lucide_dioxus::User { class: "size-4" }
                }
            }
        }
    }
}
