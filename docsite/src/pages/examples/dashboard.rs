//! Dashboard example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use lumen_blocks::components::{
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

            div { class: "hidden flex-1 md:flex",
                div { class: "flex flex-1 flex-col",
                    div { class: "@container/main flex flex-1 flex-col gap-2",
                        div { class: "flex flex-col gap-4 py-4 md:gap-6 md:py-6",
                            div { class: "grid gap-4 px-4 md:grid-cols-2 xl:grid-cols-4 lg:px-6",
                                MetricCard { title: "Revenue", value: "$12,480", delta: "+12.4%" }
                                MetricCard { title: "Subscriptions", value: "1,240", delta: "+6.2%" }
                                MetricCard { title: "Sales", value: "573", delta: "+18.7%" }
                                MetricCard { title: "Active Users", value: "2,356", delta: "+3.1%" }
                            }

                            div { class: "px-4 lg:px-6",
                                Card {
                                    CardHeader {
                                        CardTitle { "Chart Area" }
                                        CardDescription {
                                            "Chart component is not ported yet. This section keeps the v4 dashboard spacing and structure."
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
