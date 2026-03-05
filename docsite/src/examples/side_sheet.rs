//! SideSheet example components.

use dioxus::prelude::*;
use lumen_blocks::components::side_sheet::{
    SideSheet, SideSheetContent, SideSheetDescription, SideSheetHeader, SideSheetTitle,
    SideSheetTrigger, SideSheetFooter, SideSheetSide, SideSheetCloseButton,
};
use lumen_blocks::components::button::Button;
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    SideSheet {
        SideSheetTrigger {
            Button { variant: ButtonVariant::Outline, "Open Sheet" }
        }
        SideSheetContent {
            SideSheetCloseButton {}
            SideSheetHeader {
                SideSheetTitle { "Edit Profile" }
                SideSheetDescription {
                    "Make changes to your profile here. Click save when you're done."
                }
            }
            div { class: "grid gap-4 p-4",
                div { class: "grid grid-cols-4 items-center gap-4",
                    Label { for_id: "name", class: "text-right", "Name" }
                    Input { id: "name", value: "John Doe", class: "col-span-3" }
                }
                div { class: "grid grid-cols-4 items-center gap-4",
                    Label { for_id: "username", class: "text-right", "Username" }
                    Input { id: "username", value: "@johndoe", class: "col-span-3" }
                }
            }
            SideSheetFooter {
                Button { "Save changes" }
            }
        }
    }
}"#;

/// Basic side sheet example.
#[component]
pub fn SideSheetBasicExample() -> Element {
    use lumen_blocks::components::button::ButtonVariant;

    rsx! {
        SideSheet {
            SideSheetTrigger {
                Button { variant: ButtonVariant::Outline, "Open Sheet" }
            }
            SideSheetContent {
                SideSheetCloseButton {}
                SideSheetHeader {
                    SideSheetTitle { "Edit Profile" }
                    SideSheetDescription {
                        "Make changes to your profile here. Click save when you're done."
                    }
                }
                div { class: "grid gap-4 p-4",
                    div { class: "grid grid-cols-4 items-center gap-4",
                        Label { for_id: "name", class: "text-right", "Name" }
                        Input { id: "name", placeholder: "John Doe", class: "col-span-3" }
                    }
                    div { class: "grid grid-cols-4 items-center gap-4",
                        Label { for_id: "username", class: "text-right", "Username" }
                        Input { id: "username", placeholder: "@johndoe", class: "col-span-3" }
                    }
                }
                SideSheetFooter {
                    Button { "Save changes" }
                }
            }
        }
    }
}

/// Source code for the sides example.
pub const SIDES_SOURCE: &str = r#"rsx! {
    div { class: "flex gap-2",
        SideSheet { side: SideSheetSide::Left,
            SideSheetTrigger {
                Button { variant: ButtonVariant::Outline, "Left" }
            }
            SideSheetContent {
                SideSheetCloseButton {}
                SideSheetHeader {
                    SideSheetTitle { "Left Sheet" }
                    SideSheetDescription { "This sheet opens from the left." }
                }
            }
        }
        SideSheet { side: SideSheetSide::Right,
            SideSheetTrigger {
                Button { variant: ButtonVariant::Outline, "Right" }
            }
            SideSheetContent {
                SideSheetCloseButton {}
                SideSheetHeader {
                    SideSheetTitle { "Right Sheet" }
                    SideSheetDescription { "This sheet opens from the right." }
                }
            }
        }
        SideSheet { side: SideSheetSide::Top,
            SideSheetTrigger {
                Button { variant: ButtonVariant::Outline, "Top" }
            }
            SideSheetContent {
                SideSheetCloseButton {}
                SideSheetHeader {
                    SideSheetTitle { "Top Sheet" }
                    SideSheetDescription { "This sheet opens from the top." }
                }
            }
        }
        SideSheet { side: SideSheetSide::Bottom,
            SideSheetTrigger {
                Button { variant: ButtonVariant::Outline, "Bottom" }
            }
            SideSheetContent {
                SideSheetCloseButton {}
                SideSheetHeader {
                    SideSheetTitle { "Bottom Sheet" }
                    SideSheetDescription { "This sheet opens from the bottom." }
                }
            }
        }
    }
}"#;

/// Side sheet positions example.
#[component]
pub fn SideSheetSidesExample() -> Element {
    use lumen_blocks::components::button::ButtonVariant;

    rsx! {
        div { class: "flex gap-2 flex-wrap",
            SideSheet { side: SideSheetSide::Left,
                SideSheetTrigger {
                    Button { variant: ButtonVariant::Outline, "Left" }
                }
                SideSheetContent {
                    SideSheetCloseButton {}
                    SideSheetHeader {
                        SideSheetTitle { "Left Sheet" }
                        SideSheetDescription { "This sheet opens from the left." }
                    }
                }
            }
            SideSheet { side: SideSheetSide::Right,
                SideSheetTrigger {
                    Button { variant: ButtonVariant::Outline, "Right" }
                }
                SideSheetContent {
                    SideSheetCloseButton {}
                    SideSheetHeader {
                        SideSheetTitle { "Right Sheet" }
                        SideSheetDescription { "This sheet opens from the right." }
                    }
                }
            }
            SideSheet { side: SideSheetSide::Top,
                SideSheetTrigger {
                    Button { variant: ButtonVariant::Outline, "Top" }
                }
                SideSheetContent {
                    SideSheetCloseButton {}
                    SideSheetHeader {
                        SideSheetTitle { "Top Sheet" }
                        SideSheetDescription { "This sheet opens from the top." }
                    }
                }
            }
            SideSheet { side: SideSheetSide::Bottom,
                SideSheetTrigger {
                    Button { variant: ButtonVariant::Outline, "Bottom" }
                }
                SideSheetContent {
                    SideSheetCloseButton {}
                    SideSheetHeader {
                        SideSheetTitle { "Bottom Sheet" }
                        SideSheetDescription { "This sheet opens from the bottom." }
                    }
                }
            }
        }
    }
}
