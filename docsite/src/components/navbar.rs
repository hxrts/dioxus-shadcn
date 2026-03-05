//! Navigation bar component.

use crate::pages::docs::navigation::DOCS_NAV;
use crate::Route;
use crate::LOGO_SMALL;
use dioxus::prelude::*;
use lucide_dioxus::Menu;
use lumen_blocks::components::button::{Button, ButtonVariant};
use lumen_blocks::components::side_sheet::{
    SideSheet, SideSheetCloseButton, SideSheetContent, SideSheetSide, SideSheetTrigger,
};

/// Navigation bar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        SideSheet { side: SideSheetSide::Left,
            nav {
                class: "bg-card/80 backdrop-blur-sm border-b border-border px-6 py-4 sticky top-0 z-50",

                div {
                    class: "max-w-6xl mx-auto flex items-center justify-between",

                    // Logo section
                    Link {
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        div {
                            class: "flex items-center gap-2",
                            img { class: "w-8 h-8 dark:invert", src: LOGO_SMALL }
                            span { class: "text-xl font-bold text-foreground", "dioxus-shadcn" }
                        }
                    }

                    // Desktop navigation links
                    div {
                        class: "hidden md:flex items-center gap-6",
                        Link {
                            to: Route::Home {},
                            class: "text-foreground hover:text-primary transition-colors",
                            "Home"
                        }
                        a {
                            href: "/docs",
                            class: "text-foreground hover:text-primary transition-colors",
                            "Docs"
                        }
                        a {
                            href: "https://github.com/hxrts/dioxus-shadcn",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-foreground hover:text-primary transition-colors",
                            "GitHub"
                        }
                    }

                    // Mobile menu button
                    div { class: "md:hidden",
                        SideSheetTrigger {
                            Button {
                                variant: ButtonVariant::Ghost,
                                Menu { class: "h-6 w-6" }
                            }
                        }
                    }
                }
            }

            // Mobile navigation drawer
            SideSheetContent {
                class: "p-6 w-64 h-full flex flex-col space-y-8 overflow-scroll",
                SideSheetCloseButton {}

                // Main menu
                nav {
                    class: "flex flex-col space-y-3",
                    div { class: "text-muted-foreground text-xs", "Menu" }
                    Link {
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Home"
                    }
                    a {
                        href: "/docs",
                        class: "text-foreground hover:text-primary transition-colors",
                        "Docs"
                    }
                    a {
                        href: "https://github.com/hxrts/dioxus-shadcn",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-foreground hover:text-primary transition-colors",
                        "GitHub"
                    }
                }

                // Documentation sections
                nav {
                    class: "flex flex-col space-y-3",
                    div { class: "text-muted-foreground text-xs", "Docs" }

                    for section in DOCS_NAV {
                        div { class: "space-y-1",
                            h4 { class: "text-sm font-semibold text-foreground py-1",
                                "{section.title}"
                            }

                            for item in section.items {
                                a {
                                    href: "{item.route.to_path()}",
                                    class: "block py-1 pl-2 text-sm text-muted-foreground hover:text-foreground transition-colors",
                                    "{item.title}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
