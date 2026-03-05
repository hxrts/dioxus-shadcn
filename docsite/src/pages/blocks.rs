//! Blocks showcase page - pre-built component compositions.

use dioxus::prelude::*;
use lumen_blocks::components::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    input::Input,
    label::Label,
    separator::Separator,
};
use lucide_dioxus::{Bell, CreditCard, Mail};

/// Blocks showcase page.
#[component]
pub fn Blocks() -> Element {
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
                            "Blocks"
                        }
                        p {
                            class: "max-w-2xl text-lg text-muted-foreground",
                            "Pre-built component compositions ready to copy and paste into your projects."
                        }
                    }
                }
            }

            // Blocks grid
            div { class: "container max-w-screen-2xl px-4 md:px-6 py-12",
                div { class: "grid gap-8 md:grid-cols-2 lg:grid-cols-3",
                    // Login Card Block
                    BlockCard {
                        title: "Login Card",
                        description: "A simple login form with email and password.",
                        LoginCardExample {}
                    }

                    // Profile Card Block
                    BlockCard {
                        title: "Profile Card",
                        description: "User profile card with avatar and actions.",
                        ProfileCardExample {}
                    }

                    // Notification Card Block
                    BlockCard {
                        title: "Notification Settings",
                        description: "Settings card with toggle options.",
                        NotificationCardExample {}
                    }

                    // Payment Card Block
                    BlockCard {
                        title: "Payment Method",
                        description: "Card for managing payment methods.",
                        PaymentCardExample {}
                    }

                    // Team Members Block
                    BlockCard {
                        title: "Team Members",
                        description: "List of team members with avatars.",
                        TeamMembersExample {}
                    }

                    // Stats Card Block
                    BlockCard {
                        title: "Stats Overview",
                        description: "Dashboard stats card with metrics.",
                        StatsCardExample {}
                    }
                }
            }
        }
    }
}

/// Block card wrapper component.
#[component]
fn BlockCard(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            div {
                h3 { class: "font-semibold text-foreground", "{title}" }
                p { class: "text-sm text-muted-foreground", "{description}" }
            }
            div { class: "rounded-lg border border-border bg-card p-4",
                {children}
            }
        }
    }
}

/// Login card example.
#[component]
fn LoginCardExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Login" }
                CardDescription { "Enter your credentials to access your account." }
            }
            CardContent {
                div { class: "grid gap-4",
                    div { class: "grid gap-2",
                        Label { for_id: "email", "Email" }
                        Input { id: "email", r#type: "email", placeholder: "m@example.com" }
                    }
                    div { class: "grid gap-2",
                        Label { for_id: "password", "Password" }
                        Input { id: "password", r#type: "password" }
                    }
                }
            }
            CardFooter {
                Button { variant: ButtonVariant::Default, class: "w-full", "Sign In" }
            }
        }
    }
}

/// Profile card example.
#[component]
fn ProfileCardExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                div { class: "flex items-center gap-4",
                    Avatar { class: "h-12 w-12",
                        AvatarImage { src: "https://github.com/shadcn.png", alt: "User" }
                        AvatarFallback { "CN" }
                    }
                    div {
                        CardTitle { class: "text-lg", "Sofia Davis" }
                        CardDescription { "Product Designer" }
                    }
                }
            }
            CardContent {
                p { class: "text-sm text-muted-foreground",
                    "Building beautiful interfaces and delightful user experiences."
                }
            }
            CardFooter { class: "flex gap-2",
                Button { variant: ButtonVariant::Outline, size: lumen_blocks::components::button::ButtonSize::Small, "Message" }
                Button { variant: ButtonVariant::Default, size: lumen_blocks::components::button::ButtonSize::Small, "Follow" }
            }
        }
    }
}

/// Notification settings card example.
#[component]
fn NotificationCardExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Notifications" }
                CardDescription { "Manage your notification preferences." }
            }
            CardContent {
                div { class: "space-y-4",
                    div { class: "flex items-center justify-between",
                        div { class: "flex items-center gap-3",
                            Mail { class: "h-4 w-4 text-muted-foreground" }
                            span { class: "text-sm", "Email notifications" }
                        }
                        div { class: "text-sm text-muted-foreground", "On" }
                    }
                    Separator {}
                    div { class: "flex items-center justify-between",
                        div { class: "flex items-center gap-3",
                            Bell { class: "h-4 w-4 text-muted-foreground" }
                            span { class: "text-sm", "Push notifications" }
                        }
                        div { class: "text-sm text-muted-foreground", "Off" }
                    }
                }
            }
        }
    }
}

/// Payment card example.
#[component]
fn PaymentCardExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Payment Method" }
                CardDescription { "Add a new payment method to your account." }
            }
            CardContent {
                div { class: "flex items-center gap-4 p-4 rounded-lg border border-border",
                    CreditCard { class: "h-6 w-6 text-muted-foreground" }
                    div { class: "flex-1",
                        p { class: "text-sm font-medium", "Visa ending in 4242" }
                        p { class: "text-xs text-muted-foreground", "Expires 12/24" }
                    }
                    Button { variant: ButtonVariant::Ghost, size: lumen_blocks::components::button::ButtonSize::Small, "Edit" }
                }
            }
            CardFooter {
                Button { variant: ButtonVariant::Outline, class: "w-full", "Add Payment Method" }
            }
        }
    }
}

/// Team members example.
#[component]
fn TeamMembersExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Team Members" }
                CardDescription { "Invite and manage your team." }
            }
            CardContent {
                div { class: "space-y-4",
                    TeamMember { name: "Sofia Davis", email: "sofia@example.com", role: "Owner" }
                    TeamMember { name: "Jackson Lee", email: "jackson@example.com", role: "Admin" }
                    TeamMember { name: "Isabella Nguyen", email: "isabella@example.com", role: "Member" }
                }
            }
            CardFooter {
                Button { variant: ButtonVariant::Outline, class: "w-full", "Invite Member" }
            }
        }
    }
}

/// Individual team member row.
#[component]
fn TeamMember(name: &'static str, email: &'static str, role: &'static str) -> Element {
    let initials: String = name.split_whitespace().filter_map(|w| w.chars().next()).collect();

    rsx! {
        div { class: "flex items-center gap-4",
            Avatar { class: "h-9 w-9",
                AvatarFallback { "{initials}" }
            }
            div { class: "flex-1 min-w-0",
                p { class: "text-sm font-medium truncate", "{name}" }
                p { class: "text-xs text-muted-foreground truncate", "{email}" }
            }
            span { class: "text-xs text-muted-foreground", "{role}" }
        }
    }
}

/// Stats card example.
#[component]
fn StatsCardExample() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Overview" }
                CardDescription { "Your stats for this month." }
            }
            CardContent {
                div { class: "grid grid-cols-2 gap-4",
                    StatItem { label: "Total Revenue", value: "$12,345" }
                    StatItem { label: "Subscriptions", value: "+180" }
                    StatItem { label: "Sales", value: "+573" }
                    StatItem { label: "Active Users", value: "2,350" }
                }
            }
        }
    }
}

/// Individual stat item.
#[component]
fn StatItem(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            p { class: "text-xs text-muted-foreground", "{label}" }
            p { class: "text-xl font-bold", "{value}" }
        }
    }
}
