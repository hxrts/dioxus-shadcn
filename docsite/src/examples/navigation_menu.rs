//! NavigationMenu example components.

use dioxus::prelude::*;
use lumen_blocks::components::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuList, NavigationMenuTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    NavigationMenu {
        NavigationMenuList {
            NavigationMenuItem {
                NavigationMenuTrigger { "Getting Started" }
                NavigationMenuContent {
                    ul { class: "grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]",
                        li { class: "row-span-3",
                            NavigationMenuLink { href: "/",
                                div { class: "flex h-full w-full select-none flex-col justify-end rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none focus:shadow-md",
                                    div { class: "mb-2 mt-4 text-lg font-medium", "Lumen Blocks" }
                                    p { class: "text-sm leading-tight text-muted-foreground",
                                        "Beautifully designed components built with Dioxus and Tailwind CSS."
                                    }
                                }
                            }
                        }
                        li {
                            NavigationMenuLink { href: "/docs",
                                div { class: "text-sm font-medium leading-none", "Introduction" }
                                p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                    "Re-usable components built using Dioxus and Tailwind CSS."
                                }
                            }
                        }
                        li {
                            NavigationMenuLink { href: "/docs/installation",
                                div { class: "text-sm font-medium leading-none", "Installation" }
                                p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                    "How to install dependencies and structure your app."
                                }
                            }
                        }
                    }
                }
            }
            NavigationMenuItem {
                NavigationMenuLink { href: "/docs", "Documentation" }
            }
        }
    }
}"##;

/// Basic navigation menu example.
#[component]
pub fn NavigationMenuBasicExample() -> Element {
    rsx! {
        NavigationMenu {
            NavigationMenuList {
                NavigationMenuItem {
                    NavigationMenuTrigger { "Getting Started" }
                    NavigationMenuContent {
                        ul { class: "grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]",
                            li { class: "row-span-3",
                                NavigationMenuLink { href: "/",
                                    div { class: "flex h-full w-full select-none flex-col justify-end rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none focus:shadow-md",
                                        div { class: "mb-2 mt-4 text-lg font-medium", "Lumen Blocks" }
                                        p { class: "text-sm leading-tight text-muted-foreground",
                                            "Beautifully designed components built with Dioxus and Tailwind CSS."
                                        }
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "/docs",
                                    div { class: "text-sm font-medium leading-none", "Introduction" }
                                    p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                        "Re-usable components built using Dioxus and Tailwind CSS."
                                    }
                                }
                            }
                            li {
                                NavigationMenuLink { href: "/docs/installation",
                                    div { class: "text-sm font-medium leading-none", "Installation" }
                                    p { class: "line-clamp-2 text-sm leading-snug text-muted-foreground",
                                        "How to install dependencies and structure your app."
                                    }
                                }
                            }
                        }
                    }
                }
                NavigationMenuItem {
                    NavigationMenuLink { href: "/docs", "Documentation" }
                }
            }
        }
    }
}
