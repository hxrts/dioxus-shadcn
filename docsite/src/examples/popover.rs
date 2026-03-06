//! Popover example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::button::Button;
use dioxus_shadcn::components::input::Input;
use dioxus_shadcn::components::label::Label;
use dioxus_shadcn::components::popover::{
    Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle, PopoverTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    Popover {
        PopoverTrigger {
            Button { variant: ButtonVariant::Outline, "Open popover" }
        }
        PopoverContent {
            PopoverHeader {
                PopoverTitle { "Dimensions" }
                PopoverDescription { "Set the dimensions for the layer." }
            }
            div { class: "grid gap-4 pt-4",
                div { class: "grid grid-cols-3 items-center gap-4",
                    Label { for_id: "width", "Width" }
                    Input { id: "width", placeholder: "100%", class: "col-span-2 h-8" }
                }
                div { class: "grid grid-cols-3 items-center gap-4",
                    Label { for_id: "height", "Height" }
                    Input { id: "height", placeholder: "25px", class: "col-span-2 h-8" }
                }
            }
        }
    }
}"#;

/// Basic popover example.
#[component]
pub fn PopoverBasicExample() -> Element {
    use dioxus_shadcn::components::button::ButtonVariant;

    rsx! {
        Popover {
            PopoverTrigger {
                Button { variant: ButtonVariant::Outline, "Open popover" }
            }
            PopoverContent {
                PopoverHeader {
                    PopoverTitle { "Dimensions" }
                    PopoverDescription { "Set the dimensions for the layer." }
                }
                div { class: "grid gap-4 pt-4",
                    div { class: "grid grid-cols-3 items-center gap-4",
                        Label { for_id: "width", "Width" }
                        Input { id: "width", placeholder: "100%", class: "col-span-2 h-8" }
                    }
                    div { class: "grid grid-cols-3 items-center gap-4",
                        Label { for_id: "height", "Height" }
                        Input { id: "height", placeholder: "25px", class: "col-span-2 h-8" }
                    }
                }
            }
        }
    }
}

/// Source code for the positioning example.
pub const POSITIONING_SOURCE: &str = r#"rsx! {
    div { class: "flex gap-4",
        Popover {
            PopoverTrigger {
                Button { variant: ButtonVariant::Outline, "Top" }
            }
            PopoverContent {
                side: PopoverSide::Top,
                "Content appears on top"
            }
        }
        Popover {
            PopoverTrigger {
                Button { variant: ButtonVariant::Outline, "Right" }
            }
            PopoverContent {
                side: PopoverSide::Right,
                "Content appears on right"
            }
        }
        Popover {
            PopoverTrigger {
                Button { variant: ButtonVariant::Outline, "Bottom" }
            }
            PopoverContent {
                side: PopoverSide::Bottom,
                "Content appears on bottom"
            }
        }
    }
}"#;

/// Popover positioning example.
#[component]
pub fn PopoverPositioningExample() -> Element {
    use dioxus_shadcn::components::button::ButtonVariant;
    use dioxus_shadcn::components::popover::PopoverSide;

    rsx! {
        div { class: "flex gap-4",
            Popover {
                PopoverTrigger {
                    Button { variant: ButtonVariant::Outline, "Top" }
                }
                PopoverContent {
                    side: PopoverSide::Top,
                    p { "Content appears on top" }
                }
            }
            Popover {
                PopoverTrigger {
                    Button { variant: ButtonVariant::Outline, "Right" }
                }
                PopoverContent {
                    side: PopoverSide::Right,
                    p { "Content appears on right" }
                }
            }
            Popover {
                PopoverTrigger {
                    Button { variant: ButtonVariant::Outline, "Bottom" }
                }
                PopoverContent {
                    side: PopoverSide::Bottom,
                    p { "Content appears on bottom" }
                }
            }
        }
    }
}
