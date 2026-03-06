//! Carousel component documentation page.

use crate::components::docs::{CodeBlock, ComponentPreview, DocHeader};
use crate::examples::carousel::*;
use dioxus::prelude::*;

/// Carousel documentation page.
#[component]
pub fn CarouselDoc() -> Element {
    let usage_source = r##"rsx! {
    Carousel {
        CarouselContent {
            CarouselItem { "Slide 1" }
            CarouselItem { "Slide 2" }
            CarouselItem { "Slide 3" }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
}"##;

    rsx! {
        article { class: "space-y-8",
            DocHeader {
                title: "Carousel",
                description: "A carousel component built with CSS scroll-snap for smooth native scrolling.",
            }

            section { class: "space-y-4",
                h2 { id: "installation", class: "text-2xl font-semibold tracking-tight", "Installation" }
                CodeBlock {
                    source: "use dioxus_shadcn::components::carousel::{{Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext, CarouselDots}};".to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-4",
                h2 { id: "usage", class: "text-2xl font-semibold tracking-tight", "Usage" }
                CodeBlock {
                    source: usage_source.to_string(),
                    language: "rust".to_string(),
                }
            }

            section { class: "space-y-6",
                h2 { id: "examples", class: "text-2xl font-semibold tracking-tight", "Examples" }

                div { class: "space-y-4",
                    h3 { id: "basic", class: "text-xl font-medium", "Basic" }
                    ComponentPreview {
                        source: BASIC_SOURCE.to_string(),
                        CarouselBasicExample {}
                    }
                }
            }

            section { class: "space-y-4",
                h2 { id: "api", class: "text-2xl font-semibold tracking-tight", "API Reference" }

                h3 { class: "text-lg font-medium mt-6", "Carousel" }
                div { class: "overflow-x-auto",
                    table { class: "w-full text-sm",
                        thead {
                            tr { class: "border-b border-border",
                                th { class: "py-3 px-4 text-left font-medium", "Prop" }
                                th { class: "py-3 px-4 text-left font-medium", "Type" }
                                th { class: "py-3 px-4 text-left font-medium", "Default" }
                                th { class: "py-3 px-4 text-left font-medium", "Description" }
                            }
                        }
                        tbody {
                            tr { class: "border-b border-border",
                                td { class: "py-3 px-4 font-mono text-xs", "orientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "CarouselOrientation" }
                                td { class: "py-3 px-4 font-mono text-xs", "Horizontal" }
                                td { class: "py-3 px-4 text-muted-foreground", "Orientation (Horizontal, Vertical)" }
                            }
                            tr {
                                td { class: "py-3 px-4 font-mono text-xs", "opts" }
                                td { class: "py-3 px-4 font-mono text-xs", "Option<CarouselOptions>" }
                                td { class: "py-3 px-4 font-mono text-xs", "None" }
                                td { class: "py-3 px-4 text-muted-foreground", "Carousel behavior options" }
                            }
                        }
                    }
                }
            }
        }
    }
}
