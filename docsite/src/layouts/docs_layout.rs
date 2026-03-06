//! Documentation layout with three-column structure - matches shadcn-ui v4.

use crate::pages::docs::components::*;
use crate::pages::docs::navigation::DOCS_NAV;
use crate::pages::docs::{DocsRoute, InstallationDoc, IntroDoc};
use dioxus::document;
use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

/// Documentation layout with left nav, content, and right nav.
/// Matches shadcn/ui v4 docs layout exactly.
#[component]
pub fn DocsLayout(segments: Vec<String>) -> Element {
    // Parse the DocsRoute from segments
    let docs_route = DocsRoute::from_segments(&segments);

    // Generate page title
    let title = format!("dioxus-shadcn - {}", docs_route.title());

    rsx! {
        document::Title { "{title}" }

        div {
            class: "container-wrapper docs-nav flex flex-1 flex-col px-2",
            "data-slot": "docs",
            style: "--sidebar-width: calc(var(--spacing) * 72);",

            div {
                class: "min-h-min flex-1 items-start px-0 [--top-spacing:0] lg:grid lg:grid-cols-[var(--sidebar-width)_minmax(0,1fr)] lg:[--top-spacing:calc(var(--spacing)*4)] 3xl:fixed:container 3xl:fixed:px-3",

                DocsLeftNav { current_route: docs_route.clone() }

                div { class: "h-full w-full",
                    main { class: "relative py-6 lg:py-8 xl:grid xl:grid-cols-[1fr_280px] xl:gap-10",
                        DocsContent { route: docs_route.clone() }
                        DocsRightNav { route: docs_route }
                    }
                }
            }
        }
    }
}

/// Left navigation sidebar - matches shadcn-ui v4 docs-sidebar exactly.
#[component]
fn DocsLeftNav(current_route: DocsRoute) -> Element {
    rsx! {
        aside {
            class: "sticky top-[calc(var(--header-height)+1px)] z-30 hidden h-[90svh] overscroll-none bg-transparent lg:flex",
            style: "--sidebar-menu-width: calc(var(--spacing) * 56);",

            // Top spacing
            div { class: "h-9" }

            // Top gradient overlay with blur
            div { class: "absolute top-8 z-10 h-8 w-[var(--sidebar-menu-width)] shrink-0 bg-gradient-to-b from-background via-background/80 to-background/50 blur-xs" }

            // Right border gradient
            div { class: "absolute top-12 right-2 bottom-0 hidden h-full w-px bg-gradient-to-b from-transparent via-border to-transparent lg:flex" }

            // Scrollable content
            nav { class: "mx-auto w-[var(--sidebar-menu-width)] overflow-x-hidden overflow-y-auto no-scrollbar px-2",
                // Navigation sections
                div { class: "pt-6",
                    for section in DOCS_NAV {
                        div { class: "pb-4",
                            // Section label
                            p { class: "mb-1 px-2 text-sm font-medium text-muted-foreground",
                                "{section.title}"
                            }

                            // Section items
                            div { class: "grid gap-0.5",
                                for item in section.items {
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

                // Bottom gradient with blur
                div { class: "sticky -bottom-1 z-10 h-16 shrink-0 bg-gradient-to-t from-background via-background/80 to-background/50 blur-xs" }
            }
        }
    }
}

/// Navigation link component - matches shadcn-ui sidebar menu button style exactly.
#[component]
fn NavLink(
    item_route: DocsRoute,
    current_route: DocsRoute,
    title: &'static str,
    badge: Option<&'static str>,
) -> Element {
    let is_active = current_route == item_route;
    let href = item_route.to_path();

    // Matches shadcn SidebarMenuButton styling exactly
    // Base: relative h-[30px] w-fit overflow-visible border border-transparent text-[0.8rem] font-medium
    // After pseudo: after:absolute after:inset-x-0 after:-inset-y-1 after:z-0 after:rounded-md (for hitbox)
    // Active: data-[active=true]:border-accent data-[active=true]:bg-accent
    let base_class = "relative flex h-[30px] w-fit items-center overflow-visible rounded-md border border-transparent px-2 text-[0.8rem] font-medium transition-colors after:absolute after:inset-x-0 after:-inset-y-1 after:z-0 after:rounded-md";
    let state_class = if is_active {
        "border-accent bg-accent text-accent-foreground"
    } else {
        "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
    };

    rsx! {
        a {
            href: "{href}",
            class: "{base_class} {state_class}",
            "data-active": if is_active { "true" } else { "false" },

            // Expanded hitbox span (same as reference)
            span { class: "absolute inset-0 flex w-[var(--sidebar-menu-width)] bg-transparent" }

            "{title}"

            if let Some(badge_text) = badge {
                span {
                    class: "ml-2 flex size-2 rounded-full bg-blue-500",
                    title: "{badge_text}",
                }
            }
        }
    }
}

/// Main content area.
#[component]
fn DocsContent(route: DocsRoute) -> Element {
    // Determine breadcrumb section
    let section = route.section();
    let page_title = route.title();

    rsx! {
        div {
            class: "mx-auto w-full min-w-0",

            // Breadcrumbs
            nav { class: "mb-4 flex items-center space-x-1 text-sm text-muted-foreground",
                a {
                    href: "/docs",
                    class: "hover:text-foreground transition-colors",
                    "Docs"
                }
                ChevronRight { class: "h-4 w-4" }
                span { class: "text-muted-foreground", "{section}" }
                ChevronRight { class: "h-4 w-4" }
                span { class: "font-medium text-foreground", "{page_title}" }
            }

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
                DocsRoute::ButtonGroupPage => rsx! { ButtonGroupDoc {} },
                DocsRoute::CardPage => rsx! { CardDoc {} },
                DocsRoute::CarouselPage => rsx! { CarouselDoc {} },
                DocsRoute::CheckboxPage => rsx! { CheckboxDoc {} },
                DocsRoute::CollapsiblePage => rsx! { CollapsibleDoc {} },
                DocsRoute::ComboboxPage => rsx! { ComboboxDoc {} },
                DocsRoute::CommandPage => rsx! { CommandDoc {} },
                DocsRoute::ContextMenuPage => rsx! { ContextMenuDoc {} },
                DocsRoute::DialogPage => rsx! { DialogDoc {} },
                DocsRoute::DirectionPage => rsx! { DirectionDoc {} },
                DocsRoute::DrawerPage => rsx! { DrawerDoc {} },
                DocsRoute::DropdownPage => rsx! { DropdownDoc {} },
                DocsRoute::EmptyPage => rsx! { EmptyDoc {} },
                DocsRoute::FieldPage => rsx! { FieldDoc {} },
                DocsRoute::FormPage => rsx! { FormDoc {} },
                DocsRoute::HoverCardPage => rsx! { HoverCardDoc {} },
                DocsRoute::InputPage => rsx! { InputDoc {} },
                DocsRoute::InputGroupPage => rsx! { InputGroupDoc {} },
                DocsRoute::InputOTPPage => rsx! { InputOTPDoc {} },
                DocsRoute::ItemPage => rsx! { ItemDoc {} },
                DocsRoute::KbdPage => rsx! { KbdDoc {} },
                DocsRoute::LabelPage => rsx! { LabelDoc {} },
                DocsRoute::MenubarPage => rsx! { MenubarDoc {} },
                DocsRoute::NativeSelectPage => rsx! { NativeSelectDoc {} },
                DocsRoute::NavigationMenuPage => rsx! { NavigationMenuDoc {} },
                DocsRoute::PaginationPage => rsx! { PaginationDoc {} },
                DocsRoute::PopoverPage => rsx! { PopoverDoc {} },
                DocsRoute::ProgressPage => rsx! { ProgressDoc {} },
                DocsRoute::RadioGroupPage => rsx! { RadioGroupDoc {} },
                DocsRoute::ResizablePage => rsx! { ResizableDoc {} },
                DocsRoute::ScrollAreaPage => rsx! { ScrollAreaDoc {} },
                DocsRoute::SelectPage => rsx! { SelectDoc {} },
                DocsRoute::SeparatorPage => rsx! { SeparatorDoc {} },
                DocsRoute::SideSheetPage => rsx! { SideSheetDoc {} },
                DocsRoute::SkeletonPage => rsx! { SkeletonDoc {} },
                DocsRoute::SliderPage => rsx! { SliderDoc {} },
                DocsRoute::SpinnerPage => rsx! { SpinnerDoc {} },
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
            TocItem {
                id: "about",
                title: "About",
                level: 2,
            },
            TocItem {
                id: "features",
                title: "Features",
                level: 2,
            },
            TocItem {
                id: "philosophy",
                title: "Philosophy",
                level: 2,
            },
            TocItem {
                id: "getting-started",
                title: "Getting Started",
                level: 2,
            },
        ],
        DocsRoute::InstallationPage => &[
            TocItem {
                id: "prerequisites",
                title: "Prerequisites",
                level: 2,
            },
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "tailwind",
                title: "Tailwind CSS Setup",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
        ],
        DocsRoute::AccordionPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "multiple",
                title: "Multiple Open",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::AlertPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "without-icon",
                title: "Without Icon",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::AvatarPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "group",
                title: "Avatar Group",
                level: 3,
            },
            TocItem {
                id: "fallback",
                title: "Fallback",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::BadgePage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "with-icons",
                title: "With Icons",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::BreadcrumbPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "ellipsis",
                title: "With Ellipsis",
                level: 3,
            },
            TocItem {
                id: "custom-separator",
                title: "Custom Separator",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ButtonPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "states",
                title: "States",
                level: 3,
            },
            TocItem {
                id: "with-icons",
                title: "With Icons",
                level: 3,
            },
            TocItem {
                id: "icon-buttons",
                title: "Icon Buttons",
                level: 3,
            },
            TocItem {
                id: "full-width",
                title: "Full Width",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::CardPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-form",
                title: "With Form",
                level: 3,
            },
            TocItem {
                id: "with-action",
                title: "With Action",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::CheckboxPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "controlled",
                title: "Controlled",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::CollapsiblePage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "repository",
                title: "Repository",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::DialogPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-form",
                title: "With Form",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::DropdownPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-icons",
                title: "With Icons",
                level: 3,
            },
            TocItem {
                id: "destructive",
                title: "Destructive Items",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::HoverCardPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::InputPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-label",
                title: "With Label",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "states",
                title: "States",
                level: 3,
            },
            TocItem {
                id: "with-icons",
                title: "With Icons",
                level: 3,
            },
            TocItem {
                id: "types",
                title: "Input Types",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::InputOTPPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "pattern",
                title: "Pattern",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::LabelPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "required",
                title: "Required",
                level: 3,
            },
            TocItem {
                id: "with-checkbox",
                title: "With Checkbox",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ProgressPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "with-percentage",
                title: "With Percentage",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SelectPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-groups",
                title: "With Groups",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SeparatorPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "horizontal",
                title: "Horizontal",
                level: 3,
            },
            TocItem {
                id: "vertical",
                title: "Vertical",
                level: 3,
            },
            TocItem {
                id: "in-card",
                title: "In Card",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SwitchPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-label",
                title: "With Label",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::TabsPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "line-variant",
                title: "Line Variant",
                level: 3,
            },
            TocItem {
                id: "controlled",
                title: "Controlled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::TextareaPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "with-label",
                title: "With Label",
                level: 3,
            },
            TocItem {
                id: "rows",
                title: "Custom Rows",
                level: 3,
            },
            TocItem {
                id: "states",
                title: "States",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ToastPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "with-description",
                title: "With Description",
                level: 3,
            },
            TocItem {
                id: "duration",
                title: "Custom Duration",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::TooltipPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sides",
                title: "Sides",
                level: 3,
            },
            TocItem {
                id: "with-provider",
                title: "With Provider",
                level: 3,
            },
            TocItem {
                id: "no-arrow",
                title: "Without Arrow",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::AlertDialogPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "destructive",
                title: "Destructive",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::AspectRatioPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "16:9 Ratio",
                level: 3,
            },
            TocItem {
                id: "square",
                title: "Square",
                level: 3,
            },
            TocItem {
                id: "portrait",
                title: "Portrait",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::PopoverPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "positioning",
                title: "Positioning",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::TablePage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "selected",
                title: "Selected Rows",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SliderPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "range",
                title: "Custom Range",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::TogglePage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "variants",
                title: "Variants",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "with-text",
                title: "With Text",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ToggleGroupPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "multiple",
                title: "Multiple Selection",
                level: 3,
            },
            TocItem {
                id: "outline",
                title: "Outline Variant",
                level: 3,
            },
            TocItem {
                id: "sizes",
                title: "Sizes",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::RadioGroupPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "horizontal",
                title: "Horizontal",
                level: 3,
            },
            TocItem {
                id: "disabled",
                title: "Disabled",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ScrollAreaPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Vertical",
                level: 3,
            },
            TocItem {
                id: "horizontal",
                title: "Horizontal",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SideSheetPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "sides",
                title: "Sides",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SkeletonPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "card",
                title: "Card",
                level: 3,
            },
            TocItem {
                id: "text-block",
                title: "Text Block",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::CommandPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ContextMenuPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::FormPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::MenubarPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::NavigationMenuPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::PaginationPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ButtonGroupPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::CarouselPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ComboboxPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::DirectionPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::DrawerPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::EmptyPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::FieldPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::InputGroupPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ItemPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::KbdPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::NativeSelectPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::ResizablePage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
        DocsRoute::SpinnerPage => &[
            TocItem {
                id: "installation",
                title: "Installation",
                level: 2,
            },
            TocItem {
                id: "usage",
                title: "Usage",
                level: 2,
            },
            TocItem {
                id: "examples",
                title: "Examples",
                level: 2,
            },
            TocItem {
                id: "basic",
                title: "Basic",
                level: 3,
            },
            TocItem {
                id: "api",
                title: "API Reference",
                level: 2,
            },
        ],
    }
}

/// Right navigation with table of contents.
#[component]
fn DocsRightNav(route: DocsRoute) -> Element {
    let toc = get_toc(&route);

    rsx! {
        div {
            class: "hidden text-sm xl:block",

            div {
                class: "sticky top-16 -mt-10 pt-4",

                div { class: "pb-10",
                    div { class: "sticky top-16 -mt-10 h-[calc(100vh-3.5rem)] py-12",
                        div { class: "space-y-2",
                            p { class: "font-medium", "On This Page" }

                            ul { class: "m-0 list-none",
                                for item in toc {
                                    li { class: "mt-0 pt-2",
                                        a {
                                            class: match item.level {
                                                2 => "inline-block no-underline text-muted-foreground transition-colors hover:text-foreground",
                                                3 => "inline-block no-underline text-muted-foreground transition-colors hover:text-foreground pl-4",
                                                _ => "inline-block no-underline text-muted-foreground transition-colors hover:text-foreground pl-8",
                                            },
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
        }
    }
}
