//! Dropdown component examples with embedded source code.

use dioxus::prelude::*;
use lucide_dioxus::{LogOut, Settings, User};
use dioxus_shadcn::components::button::{Button, ButtonVariant};
use dioxus_shadcn::components::dropdown::{
    Dropdown, DropdownContent, DropdownItem, DropdownLabel, DropdownSeparator, DropdownTrigger,
};

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use dioxus_shadcn::components::dropdown::{
    Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
};
use dioxus_shadcn::components::button::{Button, ButtonVariant};

rsx! {
    Dropdown {
        DropdownTrigger {
            Button { variant: ButtonVariant::Outline, "Open Menu" }
        }
        DropdownContent {
            DropdownItem { value: ReadSignal::new(Signal::new("profile")), "Profile" }
            DropdownItem { value: ReadSignal::new(Signal::new("settings")), "Settings" }
            DropdownItem { value: ReadSignal::new(Signal::new("logout")), "Log out" }
        }
    }
}"#;

pub const WITH_ICONS_SOURCE: &str = r#"use lucide_dioxus::{User, Settings, LogOut};
use dioxus_shadcn::components::dropdown::{
    Dropdown, DropdownContent, DropdownItem, DropdownLabel, DropdownSeparator, DropdownTrigger,
};

rsx! {
    Dropdown {
        DropdownTrigger {
            Button { variant: ButtonVariant::Outline, "My Account" }
        }
        DropdownContent {
            DropdownLabel { "My Account" }
            DropdownSeparator {}
            DropdownItem {
                icon: Some(rsx! { User { size: 16 } }),
                "Profile"
            }
            DropdownItem {
                icon: Some(rsx! { Settings { size: 16 } }),
                "Settings"
            }
            DropdownSeparator {}
            DropdownItem {
                icon: Some(rsx! { LogOut { size: 16 } }),
                "Log out"
            }
        }
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn DropdownBasicExample() -> Element {
    rsx! {
        Dropdown {
            DropdownTrigger {
                Button { variant: ButtonVariant::Outline, "Open Menu" }
            }
            DropdownContent {
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("profile")),
                    on_select: |_| {},
                    "Profile"
                }
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("settings")),
                    on_select: |_| {},
                    "Settings"
                }
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("logout")),
                    on_select: |_| {},
                    "Log out"
                }
            }
        }
    }
}

#[component]
pub fn DropdownWithIconsExample() -> Element {
    rsx! {
        Dropdown {
            DropdownTrigger {
                Button { variant: ButtonVariant::Outline, "My Account" }
            }
            DropdownContent {
                DropdownLabel { "My Account" }
                DropdownSeparator {}
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("profile")),
                    on_select: |_| {},
                    icon: Some(rsx! { User { size: 16 } }),
                    "Profile"
                }
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("settings")),
                    on_select: |_| {},
                    icon: Some(rsx! { Settings { size: 16 } }),
                    "Settings"
                }
                DropdownSeparator {}
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("logout")),
                    on_select: |_| {},
                    icon: Some(rsx! { LogOut { size: 16 } }),
                    "Log out"
                }
            }
        }
    }
}

#[component]
pub fn DropdownDestructiveExample() -> Element {
    rsx! {
        Dropdown {
            DropdownTrigger {
                Button { variant: ButtonVariant::Outline, "Actions" }
            }
            DropdownContent {
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("edit")),
                    on_select: |_| {},
                    "Edit"
                }
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("duplicate")),
                    on_select: |_| {},
                    "Duplicate"
                }
                DropdownSeparator {}
                DropdownItem::<&str> {
                    value: ReadSignal::new(Signal::new("delete")),
                    on_select: |_| {},
                    destructive: true,
                    "Delete"
                }
            }
        }
    }
}
