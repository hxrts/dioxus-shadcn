//! Progress component examples and source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::progress::{Progress, ProgressSize, ProgressVariant};

pub const BASIC_SOURCE: &str = r#"rsx! {
    Progress { value: use_signal(|| 60.0) }
}"#;

#[component]
pub fn ProgressBasicExample() -> Element {
    rsx! {
        Progress { value: use_signal(|| 60.0) }
    }
}

pub const SIZES_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Progress { value: use_signal(|| 40.0), size: ProgressSize::Small }
        Progress { value: use_signal(|| 60.0), size: ProgressSize::Medium }
        Progress { value: use_signal(|| 80.0), size: ProgressSize::Large }
    }
}"#;

#[component]
pub fn ProgressSizesExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Progress { value: use_signal(|| 40.0), size: ProgressSize::Small }
            Progress { value: use_signal(|| 60.0), size: ProgressSize::Medium }
            Progress { value: use_signal(|| 80.0), size: ProgressSize::Large }
        }
    }
}

pub const VARIANTS_SOURCE: &str = r#"rsx! {
    div { class: "grid gap-4",
        Progress { value: use_signal(|| 25.0), variant: ProgressVariant::Default }
        Progress { value: use_signal(|| 50.0), variant: ProgressVariant::Success }
        Progress { value: use_signal(|| 75.0), variant: ProgressVariant::Warning }
        Progress { value: use_signal(|| 100.0), variant: ProgressVariant::Destructive }
    }
}"#;

#[component]
pub fn ProgressVariantsExample() -> Element {
    rsx! {
        div { class: "grid gap-4",
            Progress { value: use_signal(|| 25.0), variant: ProgressVariant::Default }
            Progress { value: use_signal(|| 50.0), variant: ProgressVariant::Success }
            Progress { value: use_signal(|| 75.0), variant: ProgressVariant::Warning }
            Progress { value: use_signal(|| 100.0), variant: ProgressVariant::Destructive }
        }
    }
}

pub const WITH_PERCENTAGE_SOURCE: &str = r#"rsx! {
    Progress {
        value: use_signal(|| 66.0),
        show_percentage: true,
        aria_label: "Download progress",
    }
}"#;

#[component]
pub fn ProgressWithPercentageExample() -> Element {
    rsx! {
        Progress {
            value: use_signal(|| 66.0),
            show_percentage: true,
            aria_label: "Download progress",
        }
    }
}
