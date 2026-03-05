//! Authentication example page - login and signup forms.

use dioxus::prelude::*;
use lumen_blocks::components::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    input::Input,
    label::Label,
    separator::Separator,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
};
use lucide_dioxus::Github;

/// Authentication example page.
#[component]
pub fn AuthenticationExample() -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col",
            // Page header
            div {
                class: "border-b border-border/40",
                div {
                    class: "container max-w-screen-2xl",
                    div {
                        class: "flex flex-col items-center gap-4 py-12 md:py-16 text-center px-4",
                        h1 {
                            class: "text-3xl font-bold leading-tight tracking-tighter md:text-4xl",
                            "Authentication"
                        }
                        p {
                            class: "max-w-2xl text-lg text-muted-foreground",
                            "Example authentication forms built with dioxus-shadcn components."
                        }
                    }
                }
            }

            // Auth examples
            div { class: "container max-w-screen-2xl px-4 md:px-6 py-12",
                div { class: "grid gap-12",
                    // Centered auth card
                    section {
                        h2 { class: "text-xl font-semibold mb-4", "Centered Login" }
                        p { class: "text-muted-foreground mb-6", "A centered login card with social login options." }

                        div { class: "flex justify-center",
                            div { class: "w-full max-w-md",
                                CenteredAuthCard {}
                            }
                        }
                    }

                    // Split auth layout
                    section {
                        h2 { class: "text-xl font-semibold mb-4", "Split Layout" }
                        p { class: "text-muted-foreground mb-6", "Login with a side panel for branding." }

                        div { class: "rounded-lg border border-border overflow-hidden",
                            SplitAuthLayout {}
                        }
                    }

                    // Tabbed auth
                    section {
                        h2 { class: "text-xl font-semibold mb-4", "Tabbed Login/Signup" }
                        p { class: "text-muted-foreground mb-6", "Combined login and signup in tabs." }

                        div { class: "flex justify-center",
                            div { class: "w-full max-w-md",
                                TabbedAuth {}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Centered authentication card with social login.
#[component]
fn CenteredAuthCard() -> Element {
    rsx! {
        Card {
            CardHeader { class: "space-y-1",
                CardTitle { class: "text-2xl text-center", "Sign in" }
                CardDescription { class: "text-center",
                    "Enter your email and password to sign in"
                }
            }
            CardContent { class: "grid gap-4",
                // Social login buttons
                div { class: "grid grid-cols-2 gap-4",
                    Button { variant: ButtonVariant::Outline,
                        Github { class: "mr-2 h-4 w-4" }
                        "GitHub"
                    }
                    Button { variant: ButtonVariant::Outline,
                        svg {
                            class: "mr-2 h-4 w-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                            }
                            path {
                                d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                            }
                            path {
                                d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                            }
                            path {
                                d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                            }
                        }
                        "Google"
                    }
                }

                div { class: "relative",
                    div { class: "absolute inset-0 flex items-center",
                        Separator {}
                    }
                    div { class: "relative flex justify-center text-xs uppercase",
                        span { class: "bg-card px-2 text-muted-foreground", "Or continue with" }
                    }
                }

                // Email/password form
                div { class: "grid gap-4",
                    div { class: "grid gap-2",
                        Label { for_id: "email", "Email" }
                        Input {
                            id: "email",
                            r#type: "email",
                            placeholder: "m@example.com"
                        }
                    }
                    div { class: "grid gap-2",
                        div { class: "flex items-center justify-between",
                            Label { for_id: "password", "Password" }
                            a {
                                href: "#",
                                class: "text-sm text-primary underline-offset-4 hover:underline",
                                "Forgot password?"
                            }
                        }
                        Input {
                            id: "password",
                            r#type: "password"
                        }
                    }
                }
            }
            CardFooter { class: "flex flex-col gap-4",
                Button {
                    variant: ButtonVariant::Default,
                    class: "w-full",
                    "Sign In"
                }
                p { class: "text-center text-sm text-muted-foreground",
                    "Don't have an account? "
                    a {
                        href: "#",
                        class: "text-primary underline-offset-4 hover:underline",
                        "Sign up"
                    }
                }
            }
        }
    }
}

/// Split layout authentication.
#[component]
fn SplitAuthLayout() -> Element {
    rsx! {
        div { class: "grid lg:grid-cols-2 min-h-[500px]",
            // Left side - branding
            div { class: "hidden lg:flex flex-col justify-between bg-primary p-10 text-primary-foreground",
                div { class: "flex items-center gap-2 text-lg font-medium",
                    div { class: "h-8 w-8 rounded-full bg-primary-foreground/20 flex items-center justify-center",
                        span { class: "text-sm font-bold", "DS" }
                    }
                    "dioxus-shadcn"
                }
                div {
                    blockquote { class: "space-y-2",
                        p { class: "text-lg",
                            "\"This library has saved me countless hours of work and helped me deliver stunning designs to my clients faster than ever before.\""
                        }
                        footer { class: "text-sm opacity-80", "Sofia Davis" }
                    }
                }
            }

            // Right side - form
            div { class: "flex items-center justify-center p-8",
                div { class: "mx-auto w-full max-w-sm space-y-6",
                    div { class: "space-y-2 text-center",
                        h1 { class: "text-2xl font-semibold tracking-tight", "Create an account" }
                        p { class: "text-sm text-muted-foreground",
                            "Enter your email below to create your account"
                        }
                    }

                    div { class: "grid gap-4",
                        div { class: "grid gap-2",
                            Label { for_id: "split-email", "Email" }
                            Input {
                                id: "split-email",
                                r#type: "email",
                                placeholder: "name@example.com"
                            }
                        }
                        div { class: "grid gap-2",
                            Label { for_id: "split-password", "Password" }
                            Input {
                                id: "split-password",
                                r#type: "password"
                            }
                        }
                        Button { variant: ButtonVariant::Default, class: "w-full",
                            "Create Account"
                        }
                    }

                    p { class: "text-center text-sm text-muted-foreground",
                        "By clicking continue, you agree to our "
                        a { href: "#", class: "underline underline-offset-4 hover:text-primary", "Terms of Service" }
                        " and "
                        a { href: "#", class: "underline underline-offset-4 hover:text-primary", "Privacy Policy" }
                        "."
                    }
                }
            }
        }
    }
}

/// Tabbed authentication (login/signup).
#[component]
fn TabbedAuth() -> Element {
    rsx! {
        Tabs { default_value: "login",
            TabsList { class: "grid w-full grid-cols-2",
                TabsTrigger { value: "login", "Login" }
                TabsTrigger { value: "signup", "Sign Up" }
            }

            TabsContent { value: "login",
                Card {
                    CardHeader {
                        CardTitle { "Login" }
                        CardDescription { "Enter your credentials to access your account." }
                    }
                    CardContent { class: "space-y-4",
                        div { class: "space-y-2",
                            Label { for_id: "tab-email", "Email" }
                            Input { id: "tab-email", r#type: "email", placeholder: "m@example.com" }
                        }
                        div { class: "space-y-2",
                            Label { for_id: "tab-password", "Password" }
                            Input { id: "tab-password", r#type: "password" }
                        }
                    }
                    CardFooter {
                        Button { variant: ButtonVariant::Default, class: "w-full", "Login" }
                    }
                }
            }

            TabsContent { value: "signup",
                Card {
                    CardHeader {
                        CardTitle { "Create Account" }
                        CardDescription { "Enter your details to create a new account." }
                    }
                    CardContent { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "space-y-2",
                                Label { for_id: "first-name", "First Name" }
                                Input { id: "first-name", placeholder: "John" }
                            }
                            div { class: "space-y-2",
                                Label { for_id: "last-name", "Last Name" }
                                Input { id: "last-name", placeholder: "Doe" }
                            }
                        }
                        div { class: "space-y-2",
                            Label { for_id: "signup-email", "Email" }
                            Input { id: "signup-email", r#type: "email", placeholder: "m@example.com" }
                        }
                        div { class: "space-y-2",
                            Label { for_id: "signup-password", "Password" }
                            Input { id: "signup-password", r#type: "password" }
                        }
                    }
                    CardFooter {
                        Button { variant: ButtonVariant::Default, class: "w-full", "Create Account" }
                    }
                }
            }
        }
    }
}
