//! Carousel example components.

use dioxus::prelude::*;
use lumen_blocks::components::card::Card;
use lumen_blocks::components::carousel::{
    Carousel, CarouselContent, CarouselDots, CarouselItem, CarouselNext, CarouselPrevious,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r##"rsx! {
    Carousel { class: "w-full max-w-xs mx-auto",
        CarouselContent {
            CarouselItem {
                Card { class: "p-6 flex items-center justify-center h-40",
                    "Slide 1"
                }
            }
            CarouselItem {
                Card { class: "p-6 flex items-center justify-center h-40",
                    "Slide 2"
                }
            }
            CarouselItem {
                Card { class: "p-6 flex items-center justify-center h-40",
                    "Slide 3"
                }
            }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
}"##;

/// Basic carousel example.
#[component]
pub fn CarouselBasicExample() -> Element {
    rsx! {
        Carousel { class: "w-full max-w-xs mx-auto",
            CarouselContent {
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 1"
                    }
                }
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 2"
                    }
                }
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 3"
                    }
                }
            }
            CarouselPrevious {}
            CarouselNext {}
        }
    }
}

/// Carousel with dots example.
#[component]
pub fn CarouselDotsExample() -> Element {
    rsx! {
        Carousel { class: "w-full max-w-xs mx-auto",
            CarouselContent {
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 1"
                    }
                }
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 2"
                    }
                }
                CarouselItem {
                    Card { class: "p-6 flex items-center justify-center h-40",
                        "Slide 3"
                    }
                }
            }
            CarouselDots {}
        }
    }
}
