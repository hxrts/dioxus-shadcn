//! Blocks showcase page.

use crate::components::{
    Announcement, BlocksNav, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
    PageNav,
};
use dioxus::prelude::*;

const TITLE: &str = "Building Blocks for the Web";
const DESCRIPTION: &str =
    "Clean, modern building blocks. Copy and paste into your apps. Works with all React frameworks. Open Source. Free forever.";

#[derive(Clone, Copy)]
struct BlockEntry {
    id: &'static str,
    category: &'static str,
}

const FEATURED_BLOCKS: [BlockEntry; 5] = [
    BlockEntry {
        id: "dashboard-01",
        category: "dashboard",
    },
    BlockEntry {
        id: "sidebar-07",
        category: "sidebar",
    },
    BlockEntry {
        id: "sidebar-03",
        category: "sidebar",
    },
    BlockEntry {
        id: "login-03",
        category: "forms",
    },
    BlockEntry {
        id: "login-04",
        category: "forms",
    },
];

/// Featured blocks page.
#[component]
pub fn Blocks() -> Element {
    rsx! { BlocksPage { active_category: None } }
}

/// Category-filtered blocks page.
#[component]
pub fn BlocksCategory(category: String) -> Element {
    rsx! { BlocksPage { active_category: Some(category) } }
}

#[component]
fn BlocksPage(active_category: Option<String>) -> Element {
    let active = active_category.unwrap_or_default();
    let show_featured = active.is_empty();

    let visible_blocks: Vec<BlockEntry> = if show_featured {
        FEATURED_BLOCKS.to_vec()
    } else {
        FEATURED_BLOCKS
            .iter()
            .copied()
            .filter(|block| block.category == active)
            .collect()
    };

    rsx! {
        div { class: "flex flex-1 flex-col",
            PageHeader {
                Announcement {}
                PageHeaderHeading { "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    a {
                        href: "#blocks",
                        class: "inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Browse Blocks"
                    }
                    Link {
                        to: "/docs/blocks",
                        class: "inline-flex h-8 items-center justify-center rounded-md px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "Add a block"
                    }
                }
            }

            PageNav { id: "blocks",
                BlocksNav {}
                Link {
                    to: "/blocks/sidebar",
                    class: "mr-7 hidden h-8 items-center justify-center rounded-md bg-secondary px-3 text-sm font-medium text-secondary-foreground transition-colors hover:bg-secondary/80 lg:flex",
                    "Browse all blocks"
                }
            }

            div { class: "container-wrapper flex-1 section-soft md:py-12",
                div { class: "container",
                    div { class: "flex flex-col gap-12 md:gap-24",
                        for block in &visible_blocks {
                            BlockDisplayPlaceholder { name: block.id }
                        }

                        if show_featured {
                            div { class: "container-wrapper",
                                div { class: "container flex justify-center py-6",
                                    Link {
                                        to: "/blocks/sidebar",
                                        class: "inline-flex h-9 items-center justify-center rounded-md border bg-background px-4 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground",
                                        "Browse more blocks"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BlockDisplayPlaceholder(name: &'static str) -> Element {
    rsx! {
        section { class: "rounded-xl border bg-card p-5",
            div { class: "mb-4 flex items-center justify-between",
                h3 { class: "text-sm font-medium uppercase tracking-wide text-muted-foreground", "{name}" }
                span { class: "rounded-md border px-2 py-0.5 text-xs text-muted-foreground", "Preview" }
            }
            div { class: "rounded-lg border border-dashed p-8 text-center text-sm text-muted-foreground",
                "Block rendering component is not ported yet."
            }
        }
    }
}
