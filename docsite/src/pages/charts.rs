//! Charts showcase page.

use crate::components::{
    Announcement, ChartsNav, PageActions, PageHeader, PageHeaderDescription, PageHeaderHeading,
    PageNav,
};
use dioxus::prelude::*;
use lumen_blocks::components::{
    card::{Card, CardContent, CardHeader, CardTitle},
    progress::Progress,
};

const TITLE: &str = "Beautiful Charts & Graphs";
const DESCRIPTION: &str =
    "A collection of ready-to-use chart components built with Recharts. From basic charts to rich data displays, copy and paste into your apps.";

/// Default charts page.
#[component]
pub fn Charts() -> Element {
    rsx! { ChartTypePage { chart_type: "area".to_string() } }
}

/// Typed chart page.
#[component]
pub fn ChartType(chart_type: String) -> Element {
    rsx! { ChartTypePage { chart_type } }
}

#[component]
fn ChartTypePage(chart_type: String) -> Element {
    let normalized = if matches!(
        chart_type.as_str(),
        "area" | "bar" | "line" | "pie" | "radar" | "radial" | "tooltip"
    ) {
        chart_type
    } else {
        "area".to_string()
    };
    let heading = format!(
        "{} Charts",
        normalized[0..1].to_uppercase().to_string() + &normalized[1..]
    );

    rsx! {
        div { class: "flex flex-1 flex-col",
            PageHeader {
                Announcement {}
                PageHeaderHeading { "{TITLE}" }
                PageHeaderDescription { "{DESCRIPTION}" }
                PageActions {
                    a {
                        href: "#charts",
                        class: "inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90",
                        "Browse Charts"
                    }
                    a {
                        href: "/docs/components/chart",
                        class: "inline-flex h-8 items-center justify-center rounded-md px-3 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                        "Documentation"
                    }
                }
            }

            PageNav { id: "charts",
                ChartsNav {}
            }

            div { class: "container-wrapper flex-1",
                div { class: "container pb-6",
                    section { class: "theme-container",
                        div { class: "grid flex-1 gap-12 lg:gap-24",
                            h2 { class: "sr-only", "{heading}" }
                            div { class: "grid flex-1 scroll-mt-20 items-stretch gap-10 md:grid-cols-2 md:gap-6 lg:grid-cols-3 xl:gap-10",
                                for i in 0..12 {
                                    ChartCardPlaceholder { key: "{i}", index: i + 1 }
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
fn ChartCardPlaceholder(index: usize) -> Element {
    let a = 20 + ((index * 7) % 60) as u8;
    let b = 15 + ((index * 5) % 65) as u8;
    let c = 10 + ((index * 9) % 70) as u8;

    rsx! {
        Card {
            CardHeader {
                CardTitle { class: "text-sm", "Chart {index}" }
            }
            CardContent { class: "space-y-3",
                Progress { value: a }
                Progress { value: b }
                Progress { value: c }
                p { class: "text-xs text-muted-foreground", "Chart component is not ported yet." }
            }
        }
    }
}
