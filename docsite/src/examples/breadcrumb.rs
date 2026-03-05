//! Breadcrumb example components.

use dioxus::prelude::*;
use lumen_blocks::components::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
    BreadcrumbEllipsis,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem {
                BreadcrumbLink { href: "/", "Home" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbLink { href: "/docs", "Documentation" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbPage { "Breadcrumb" }
            }
        }
    }
}"#;

/// Basic breadcrumb example.
#[component]
pub fn BreadcrumbBasicExample() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Home" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Documentation" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Breadcrumb" }
                }
            }
        }
    }
}

/// Source code for the ellipsis example.
pub const ELLIPSIS_SOURCE: &str = r#"rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem {
                BreadcrumbLink { href: "/", "Home" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbEllipsis {}
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbLink { href: "/docs/components", "Components" }
            }
            BreadcrumbSeparator {}
            BreadcrumbItem {
                BreadcrumbPage { "Breadcrumb" }
            }
        }
    }
}"#;

/// Breadcrumb with ellipsis example.
#[component]
pub fn BreadcrumbEllipsisExample() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Home" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbEllipsis {}
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Components" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Breadcrumb" }
                }
            }
        }
    }
}

/// Source code for the custom separator example.
pub const CUSTOM_SEPARATOR_SOURCE: &str = r#"rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem {
                BreadcrumbLink { href: "/", "Home" }
            }
            BreadcrumbSeparator {
                span { "/" }
            }
            BreadcrumbItem {
                BreadcrumbLink { href: "/docs", "Docs" }
            }
            BreadcrumbSeparator {
                span { "/" }
            }
            BreadcrumbItem {
                BreadcrumbPage { "Current" }
            }
        }
    }
}"#;

/// Breadcrumb with custom separator example.
#[component]
pub fn BreadcrumbCustomSeparatorExample() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Home" }
                }
                BreadcrumbSeparator {
                    span { "/" }
                }
                BreadcrumbItem {
                    BreadcrumbLink { href: "#", "Docs" }
                }
                BreadcrumbSeparator {
                    span { "/" }
                }
                BreadcrumbItem {
                    BreadcrumbPage { "Current" }
                }
            }
        }
    }
}
