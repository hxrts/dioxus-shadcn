//! Charts section navigation.

use crate::Route;
use dioxus::prelude::*;
use dioxus_router::use_route;

const CHART_LINKS: [(&str, &str); 7] = [
    ("Area Charts", "/charts/area#charts"),
    ("Bar Charts", "/charts/bar#charts"),
    ("Line Charts", "/charts/line#charts"),
    ("Pie Charts", "/charts/pie#charts"),
    ("Radar Charts", "/charts/radar#charts"),
    ("Radial Charts", "/charts/radial#charts"),
    ("Tooltips", "/charts/tooltip#charts"),
];

/// Horizontal chart-type navigation.
#[component]
pub fn ChartsNav() -> Element {
    let route = use_route::<Route>();

    let current_path = match route {
        Route::Charts { .. } => "/charts/area#charts",
        Route::ChartType { ref chart_type } => {
            if chart_type == "bar" {
                "/charts/bar#charts"
            } else if chart_type == "line" {
                "/charts/line#charts"
            } else if chart_type == "pie" {
                "/charts/pie#charts"
            } else if chart_type == "radar" {
                "/charts/radar#charts"
            } else if chart_type == "radial" {
                "/charts/radial#charts"
            } else if chart_type == "tooltip" {
                "/charts/tooltip#charts"
            } else {
                "/charts/area#charts"
            }
        }
        _ => "",
    };

    rsx! {
        div { class: "relative overflow-hidden",
            div { class: "flex max-w-[600px] items-center overflow-x-auto no-scrollbar lg:max-w-none",
                for (name, href) in CHART_LINKS {
                    Link {
                        to: href,
                        class: "flex h-7 shrink-0 items-center justify-center px-4 text-center text-base font-medium text-muted-foreground transition-colors hover:text-primary data-[active=true]:text-primary",
                        "data-active": if current_path == href { "true" } else { "false" },
                        "{name}"
                    }
                }
            }
        }
    }
}
