//! Tabs component examples with embedded source code.

use dioxus::prelude::*;
use lumen_blocks::components::button::{Button, ButtonVariant};
use lumen_blocks::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use lumen_blocks::components::input::Input;
use lumen_blocks::components::label::Label;
use lumen_blocks::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

// ============================================================================
// Source code strings for documentation
// ============================================================================

pub const BASIC_SOURCE: &str = r#"use lumen_blocks::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

rsx! {
    Tabs { default_value: Some("account".to_string()),
        TabsList {
            TabsTrigger { value: "account", "Account" }
            TabsTrigger { value: "password", "Password" }
        }
        TabsContent { value: "account",
            p { "Make changes to your account here." }
        }
        TabsContent { value: "password",
            p { "Change your password here." }
        }
    }
}"#;

pub const LINE_VARIANT_SOURCE: &str = r#"use lumen_blocks::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

rsx! {
    Tabs {
        default_value: Some("overview".to_string()),
        variant: TabsVariant::Line,

        TabsList {
            TabsTrigger { value: "overview", "Overview" }
            TabsTrigger { value: "analytics", "Analytics" }
            TabsTrigger { value: "reports", "Reports" }
        }
        // Tab contents...
    }
}"#;

pub const CONTROLLED_SOURCE: &str = r#"use lumen_blocks::components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

let active_tab = use_signal(|| "tab1".to_string());

rsx! {
    Tabs {
        value: Some(active_tab),
        on_value_change: Some(Callback::new(move |v| active_tab.set(v))),

        TabsList {
            TabsTrigger { value: "tab1", "Tab 1" }
            TabsTrigger { value: "tab2", "Tab 2" }
        }
        TabsContent { value: "tab1", "Content 1" }
        TabsContent { value: "tab2", "Content 2" }
    }
}"#;

// ============================================================================
// Live example components
// ============================================================================

#[component]
pub fn TabsBasicExample() -> Element {
    rsx! {
        Tabs { default_value: Some("account".to_string()), class: "w-full max-w-md",
            TabsList {
                TabsTrigger { value: "account".to_string(), "Account" }
                TabsTrigger { value: "password".to_string(), "Password" }
            }
            TabsContent { value: "account".to_string(),
                Card {
                    CardHeader {
                        CardTitle { "Account" }
                        CardDescription { "Make changes to your account here. Click save when you're done." }
                    }
                    CardContent { class: "space-y-4",
                        div { class: "space-y-2",
                            Label { for_id: "name", "Name" }
                            Input { id: "name", placeholder: "John Doe" }
                        }
                        div { class: "space-y-2",
                            Label { for_id: "username", "Username" }
                            Input { id: "username", placeholder: "@johndoe" }
                        }
                        Button { variant: ButtonVariant::Primary, "Save changes" }
                    }
                }
            }
            TabsContent { value: "password".to_string(),
                Card {
                    CardHeader {
                        CardTitle { "Password" }
                        CardDescription { "Change your password here. After saving, you'll be logged out." }
                    }
                    CardContent { class: "space-y-4",
                        div { class: "space-y-2",
                            Label { for_id: "current", "Current password" }
                            Input { id: "current", r#type: "password" }
                        }
                        div { class: "space-y-2",
                            Label { for_id: "new", "New password" }
                            Input { id: "new", r#type: "password" }
                        }
                        Button { variant: ButtonVariant::Primary, "Save password" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TabsLineVariantExample() -> Element {
    rsx! {
        Tabs {
            default_value: Some("overview".to_string()),
            variant: TabsVariant::Line,
            class: "w-full max-w-md",

            TabsList {
                TabsTrigger { value: "overview".to_string(), "Overview" }
                TabsTrigger { value: "analytics".to_string(), "Analytics" }
                TabsTrigger { value: "reports".to_string(), "Reports" }
            }
            TabsContent { value: "overview".to_string(),
                p { class: "text-sm text-muted-foreground py-4",
                    "Overview content goes here."
                }
            }
            TabsContent { value: "analytics".to_string(),
                p { class: "text-sm text-muted-foreground py-4",
                    "Analytics content goes here."
                }
            }
            TabsContent { value: "reports".to_string(),
                p { class: "text-sm text-muted-foreground py-4",
                    "Reports content goes here."
                }
            }
        }
    }
}

#[component]
pub fn TabsControlledExample() -> Element {
    let mut active_tab = use_signal(|| "tab1".to_string());

    rsx! {
        div { class: "space-y-4",
            p { class: "text-sm text-muted-foreground",
                "Active tab: " strong { "{active_tab}" }
            }

            Tabs {
                value: Some(active_tab),
                on_value_change: Some(Callback::new(move |v| active_tab.set(v))),
                class: "w-full max-w-md",

                TabsList {
                    TabsTrigger { value: "tab1".to_string(), "Tab 1" }
                    TabsTrigger { value: "tab2".to_string(), "Tab 2" }
                    TabsTrigger { value: "tab3".to_string(), "Tab 3" }
                }
                TabsContent { value: "tab1".to_string(),
                    p { class: "py-4", "Content for Tab 1" }
                }
                TabsContent { value: "tab2".to_string(),
                    p { class: "py-4", "Content for Tab 2" }
                }
                TabsContent { value: "tab3".to_string(),
                    p { class: "py-4", "Content for Tab 3" }
                }
            }
        }
    }
}
