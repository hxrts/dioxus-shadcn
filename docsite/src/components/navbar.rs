//! Navigation bar component - matches shadcn-ui v4 site-header style exactly.

use crate::components::ThemeToggle;
use crate::LOGO_SMALL;
use dioxus::prelude::*;
use lucide_dioxus::{Github, Menu, X};
use lumen_blocks::components::button::{Button, ButtonSize, ButtonVariant};
use lumen_blocks::components::separator::{Separator, SeparatorOrientation};

// Button styling classes matching shadcn Button component
const BTN_BASE: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50";
const BTN_GHOST: &str = "hover:bg-accent hover:text-accent-foreground";
const BTN_SM: &str = "h-8 gap-1.5 px-3";
const BTN_ICON: &str = "size-8";
const BTN_ICON_SM: &str = "size-7";

/// Signal for mobile menu visibility.
static MOBILE_MENU_OPEN: GlobalSignal<bool> = Signal::global(|| false);

/// Navigation bar component - matches shadcn/ui v4 header style exactly.
#[component]
pub fn Navbar() -> Element {
    let is_mobile_menu_open = *MOBILE_MENU_OPEN.read();

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full bg-background",

            div {
                class: "container-wrapper px-6",

                div {
                    class: "flex h-14 items-center",

                    // Mobile menu button (visible on mobile only)
                    div { class: "flex lg:hidden",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            class: "size-8",
                            on_click: move |_| {
                                *MOBILE_MENU_OPEN.write() = !is_mobile_menu_open;
                            },
                            if is_mobile_menu_open {
                                X { class: "size-5" }
                            } else {
                                Menu { class: "size-5" }
                            }
                        }
                    }

                    // Logo - styled anchor matching shadcn Button ghost icon
                    a {
                        href: "/",
                        class: "{BTN_BASE} {BTN_GHOST} {BTN_ICON} hidden lg:flex",
                        img { class: "size-5 dark:invert", src: LOGO_SMALL }
                        span { class: "sr-only", "dioxus-shadcn" }
                    }

                    // Desktop navigation links (MainNav) - matches reference exactly
                    nav {
                        class: "hidden items-center gap-0 lg:flex",

                        NavButton { href: "/docs", "Docs" }
                        NavButton { href: "/docs/components/accordion", "Components" }
                        NavButton { href: "/blocks", "Blocks" }
                        NavButton { href: "/themes", "Themes" }
                        NavButton { href: "/examples/authentication", "Examples" }
                    }

                    // Right side - spacer and actions
                    div { class: "ml-auto flex items-center gap-2 md:flex-1 md:justify-end",
                        // Separator before GitHub
                        Separator { orientation: SeparatorOrientation::Vertical, class: "ml-2 hidden h-4 lg:block" }

                        // GitHub link - styled anchor matching shadcn Button ghost icon-sm
                        a {
                            href: "https://github.com/hxrts/dioxus-shadcn",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "{BTN_BASE} {BTN_GHOST} {BTN_ICON_SM}",
                            Github { class: "size-4" }
                            span { class: "sr-only", "GitHub" }
                        }

                        // Separator
                        Separator { orientation: SeparatorOrientation::Vertical, class: "h-4" }

                        // Theme toggle
                        ThemeToggle {}
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

/// Navigation button component - styled anchor matching shadcn MainNav Button ghost sm.
#[component]
fn NavButton(href: &'static str, children: Element) -> Element {
    // Reference uses Button variant="ghost" size="sm" className="px-2.5"
    // sm size: h-8 gap-1.5 rounded-md px-3 -> override to px-2.5
    rsx! {
        a {
            href: href,
            class: "{BTN_BASE} {BTN_GHOST} {BTN_SM} px-2.5",
            {children}
        }
    }
}

/// Mobile navigation overlay.
#[component]
fn MobileNav() -> Element {
    rsx! {
        div {
            class: "fixed inset-0 top-14 z-50 lg:hidden",

            // Backdrop
            div {
                class: "fixed inset-0 top-14 bg-background/80 backdrop-blur-sm",
                onclick: move |_| {
                    *MOBILE_MENU_OPEN.write() = false;
                },
            }

            // Menu content
            nav {
                class: "fixed top-14 left-0 bottom-0 w-full max-w-xs border-r border-border bg-background p-6 shadow-lg",

                div { class: "flex flex-col gap-4",
                    MobileNavLink { href: "/", "Home" }
                    MobileNavLink { href: "/docs", "Docs" }
                    MobileNavLink { href: "/docs/components/accordion", "Components" }
                    MobileNavLink { href: "/blocks", "Blocks" }
                    MobileNavLink { href: "/themes", "Themes" }
                    MobileNavLink { href: "/examples/authentication", "Examples" }
                    a {
                        href: "https://github.com/hxrts/dioxus-shadcn",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "font-medium text-foreground transition-colors hover:text-foreground/80",
                        onclick: move |_| { *MOBILE_MENU_OPEN.write() = false; },
                        "GitHub"
                    }
                }
            }
        }
    }
}

/// Mobile navigation link.
#[component]
fn MobileNavLink(href: &'static str, children: Element) -> Element {
    rsx! {
        a {
            href: href,
            class: "font-medium text-foreground transition-colors hover:text-foreground/80",
            onclick: move |_| { *MOBILE_MENU_OPEN.write() = false; },
            {children}
        }
    }
}
