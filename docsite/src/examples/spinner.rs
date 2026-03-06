//! Spinner example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::spinner::{Spinner, SpinnerSize};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Spinner {}
}"##;

/// Source code for sizes example.
pub const SIZES_SOURCE: &str = r##"rsx! {
    div { class: "flex items-center gap-4",
        Spinner { size: SpinnerSize::Xs }
        Spinner { size: SpinnerSize::Sm }
        Spinner { size: SpinnerSize::Md }
        Spinner { size: SpinnerSize::Lg }
        Spinner { size: SpinnerSize::Xl }
    }
}"##;

/// Basic spinner example.
#[component]
pub fn SpinnerBasicExample() -> Element {
    rsx! {
        Spinner {}
    }
}

/// Spinner sizes example.
#[component]
pub fn SpinnerSizesExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            Spinner { size: SpinnerSize::Xs }
            Spinner { size: SpinnerSize::Sm }
            Spinner { size: SpinnerSize::Md }
            Spinner { size: SpinnerSize::Lg }
            Spinner { size: SpinnerSize::Xl }
        }
    }
}
