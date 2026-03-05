//! Home page component - matches shadcn-ui v4 home layout exactly.

use dioxus::prelude::*;

// Button styling classes matching shadcn Button component
const BTN_BASE: &str = "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50";
const BTN_DEFAULT: &str = "bg-primary text-primary-foreground hover:bg-primary/90";
const BTN_GHOST: &str = "hover:bg-accent hover:text-accent-foreground";
const BTN_OUTLINE: &str = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground";
const BTN_SM: &str = "h-8 gap-1.5 px-3 text-sm";

/// Home page component - matches shadcn/ui v4 structure exactly.
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col",
            // PageHeader section - matches reference exactly
            section { class: "border-grid",
                div { class: "container-wrapper",
                    div { class: "container flex flex-col items-center gap-2 px-6 py-8 text-center md:py-16 lg:py-20 xl:gap-4",
                        // Heading
                        h1 {
                            class: "leading-tighter max-w-4xl text-3xl font-semibold tracking-tight text-balance text-primary lg:leading-[1.1] lg:font-semibold xl:text-5xl xl:tracking-tighter",
                            "Accessible Components for Dioxus"
                        }

                        // Description
                        p {
                            class: "max-w-4xl text-base text-balance text-foreground sm:text-lg",
                            "A set of beautifully designed, accessible components built with Dioxus Primitives. Styled with Tailwind CSS v4. Open Source."
                        }

                        // Actions - use anchor elements styled as buttons
                        div {
                            class: "flex w-full items-center justify-center gap-2 pt-2",
                            a {
                                href: "/docs",
                                class: "{BTN_BASE} {BTN_DEFAULT} {BTN_SM} h-[31px] rounded-lg",
                                "Get Started"
                            }
                            a {
                                href: "/docs/components/accordion",
                                class: "{BTN_BASE} {BTN_GHOST} {BTN_SM} rounded-lg",
                                "View Components"
                            }
                        }
                    }
                }
            }

            // Main content area with gradient background
            div { class: "container-wrapper flex-1 section-soft pb-6",
                div { class: "container overflow-hidden",
                    // Component showcase section
                    section { class: "py-8 md:py-12",
                        // Placeholder for component demos
                        div { class: "rounded-lg border border-border/50 bg-card p-8",
                            div { class: "flex flex-col items-center justify-center gap-4 text-center",
                                h2 { class: "text-2xl font-semibold tracking-tight",
                                    "Component Library"
                                }
                                p { class: "max-w-2xl text-muted-foreground",
                                    "40+ accessible components including buttons, dialogs, tabs, dropdowns, and more. Built on Dioxus Primitives with full keyboard navigation and screen reader support."
                                }
                                div { class: "flex gap-2 pt-4",
                                    a {
                                        href: "/docs/components/button",
                                        class: "{BTN_BASE} {BTN_OUTLINE} {BTN_SM}",
                                        "Browse Components"
                                    }
                                    a {
                                        href: "/blocks",
                                        class: "{BTN_BASE} {BTN_GHOST} {BTN_SM}",
                                        "View Blocks"
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
