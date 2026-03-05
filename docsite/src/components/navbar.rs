//! Navigation bar component.

use crate::Route;
use crate::LOGO_SMALL;
use dioxus::prelude::*;
use lucide_dioxus::{Menu, X};

/// Signal for mobile menu visibility.
static MOBILE_MENU_OPEN: GlobalSignal<bool> = Signal::global(|| false);

/// Navigation bar component - matches shadcn/ui header style.
#[component]
pub fn Navbar() -> Element {
    let is_mobile_menu_open = *MOBILE_MENU_OPEN.read();

    rsx! {
        // Main header bar
        header {
            class: "sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60",

            div {
                class: "container flex h-14 max-w-screen-2xl items-center px-4 lg:px-8",

                // Logo section
                Link {
                    to: Route::Home {},
                    class: "mr-6 flex items-center space-x-2",
                    img { class: "h-6 w-6 dark:invert", src: LOGO_SMALL }
                    span { class: "hidden font-bold sm:inline-block", "dioxus-shadcn" }
                }

                // Desktop navigation links
                nav {
                    class: "hidden md:flex items-center gap-4 text-sm lg:gap-6",
                    a {
                        href: "/docs",
                        class: "text-foreground/60 transition-colors hover:text-foreground/80",
                        "Docs"
                    }
                    a {
                        href: "/docs/components/accordion",
                        class: "text-foreground/60 transition-colors hover:text-foreground/80",
                        "Components"
                    }
                    a {
                        href: "https://github.com/hxrts/dioxus-shadcn",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-foreground/60 transition-colors hover:text-foreground/80",
                        "GitHub"
                    }
                }

                // Spacer
                div { class: "flex flex-1 items-center justify-end space-x-2" }

                // Mobile menu button
                div { class: "md:hidden",
                    button {
                        class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9 px-0",
                        onclick: move |_| {
                            *MOBILE_MENU_OPEN.write() = !is_mobile_menu_open;
                        },
                        if is_mobile_menu_open {
                            X { class: "h-5 w-5" }
                        } else {
                            Menu { class: "h-5 w-5" }
                        }
                        span { class: "sr-only", "Toggle menu" }
                    }
                }
            }
        }

        // Mobile navigation overlay
        if is_mobile_menu_open {
            MobileNav {}
        }
    }
}

/// Mobile navigation overlay.
#[component]
fn MobileNav() -> Element {
    rsx! {
        div {
            class: "fixed inset-0 top-14 z-50 md:hidden",

            // Backdrop
            div {
                class: "fixed inset-0 top-14 bg-background/80 backdrop-blur-sm",
                onclick: move |_| {
                    *MOBILE_MENU_OPEN.write() = false;
                },
            }

            // Menu content
            nav {
                class: "fixed top-14 left-0 bottom-0 w-full max-w-xs border-r bg-background p-6 shadow-lg",

                div { class: "flex flex-col space-y-4",
                    Link {
                        to: Route::Home {},
                        class: "text-foreground font-medium",
                        onclick: move |_| { *MOBILE_MENU_OPEN.write() = false; },
                        "Home"
                    }
                    a {
                        href: "/docs",
                        class: "text-foreground font-medium",
                        onclick: move |_| { *MOBILE_MENU_OPEN.write() = false; },
                        "Docs"
                    }
                    a {
                        href: "/docs/components/accordion",
                        class: "text-foreground font-medium",
                        onclick: move |_| { *MOBILE_MENU_OPEN.write() = false; },
                        "Components"
                    }
                    a {
                        href: "https://github.com/hxrts/dioxus-shadcn",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-foreground font-medium",
                        "GitHub"
                    }
                }
            }
        }
    }
}
