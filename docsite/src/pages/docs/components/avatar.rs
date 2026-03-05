//! Avatar component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::avatar::*;
use dioxus::prelude::*;

/// Avatar documentation page.
#[component]
pub fn AvatarDoc() -> Element {
    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Avatar",
                description: "An image element with a fallback for representing a user.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: r#"use lumen_blocks::components::avatar::{
    Avatar, AvatarImage, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarSize
};"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: r#"rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/user.png", alt: "User" }
        AvatarFallback { "JD" }
    }
}"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        filename: Some("avatar_basic.rs".to_string()),
                        AvatarBasicExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "sizes", class: "text-xl font-medium", "Sizes" }
                    ComponentPreview {
                        source: SIZES_SOURCE.to_string(),
                        filename: Some("avatar_sizes.rs".to_string()),
                        AvatarSizesExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "group", class: "text-xl font-medium", "Avatar Group" }
                    p { class: "text-muted-foreground", "Display multiple avatars in a stacked layout." }
                    ComponentPreview {
                        source: GROUP_SOURCE.to_string(),
                        filename: Some("avatar_group.rs".to_string()),
                        AvatarGroupExample {}
                    }
                }

                div { class: "space-y-4",
                    h3 { id: "fallback", class: "text-xl font-medium", "Fallback" }
                    p { class: "text-muted-foreground", "Show initials or an icon when the image fails to load." }
                    ComponentPreview {
                        source: FALLBACK_SOURCE.to_string(),
                        filename: Some("avatar_fallback.rs".to_string()),
                        AvatarFallbackExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }
                div { class: "overflow-x-auto",
                    table { class: "w-full text-sm",
                        thead {
                            tr { class: "border-b border-border",
                                th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                th { class: "py-3 px-4 text-left font-medium", "Type" }
                                th { class: "py-3 px-4 text-left font-medium", "Default" }
                                th { class: "py-3 px-4 text-left font-medium", "Description" }
                            }
                        }
                        tbody {
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "size" }
                                td { class: "py-3 px-4 font-mono text-xs", "AvatarSize" }
                                td { class: "py-3 px-4 font-mono text-xs", "Default" }
                                td { class: "py-3 px-4 text-muted-foreground", "Size variant (Sm, Default, Lg)" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "class" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<String>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Additional CSS classes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
