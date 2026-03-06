//! Authentication example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use lumen_blocks::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
};

/// Authentication example page.
#[component]
pub fn AuthenticationExample() -> Element {
    rsx! {
        ExamplesShell {
            div { class: "md:hidden",
                img {
                    src: "/examples/authentication-light.png",
                    alt: "Authentication",
                    class: "block w-full dark:hidden",
                }
                img {
                    src: "/examples/authentication-dark.png",
                    alt: "Authentication",
                    class: "hidden w-full dark:block",
                }
            }

            div { class: "relative container hidden flex-1 shrink-0 items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0",
                a {
                    href: "/examples/authentication",
                    class: "absolute top-4 right-4 inline-flex h-9 items-center justify-center rounded-md px-4 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground md:top-8 md:right-8",
                    "Login"
                }

                div { class: "relative hidden h-full flex-col p-10 text-primary lg:flex dark:border-r",
                    div { class: "absolute inset-0 bg-primary/5" }
                    div { class: "relative z-20 flex items-center gap-2 text-lg font-medium",
                        div { class: "flex size-8 items-center justify-center rounded-full bg-primary/15", "LB" }
                        "Lumen Blocks"
                    }
                    div { class: "relative z-20 mt-auto",
                        blockquote { class: "leading-normal text-balance",
                            "\"This library has saved me countless hours of work and helped me deliver stunning designs faster than ever before.\" - Sofia Davis"
                        }
                    }
                }

                div { class: "flex items-center justify-center lg:h-[1000px] lg:p-8",
                    div { class: "mx-auto flex w-full flex-col justify-center gap-6 sm:w-[350px]",
                        div { class: "flex flex-col gap-2 text-center",
                            h1 { class: "text-2xl font-semibold tracking-tight", "Create an account" }
                            p { class: "text-sm text-muted-foreground", "Enter your email below to create your account" }
                        }

                        div { class: "grid gap-4",
                            div { class: "grid gap-2",
                                Label { for_id: "auth-email", "Email" }
                                Input { id: "auth-email", r#type: "email", placeholder: "name@example.com" }
                            }
                            div { class: "grid gap-2",
                                Label { for_id: "auth-password", "Password" }
                                Input { id: "auth-password", r#type: "password" }
                            }
                            Button { "Continue" }
                            Button { variant: ButtonVariant::Outline, "Continue with GitHub" }
                        }

                        p { class: "px-6 text-center text-xs text-muted-foreground",
                            "By clicking continue, you agree to our "
                            a { href: "/terms", class: "underline underline-offset-4 hover:text-primary", "Terms of Service" }
                            " and "
                            a { href: "/privacy", class: "underline underline-offset-4 hover:text-primary", "Privacy Policy" }
                            "."
                        }
                    }
                }
            }
        }
    }
}
