//! Documentation layout with three-column structure.

use crate::pages::docs::components::*;
use crate::pages::docs::navigation::DOCS_NAV;
use crate::pages::docs::{DocsRoute, InstallationDoc, IntroDoc};
use dioxus::document;
use dioxus::prelude::*;

/// Signal to control mobile sidebar visibility.
pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);

/// Documentation layout with left nav, content, and right nav.
#[component]
pub fn DocsLayout(segments: Vec<String>) -> Element {
    // Parse the DocsRoute from segments
    let docs_route = DocsRoute::from_segments(&segments);

    // Generate page title
    let title = format!("dioxus-shadcn - {}", docs_route.title());

    rsx! {
        document::Title { "{title}" }

        div { class: "w-full text-sm border-b border-border relative bg-background",
            div { class: "flex flex-row justify-center text-foreground font-light lg:gap-12",
                DocsLeftNav { current_route: docs_route.clone() }
                DocsContent { route: docs_route.clone() }
                DocsRightNav { route: docs_route }
            }
        }
    }
}

/// Left navigation sidebar.
#[component]
fn DocsLeftNav(current_route: DocsRoute) -> Element {
    let is_sidebar_visible = *SHOW_SIDEBAR.read();

    rsx! {
        div {
            class: "min-w-[240px] pt-12 pb-16 border-r border-border sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto backdrop-blur-sm",
            class: if is_sidebar_visible { "block" } else { "hidden md:block" },

            div { class: "pr-8 pl-4",
                nav { class: "space-y-6",
                    for section in DOCS_NAV {
                        div { class: "space-y-2",
                            // Section title
                            h4 { class: "text-sm font-semibold text-foreground mb-2",
                                "{section.title}"
                            }

                            // Section items
                            ul { class: "space-y-1",
                                for item in section.items {
                                    li {
                                        NavLink {
                                            item_route: item.route.clone(),
                                            current_route: current_route.clone(),
                                            title: item.title,
                                            badge: item.badge,
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
}

/// Navigation link component.
#[component]
fn NavLink(
    item_route: DocsRoute,
    current_route: DocsRoute,
    title: &'static str,
    badge: Option<&'static str>,
) -> Element {
    let is_active = current_route == item_route;
    let href = item_route.to_path();

    let class = if is_active {
        "flex items-center justify-between py-1.5 px-2 text-sm rounded-md bg-muted text-foreground font-medium"
    } else {
        "flex items-center justify-between py-1.5 px-2 text-sm rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/50 transition-colors"
    };

    rsx! {
        a {
            href: "{href}",
            class: class,
            onclick: move |_| {
                *SHOW_SIDEBAR.write() = false;
            },

            span { "{title}" }

            if let Some(badge_text) = badge {
                span {
                    class: "text-xs px-1.5 py-0.5 rounded bg-primary/10 text-primary",
                    "{badge_text}"
                }
            }
        }
    }
}

/// Main content area.
#[component]
fn DocsContent(route: DocsRoute) -> Element {
    rsx! {
        div {
            class: "flex-1 max-w-[80ch] w-full pt-12 pb-16 px-6 text-foreground bg-background",

            // Render the appropriate page based on route
            match route {
                DocsRoute::IntroPage => rsx! { IntroDoc {} },
                DocsRoute::InstallationPage => rsx! { InstallationDoc {} },
                DocsRoute::AccordionPage => rsx! { AccordionDoc {} },
                DocsRoute::AlertPage => rsx! { AlertDoc {} },
                DocsRoute::AlertDialogPage => rsx! { AlertDialogDoc {} },
                DocsRoute::AspectRatioPage => rsx! { AspectRatioDoc {} },
                DocsRoute::AvatarPage => rsx! { AvatarDoc {} },
                DocsRoute::BadgePage => rsx! { BadgeDoc {} },
                DocsRoute::BreadcrumbPage => rsx! { BreadcrumbDoc {} },
                DocsRoute::ButtonPage => rsx! { ButtonDoc {} },
                DocsRoute::CardPage => rsx! { CardDoc {} },
                DocsRoute::CheckboxPage => rsx! { CheckboxDoc {} },
                DocsRoute::CollapsiblePage => rsx! { CollapsibleDoc {} },
                DocsRoute::DialogPage => rsx! { DialogDoc {} },
                DocsRoute::DropdownPage => rsx! { DropdownDoc {} },
                DocsRoute::HoverCardPage => rsx! { HoverCardDoc {} },
                DocsRoute::InputPage => rsx! { InputDoc {} },
                DocsRoute::InputOTPPage => rsx! { InputOTPDoc {} },
                DocsRoute::LabelPage => rsx! { LabelDoc {} },
                DocsRoute::PopoverPage => rsx! { PopoverDoc {} },
                DocsRoute::ProgressPage => rsx! { ProgressDoc {} },
                DocsRoute::RadioGroupPage => rsx! { RadioGroupDoc {} },
                DocsRoute::ScrollAreaPage => rsx! { ScrollAreaDoc {} },
                DocsRoute::SelectPage => rsx! { SelectDoc {} },
                DocsRoute::SeparatorPage => rsx! { SeparatorDoc {} },
                DocsRoute::SideSheetPage => rsx! { SideSheetDoc {} },
                DocsRoute::SkeletonPage => rsx! { SkeletonDoc {} },
                DocsRoute::SliderPage => rsx! { SliderDoc {} },
                DocsRoute::SwitchPage => rsx! { SwitchDoc {} },
                DocsRoute::TablePage => rsx! { TableDoc {} },
                DocsRoute::TabsPage => rsx! { TabsDoc {} },
                DocsRoute::TextareaPage => rsx! { TextareaDoc {} },
                DocsRoute::ToastPage => rsx! { ToastDoc {} },
                DocsRoute::TogglePage => rsx! { ToggleDoc {} },
                DocsRoute::ToggleGroupPage => rsx! { ToggleGroupDoc {} },
                DocsRoute::TooltipPage => rsx! { TooltipDoc {} },
            }
        }
    }
}

/// Static table of contents for documentation pages.
struct TocItem {
    id: &'static str,
    title: &'static str,
    level: u8,
}

/// Get table of contents for a route.
fn get_toc(route: &DocsRoute) -> &'static [TocItem] {
    match route {
        DocsRoute::IntroPage => &[
            TocItem { id: "about", title: "About", level: 2 },
            TocItem { id: "features", title: "Features", level: 2 },
            TocItem { id: "philosophy", title: "Philosophy", level: 2 },
            TocItem { id: "getting-started", title: "Getting Started", level: 2 },
        ],
        DocsRoute::InstallationPage => &[
            TocItem { id: "prerequisites", title: "Prerequisites", level: 2 },
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "tailwind", title: "Tailwind CSS Setup", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
        ],
        DocsRoute::AccordionPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "multiple", title: "Multiple Open", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::AlertPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "without-icon", title: "Without Icon", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::AvatarPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "group", title: "Avatar Group", level: 3 },
            TocItem { id: "fallback", title: "Fallback", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::BadgePage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "with-icons", title: "With Icons", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::BreadcrumbPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "ellipsis", title: "With Ellipsis", level: 3 },
            TocItem { id: "custom-separator", title: "Custom Separator", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::ButtonPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "states", title: "States", level: 3 },
            TocItem { id: "with-icons", title: "With Icons", level: 3 },
            TocItem { id: "icon-buttons", title: "Icon Buttons", level: 3 },
            TocItem { id: "full-width", title: "Full Width", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::CardPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-form", title: "With Form", level: 3 },
            TocItem { id: "with-action", title: "With Action", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::CheckboxPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "controlled", title: "Controlled", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::CollapsiblePage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "repository", title: "Repository", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::DialogPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-form", title: "With Form", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::DropdownPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-icons", title: "With Icons", level: 3 },
            TocItem { id: "destructive", title: "Destructive Items", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::HoverCardPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::InputPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-label", title: "With Label", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "states", title: "States", level: 3 },
            TocItem { id: "with-icons", title: "With Icons", level: 3 },
            TocItem { id: "types", title: "Input Types", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::InputOTPPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "pattern", title: "Pattern", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::LabelPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "required", title: "Required", level: 3 },
            TocItem { id: "with-checkbox", title: "With Checkbox", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::ProgressPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "with-percentage", title: "With Percentage", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SelectPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-groups", title: "With Groups", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SeparatorPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "horizontal", title: "Horizontal", level: 3 },
            TocItem { id: "vertical", title: "Vertical", level: 3 },
            TocItem { id: "in-card", title: "In Card", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SwitchPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-label", title: "With Label", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::TabsPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "line-variant", title: "Line Variant", level: 3 },
            TocItem { id: "controlled", title: "Controlled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::TextareaPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "with-label", title: "With Label", level: 3 },
            TocItem { id: "rows", title: "Custom Rows", level: 3 },
            TocItem { id: "states", title: "States", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::ToastPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "with-description", title: "With Description", level: 3 },
            TocItem { id: "duration", title: "Custom Duration", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::TooltipPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sides", title: "Sides", level: 3 },
            TocItem { id: "with-provider", title: "With Provider", level: 3 },
            TocItem { id: "no-arrow", title: "Without Arrow", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::AlertDialogPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "destructive", title: "Destructive", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::AspectRatioPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "16:9 Ratio", level: 3 },
            TocItem { id: "square", title: "Square", level: 3 },
            TocItem { id: "portrait", title: "Portrait", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::PopoverPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "positioning", title: "Positioning", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::TablePage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "selected", title: "Selected Rows", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SliderPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "range", title: "Custom Range", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::TogglePage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "variants", title: "Variants", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "with-text", title: "With Text", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::ToggleGroupPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "multiple", title: "Multiple Selection", level: 3 },
            TocItem { id: "outline", title: "Outline Variant", level: 3 },
            TocItem { id: "sizes", title: "Sizes", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::RadioGroupPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "horizontal", title: "Horizontal", level: 3 },
            TocItem { id: "disabled", title: "Disabled", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::ScrollAreaPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Vertical", level: 3 },
            TocItem { id: "horizontal", title: "Horizontal", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SideSheetPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "sides", title: "Sides", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
        DocsRoute::SkeletonPage => &[
            TocItem { id: "installation", title: "Installation", level: 2 },
            TocItem { id: "usage", title: "Usage", level: 2 },
            TocItem { id: "examples", title: "Examples", level: 2 },
            TocItem { id: "basic", title: "Basic", level: 3 },
            TocItem { id: "card", title: "Card", level: 3 },
            TocItem { id: "text-block", title: "Text Block", level: 3 },
            TocItem { id: "api", title: "API Reference", level: 2 },
        ],
    }
}

/// Right navigation with table of contents.
#[component]
fn DocsRightNav(route: DocsRoute) -> Element {
    let toc = get_toc(&route);

    rsx! {
        div {
            class: "hidden xl:block min-w-[240px] pt-12 pb-16 border-l border-border sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto backdrop-blur-sm",

            div { class: "pl-8",
                h3 { class: "font-bold mb-4 text-foreground text-sm", "On This Page" }

                ul { class: "space-y-2 text-sm",
                    for item in toc {
                        li {
                            class: match item.level {
                                2 => "",
                                3 => "pl-3",
                                _ => "pl-6",
                            },

                            a {
                                class: "block py-1 text-muted-foreground hover:text-foreground transition-colors",
                                href: "#{item.id}",
                                "{item.title}"
                            }
                        }
                    }
                }
            }
        }
    }
}
