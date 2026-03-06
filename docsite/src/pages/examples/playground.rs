//! Playground example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use lumen_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    label::Label,
    separator::Separator,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
    textarea::Textarea,
};

/// Playground demo inspired by the v4 reference page.
#[component]
pub fn PlaygroundExample() -> Element {
    rsx! {
        ExamplesShell {
            div { class: "md:hidden",
                img {
                    src: "/examples/playground-light.png",
                    alt: "Playground",
                    class: "block w-full dark:hidden",
                }
                img {
                    src: "/examples/playground-dark.png",
                    alt: "Playground",
                    class: "hidden w-full dark:block",
                }
            }

            div { class: "hidden flex-1 flex-col md:flex",
                div { class: "container flex flex-col items-start justify-between gap-2 py-4 sm:flex-row sm:items-center sm:gap-0 md:h-16",
                    h2 { class: "pl-0.5 text-lg font-semibold", "Playground" }
                    div { class: "ml-auto flex w-full gap-2 sm:justify-end",
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Small, "Preset" }
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Small, "Code" }
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Small, "Share" }
                        Button { size: ButtonSize::Small, "Run" }
                    }
                }

                Separator {}

                Tabs { default_value: "complete", class: "flex flex-1 flex-col",
                    div { class: "container flex flex-1 flex-col py-6",
                        div { class: "grid flex-1 items-stretch gap-6 md:grid-cols-[1fr_200px]",
                            div { class: "hidden flex-col gap-6 sm:flex md:order-2",
                                div { class: "grid gap-3",
                                    Label { class: "text-sm leading-none font-medium", "Mode" }
                                    TabsList { class: "grid w-full grid-cols-3",
                                        TabsTrigger { value: "complete", "Complete" }
                                        TabsTrigger { value: "insert", "Insert" }
                                        TabsTrigger { value: "edit", "Edit" }
                                    }
                                }
                            }

                            div { class: "flex flex-1 flex-col *:data-[slot=tab-content]:flex-1 md:order-1",
                                TabsContent { value: "complete", class: "mt-0 border-0 p-0",
                                    div { class: "flex h-full flex-col gap-4",
                                        Textarea {
                                            placeholder: "Write a tagline for an ice cream shop",
                                            class: "min-h-[400px] flex-1 p-4 md:min-h-[700px] lg:min-h-[700px]",
                                        }
                                        div { class: "flex items-center gap-2",
                                            Button { "Submit" }
                                            Button { variant: ButtonVariant::Secondary, "Reset" }
                                        }
                                    }
                                }

                                TabsContent { value: "insert", class: "mt-0 flex flex-col gap-4 border-0 p-0",
                                    div { class: "grid h-full grid-rows-2 gap-6 lg:grid-cols-2 lg:grid-rows-1",
                                        Textarea {
                                            placeholder: "We're writing to [insert]. Congrats from OpenAI!",
                                            class: "h-full min-h-[300px] p-4 lg:min-h-[700px] xl:min-h-[700px]",
                                        }
                                        div { class: "rounded-md border bg-muted" }
                                    }
                                }

                                TabsContent { value: "edit", class: "mt-0 flex flex-col gap-4 border-0 p-0",
                                    div { class: "grid h-full gap-6 lg:grid-cols-2",
                                        div { class: "flex flex-col gap-4",
                                            div { class: "flex flex-1 flex-col gap-2",
                                                Label { for_id: "playground-input", class: "sr-only", "Input" }
                                                Textarea {
                                                    id: "playground-input",
                                                    placeholder: "We is going to the market.",
                                                    class: "flex-1 p-4 lg:min-h-[580px]",
                                                }
                                            }
                                            div { class: "flex flex-col gap-2",
                                                Label { for_id: "playground-instructions", "Instructions" }
                                                Textarea {
                                                    id: "playground-instructions",
                                                    placeholder: "Fix the grammar.",
                                                }
                                            }
                                        }
                                        div { class: "min-h-[400px] rounded-md border bg-muted lg:min-h-[700px]" }
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
