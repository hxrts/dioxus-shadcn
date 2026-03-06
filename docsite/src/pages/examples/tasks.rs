//! Tasks example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use lumen_blocks::components::{
    badge::{Badge, BadgeVariant},
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};

/// Tasks table demo.
#[component]
pub fn TasksExample() -> Element {
    rsx! {
        ExamplesShell {
            div { class: "md:hidden",
                img {
                    src: "/examples/tasks-light.png",
                    alt: "Tasks",
                    class: "block w-full dark:hidden",
                }
                img {
                    src: "/examples/tasks-dark.png",
                    alt: "Tasks",
                    class: "hidden w-full dark:block",
                }
            }

            div { class: "hidden h-full flex-1 flex-col gap-8 p-8 md:flex",
                div { class: "flex items-center justify-between gap-2",
                    div { class: "flex flex-col gap-1",
                        h2 { class: "text-2xl font-semibold tracking-tight", "Welcome back!" }
                        p { class: "text-muted-foreground", "Here's a list of your tasks for this month." }
                    }
                }

                Table {
                    TableHeader {
                        TableRow {
                            TableHead { class: "w-[90px]", "ID" }
                            TableHead { "Title" }
                            TableHead { "Status" }
                            TableHead { "Priority" }
                            TableHead { "Owner" }
                        }
                    }
                    TableBody {
                        TaskRow {
                            id: "TASK-8782",
                            title: "Update marketing homepage copy",
                            status: "In Progress",
                            status_variant: BadgeVariant::Secondary,
                            priority: "High",
                            owner: "Sofia",
                        }
                        TaskRow {
                            id: "TASK-6501",
                            title: "Implement account usage alerts",
                            status: "Todo",
                            status_variant: BadgeVariant::Outline,
                            priority: "Medium",
                            owner: "Noah",
                        }
                        TaskRow {
                            id: "TASK-9900",
                            title: "Finalize billing API migration",
                            status: "Done",
                            status_variant: BadgeVariant::Default,
                            priority: "High",
                            owner: "Isabella",
                        }
                        TaskRow {
                            id: "TASK-1123",
                            title: "Fix inconsistent button spacing",
                            status: "Review",
                            status_variant: BadgeVariant::Ghost,
                            priority: "Low",
                            owner: "Jackson",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TaskRow(
    id: &'static str,
    title: &'static str,
    status: &'static str,
    status_variant: BadgeVariant,
    priority: &'static str,
    owner: &'static str,
) -> Element {
    rsx! {
        TableRow {
            TableCell { class: "font-mono text-xs", "{id}" }
            TableCell { class: "font-medium", "{title}" }
            TableCell {
                Badge { variant: status_variant, "{status}" }
            }
            TableCell { "{priority}" }
            TableCell { class: "text-muted-foreground", "{owner}" }
        }
    }
}
