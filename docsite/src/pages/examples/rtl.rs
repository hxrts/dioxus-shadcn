//! Right-to-left example page.

use super::ExamplesShell;
use dioxus::prelude::*;
use dioxus_shadcn::components::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    direction::{Direction, DirectionProvider},
    input::Input,
    label::Label,
};

/// RTL demo showing right-to-left layout and form controls.
#[component]
pub fn RtlExample() -> Element {
    rsx! {
        ExamplesShell {
            DirectionProvider { direction: Direction::Rtl,
                div { class: "grid flex-1 gap-6 p-4 md:grid-cols-2 md:p-8", "data-slot": "rtl-components",
                    Card {
                        CardHeader {
                            CardTitle { "مرحبا بك" }
                            CardDescription { "هذا مثال يدعم اتجاه النص من اليمين إلى اليسار." }
                        }
                        CardContent { class: "space-y-4",
                            p { class: "text-sm leading-6 text-muted-foreground",
                                "يمكنك استخدام نفس المكونات في واجهات RTL دون تغيير البنية الأساسية."
                            }
                            div { class: "flex gap-2",
                                Button { "إجراء أساسي" }
                                Button { variant: ButtonVariant::Outline, "إجراء ثانوي" }
                            }
                        }
                    }

                    Card {
                        CardHeader {
                            CardTitle { "تسجيل سريع" }
                            CardDescription { "بيانات توضيحية لنموذج RTL." }
                        }
                        CardContent { class: "space-y-3",
                            div { class: "grid gap-2",
                                Label { for_id: "rtl-name", "الاسم" }
                                Input { id: "rtl-name", placeholder: "محمد أحمد" }
                            }
                            div { class: "grid gap-2",
                                Label { for_id: "rtl-email", "البريد الإلكتروني" }
                                Input { id: "rtl-email", r#type: "email", placeholder: "name@example.com" }
                            }
                            Button { class: "w-full", "متابعة" }
                        }
                    }
                }
            }
        }
    }
}
