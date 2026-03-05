//! Pagination example components.

use dioxus::prelude::*;
use lumen_blocks::components::pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Pagination {
        PaginationContent {
            PaginationItem {
                PaginationPrevious { href: "#" }
            }
            PaginationItem {
                PaginationLink { href: "#", "1" }
            }
            PaginationItem {
                PaginationLink { href: "#", is_active: true, "2" }
            }
            PaginationItem {
                PaginationLink { href: "#", "3" }
            }
            PaginationItem {
                PaginationEllipsis {}
            }
            PaginationItem {
                PaginationNext { href: "#" }
            }
        }
    }
}"##;

/// Basic pagination example.
#[component]
pub fn PaginationBasicExample() -> Element {
    rsx! {
        Pagination {
            PaginationContent {
                PaginationItem {
                    PaginationPrevious { href: "#" }
                }
                PaginationItem {
                    PaginationLink { href: "#", "1" }
                }
                PaginationItem {
                    PaginationLink { href: "#", is_active: true, "2" }
                }
                PaginationItem {
                    PaginationLink { href: "#", "3" }
                }
                PaginationItem {
                    PaginationEllipsis {}
                }
                PaginationItem {
                    PaginationNext { href: "#" }
                }
            }
        }
    }
}
