//! AspectRatio example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::aspect_ratio::AspectRatio;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    AspectRatio { ratio: 16.0 / 9.0,
        img {
            src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800",
            alt: "Photo",
            class: "rounded-md object-cover w-full h-full",
        }
    }
}"#;

/// Basic aspect ratio example.
#[component]
pub fn AspectRatioBasicExample() -> Element {
    rsx! {
        div { class: "w-[400px]",
            AspectRatio { ratio: 16.0 / 9.0,
                img {
                    src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800",
                    alt: "Photo",
                    class: "rounded-md object-cover w-full h-full",
                }
            }
        }
    }
}

/// Source code for the square example.
pub const SQUARE_SOURCE: &str = r#"rsx! {
    AspectRatio { ratio: 1.0,
        img {
            src: "https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300",
            alt: "Photo",
            class: "rounded-md object-cover w-full h-full",
        }
    }
}"#;

/// Square aspect ratio example.
#[component]
pub fn AspectRatioSquareExample() -> Element {
    rsx! {
        div { class: "w-[200px]",
            AspectRatio { ratio: 1.0,
                img {
                    src: "https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300",
                    alt: "Photo",
                    class: "rounded-md object-cover w-full h-full",
                }
            }
        }
    }
}

/// Source code for the portrait example.
pub const PORTRAIT_SOURCE: &str = r#"rsx! {
    AspectRatio { ratio: 3.0 / 4.0,
        div {
            class: "flex items-center justify-center w-full h-full bg-muted rounded-md",
            "3:4 Portrait"
        }
    }
}"#;

/// Portrait aspect ratio example.
#[component]
pub fn AspectRatioPortraitExample() -> Element {
    rsx! {
        div { class: "w-[200px]",
            AspectRatio { ratio: 3.0 / 4.0,
                div {
                    class: "flex items-center justify-center w-full h-full bg-muted rounded-md",
                    "3:4 Portrait"
                }
            }
        }
    }
}
