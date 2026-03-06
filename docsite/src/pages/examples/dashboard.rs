//! Dashboard example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use dioxus_shadcn::components::{
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    progress::Progress,
};

/// Dashboard example.
#[component]
pub fn DashboardExample() -> Element {
    rsx! {
        ExamplesShell {
            div { class: "md:hidden",
                img {
                    src: "/examples/dashboard-light.png",
                    alt: "Dashboard",
                    class: "block w-full dark:hidden",
                }
                img {
                    src: "/examples/dashboard-dark.png",
                    alt: "Dashboard",
                    class: "hidden w-full dark:block",
                }
            }

            div {
                class: "hidden flex-1 md:flex",
                style: "--sidebar-width: calc(var(--spacing) * 64); --header-height: calc(var(--spacing) * 12 + 1px);",
                aside { class: "hidden w-(--sidebar-width) shrink-0 border-r bg-muted/20 lg:flex",
                    div { class: "w-full p-4",
                        p { class: "mb-2 text-sm font-medium", "Sidebar (not ported)" }
                        p { class: "text-xs text-muted-foreground", "The `sidebar` component is not implemented yet." }
                    }
                }

                div { class: "flex flex-1 flex-col",
                    header { class: "sticky top-0 z-10 flex h-(--header-height) shrink-0 items-center gap-2 border-b bg-background/90 px-4",
                        h1 { class: "text-base font-medium", "Documents" }
                    }

                    div { class: "@container/main flex flex-1 flex-col gap-2",
                        div { class: "flex flex-col gap-4 py-4 md:gap-6 md:py-6",
                            div { class: "grid gap-4 px-4 md:grid-cols-2 xl:grid-cols-4 lg:px-6",
                                MetricCard { title: "Total Revenue", value: "$1,250.00", delta: "+12.5%" }
                                MetricCard { title: "Subscriptions", value: "+2350", delta: "+180.1%" }
                                MetricCard { title: "Sales", value: "+12,234", delta: "+19%" }
                                MetricCard { title: "Active Now", value: "+573", delta: "+201" }
                            }

                            div { class: "px-4 lg:px-6",
                                Card {
                                    CardHeader {
                                        CardTitle { "Interactive Chart" }
                                        CardDescription {
                                            "Chart component is not ported yet. This placeholder keeps the v4 dashboard spacing and structure."
                                        }
                                    }
                                    CardContent { class: "space-y-3",
                                        ChartRow { label: "Mon", value: 38 }
                                        ChartRow { label: "Tue", value: 52 }
                                        ChartRow { label: "Wed", value: 66 }
                                        ChartRow { label: "Thu", value: 58 }
                                        ChartRow { label: "Fri", value: 81 }
                                    }
                                }
                            }

                            div { class: "px-4 lg:px-6",
                                Card {
                                    CardHeader {
                                        CardTitle { "Recent Transactions" }
                                        CardDescription {
                                            "Data table component is not ported yet."
                                        }
                                    }
                                    CardContent {
                                        div { class: "h-64 rounded-md border border-dashed bg-muted/20" }
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

#[component]
fn MetricCard(title: &'static str, value: &'static str, delta: &'static str) -> Element {
    rsx! {
        Card {
            CardHeader { class: "pb-2",
                CardDescription { "{title}" }
                CardTitle { class: "text-2xl", "{value}" }
            }
            CardContent {
                p { class: "text-xs text-muted-foreground", "{delta} vs last month" }
            }
        }
    }
}

#[component]
fn ChartRow(label: &'static str, value: u8) -> Element {
    rsx! {
        div { class: "space-y-1.5",
            div { class: "flex items-center justify-between text-sm",
                span { class: "text-muted-foreground", "{label}" }
                span { class: "font-medium", "{value}%" }
            }
            Progress { value: value }
        }
    }
}
