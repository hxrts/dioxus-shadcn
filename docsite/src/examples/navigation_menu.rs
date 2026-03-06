//! NavigationMenu example components.

use dioxus::prelude::*;
use dioxus_shadcn::components::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuList, NavigationMenuTrigger,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    NavigationMenu {
        NavigationMenuList {
            NavigationMenuItem { value: "getting-started",
                NavigationMenuTrigger { "Getting started" }
                NavigationMenuContent { class: "p-4 md:w-[400px] lg:w-[500px]",
                    div { class: "grid gap-3",
                        NavigationMenuLink { href: "/docs",
                            div { class: "text-sm font-medium", "Introduction" }
                            p { class: "text-sm text-muted-foreground",
                                "Re-usable components built with Dioxus and Tailwind CSS."
                            }
                        }
                        NavigationMenuLink { href: "/docs/installation",
                            div { class: "text-sm font-medium", "Installation" }
                            p { class: "text-sm text-muted-foreground",
                                "How to install dependencies and structure your app."
                            }
                        }
                    }
                }
            }
            NavigationMenuItem { value: "components",
                NavigationMenuTrigger { "Components" }
                NavigationMenuContent { class: "p-4 md:w-[400px]",
                    div { class: "grid gap-3",
                        NavigationMenuLink { href: "/docs/components/button",
                            div { class: "text-sm font-medium", "Button" }
                            p { class: "text-sm text-muted-foreground",
                                "Displays a button or a component that looks like a button."
                            }
                        }
                        NavigationMenuLink { href: "/docs/components/dialog",
                            div { class: "text-sm font-medium", "Dialog" }
                            p { class: "text-sm text-muted-foreground",
                                "A window overlaid on the primary content."
                            }
                        }
                    }
                }
            }
            NavigationMenuItem {
                NavigationMenuLink { href: "/docs",
                    "Documentation"
                }
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
                NavigationMenuItem { value: "getting-started",
                    NavigationMenuTrigger { "Getting started" }
                    NavigationMenuContent { class: "p-4 md:w-[400px] lg:w-[500px]",
                        div { class: "grid gap-3",
                            NavigationMenuLink { href: "#",
                                div { class: "text-sm font-medium", "Introduction" }
                                p { class: "text-sm text-muted-foreground",
                                    "Re-usable components built with Dioxus and Tailwind CSS."
                                }
                            }
                            NavigationMenuLink { href: "#",
                                div { class: "text-sm font-medium", "Installation" }
                                p { class: "text-sm text-muted-foreground",
                                    "How to install dependencies and structure your app."
                                }
                            }
                        }
                    }
                }
                NavigationMenuItem { value: "components",
                    NavigationMenuTrigger { "Components" }
                    NavigationMenuContent { class: "p-4 md:w-[400px]",
                        div { class: "grid gap-3",
                            NavigationMenuLink { href: "#",
                                div { class: "text-sm font-medium", "Button" }
                                p { class: "text-sm text-muted-foreground",
                                    "Displays a button or a component that looks like a button."
                                }
                            }
                            NavigationMenuLink { href: "#",
                                div { class: "text-sm font-medium", "Dialog" }
                                p { class: "text-sm text-muted-foreground",
                                    "A window overlaid on the primary content."
                                }
                            }
                        }
                    }
                }
                NavigationMenuItem {
                    NavigationMenuLink { href: "#",
                        "Documentation"
                    }
                }
            }
        }
    }
}
