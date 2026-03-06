//! Accordion component examples with embedded source code.

use dioxus::prelude::*;
use dioxus_shadcn::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use dioxus_shadcn::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

rsx! {
    Accordion {
        AccordionItem { index: 0,
            AccordionTrigger { "Is it accessible?" }
            AccordionContent {
                "Yes. It adheres to the WAI-ARIA design pattern."
            }
        }
        AccordionItem { index: 1,
            AccordionTrigger { "Is it styled?" }
            AccordionContent {
                "Yes. It comes with default styles that match the other components' aesthetic."
            }
        }
        AccordionItem { index: 2,
            AccordionTrigger { "Is it animated?" }
            AccordionContent {
                "Yes. It's animated by default, but you can disable it if you prefer."
            }
        }
    }
}"#;

pub const MULTIPLE_SOURCE: &str = r#"use dioxus_shadcn::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

rsx! {
    Accordion { allow_multiple_open: true,
        AccordionItem { index: 0,
            AccordionTrigger { "First Section" }
            AccordionContent { "Content for the first section." }
        }
        AccordionItem { index: 1,
            AccordionTrigger { "Second Section" }
            AccordionContent { "Content for the second section." }
        }
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn AccordionBasicExample() -> Element {
    rsx! {
        Accordion { class: "w-full max-w-md",
            AccordionItem { index: 0,
                AccordionTrigger { "Is it accessible?" }
                AccordionContent {
                    "Yes. It adheres to the WAI-ARIA design pattern."
                }
            }
            AccordionItem { index: 1,
                AccordionTrigger { "Is it styled?" }
                AccordionContent {
                    "Yes. It comes with default styles that match the other components' aesthetic."
                }
            }
            AccordionItem { index: 2,
                AccordionTrigger { "Is it animated?" }
                AccordionContent {
                    "Yes. It's animated by default, but you can disable it if you prefer."
                }
            }
        }
    }
}

#[component]
pub fn AccordionMultipleExample() -> Element {
    rsx! {
        Accordion { class: "w-full max-w-md", allow_multiple_open: true,
            AccordionItem { index: 0,
                AccordionTrigger { "First Section" }
                AccordionContent { "Content for the first section. Multiple items can be open at the same time." }
            }
            AccordionItem { index: 1,
                AccordionTrigger { "Second Section" }
                AccordionContent { "Content for the second section. Try opening both!" }
            }
            AccordionItem { index: 2,
                AccordionTrigger { "Third Section" }
                AccordionContent { "Content for the third section." }
            }
        }
    }
}
