//! Authentication example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use dioxus_shadcn::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
    separator::Separator,
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
                    width: "1280",
                    height: "843",
                    class: "block h-auto w-full dark:hidden",
                }
                img {
                    src: "/examples/authentication-dark.png",
                    alt: "Authentication",
                    width: "1280",
                    height: "843",
                    class: "hidden h-auto w-full dark:block",
                }
            }

            div { class: "relative container hidden flex-1 shrink-0 items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0",
                Link {
                    to: "/examples/authentication",
                    class: "absolute top-4 right-4 inline-flex h-9 items-center justify-center rounded-md px-4 text-sm font-medium transition-all outline-none hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 md:top-8 md:right-8",
                    "Login"
                }

                div { class: "relative hidden h-full flex-col p-10 text-primary lg:flex dark:border-r",
                    div { class: "absolute inset-0 bg-primary/5" }
                    div { class: "relative z-20 flex items-center text-lg font-medium",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "mr-2 h-6 w-6",
                            path { d: "M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" }
                        }
                        "Acme Inc"
                    }
                    div { class: "relative z-20 mt-auto",
                        blockquote { class: "leading-normal text-balance",
                            "\"This library has saved me countless hours of work and helped me deliver stunning designs to my clients faster than ever before.\" - Sofia Davis"
                        }
                    }
                }

                div { class: "flex items-center justify-center lg:h-[1000px] lg:p-8",
                    div { class: "mx-auto flex w-full flex-col justify-center gap-6 sm:w-[350px]",
                        div { class: "flex flex-col gap-2 text-center",
                            h1 { class: "text-2xl font-semibold tracking-tight", "Create an account" }
                            p { class: "text-sm text-muted-foreground", "Enter your email below to create your account" }
                        }

                        div { class: "grid gap-6",
                            div { class: "grid gap-2",
                                Label { for_id: "auth-email", class: "sr-only", "Email" }
                                Input {
                                    id: "auth-email",
                                    r#type: "email",
                                    placeholder: "name@example.com",
                                    autocomplete: "email",
                                    autocorrect: "off",
                                    autocapitalize: "none",
                                }
                                Button { "Sign In with Email" }
                            }
                            div { class: "relative",
                                Separator {}
                                span { class: "bg-background text-muted-foreground absolute inset-x-0 -top-2 mx-auto w-fit px-2 text-xs", "Or continue with" }
                            }
                            Button { variant: ButtonVariant::Outline, "Continue with GitHub" }
                        }

                        p { class: "px-6 text-center text-sm text-muted-foreground",
                            "By clicking continue, you agree to our "
                            Link { to: "/terms", class: "underline underline-offset-4 hover:text-primary", "Terms of Service" }
                            " and "
                            Link { to: "/privacy", class: "underline underline-offset-4 hover:text-primary", "Privacy Policy" }
                            "."
                        }
                    }
                }
            }
        }
    }
}
