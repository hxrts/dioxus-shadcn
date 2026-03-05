//! Documentation navigation structure.

use super::DocsRoute;

/// A section in the documentation navigation.
#[derive(Clone)]
pub struct NavSection {
    pub title: &'static str,
    pub items: &'static [NavItem],
}

/// A navigation item linking to a documentation page.
#[derive(Clone)]
pub struct NavItem {
    pub title: &'static str,
    pub route: DocsRoute,
    pub badge: Option<&'static str>,
}

/// Complete documentation navigation structure.
pub static DOCS_NAV: &[NavSection] = &[
    NavSection {
        title: "Getting Started",
        items: &[
            NavItem {
                title: "Introduction",
                route: DocsRoute::IntroPage,
                badge: None,
            },
            NavItem {
                title: "Installation",
                route: DocsRoute::InstallationPage,
                badge: None,
            },
        ],
    },
    NavSection {
        title: "Components",
        items: &[
            NavItem {
                title: "Accordion",
                route: DocsRoute::AccordionPage,
                badge: None,
            },
            NavItem {
                title: "Alert",
                route: DocsRoute::AlertPage,
                badge: None,
            },
            NavItem {
                title: "Alert Dialog",
                route: DocsRoute::AlertDialogPage,
                badge: None,
            },
            NavItem {
                title: "Aspect Ratio",
                route: DocsRoute::AspectRatioPage,
                badge: None,
            },
            NavItem {
                title: "Avatar",
                route: DocsRoute::AvatarPage,
                badge: None,
            },
            NavItem {
                title: "Badge",
                route: DocsRoute::BadgePage,
                badge: None,
            },
            NavItem {
                title: "Breadcrumb",
                route: DocsRoute::BreadcrumbPage,
                badge: None,
            },
            NavItem {
                title: "Button",
                route: DocsRoute::ButtonPage,
                badge: None,
            },
            NavItem {
                title: "Button Group",
                route: DocsRoute::ButtonGroupPage,
                badge: None,
            },
            NavItem {
                title: "Card",
                route: DocsRoute::CardPage,
                badge: None,
            },
            NavItem {
                title: "Carousel",
                route: DocsRoute::CarouselPage,
                badge: None,
            },
            NavItem {
                title: "Checkbox",
                route: DocsRoute::CheckboxPage,
                badge: None,
            },
            NavItem {
                title: "Collapsible",
                route: DocsRoute::CollapsiblePage,
                badge: None,
            },
            NavItem {
                title: "Combobox",
                route: DocsRoute::ComboboxPage,
                badge: None,
            },
            NavItem {
                title: "Command",
                route: DocsRoute::CommandPage,
                badge: None,
            },
            NavItem {
                title: "Context Menu",
                route: DocsRoute::ContextMenuPage,
                badge: None,
            },
            NavItem {
                title: "Dialog",
                route: DocsRoute::DialogPage,
                badge: None,
            },
            NavItem {
                title: "Direction",
                route: DocsRoute::DirectionPage,
                badge: None,
            },
            NavItem {
                title: "Drawer",
                route: DocsRoute::DrawerPage,
                badge: None,
            },
            NavItem {
                title: "Dropdown",
                route: DocsRoute::DropdownPage,
                badge: None,
            },
            NavItem {
                title: "Empty",
                route: DocsRoute::EmptyPage,
                badge: None,
            },
            NavItem {
                title: "Field",
                route: DocsRoute::FieldPage,
                badge: None,
            },
            NavItem {
                title: "Form",
                route: DocsRoute::FormPage,
                badge: None,
            },
            NavItem {
                title: "Hover Card",
                route: DocsRoute::HoverCardPage,
                badge: None,
            },
            NavItem {
                title: "Input",
                route: DocsRoute::InputPage,
                badge: None,
            },
            NavItem {
                title: "Input Group",
                route: DocsRoute::InputGroupPage,
                badge: None,
            },
            NavItem {
                title: "Input OTP",
                route: DocsRoute::InputOTPPage,
                badge: None,
            },
            NavItem {
                title: "Item",
                route: DocsRoute::ItemPage,
                badge: None,
            },
            NavItem {
                title: "Kbd",
                route: DocsRoute::KbdPage,
                badge: None,
            },
            NavItem {
                title: "Label",
                route: DocsRoute::LabelPage,
                badge: None,
            },
            NavItem {
                title: "Menubar",
                route: DocsRoute::MenubarPage,
                badge: None,
            },
            NavItem {
                title: "Native Select",
                route: DocsRoute::NativeSelectPage,
                badge: None,
            },
            NavItem {
                title: "Navigation Menu",
                route: DocsRoute::NavigationMenuPage,
                badge: None,
            },
            NavItem {
                title: "Pagination",
                route: DocsRoute::PaginationPage,
                badge: None,
            },
            NavItem {
                title: "Popover",
                route: DocsRoute::PopoverPage,
                badge: None,
            },
            NavItem {
                title: "Progress",
                route: DocsRoute::ProgressPage,
                badge: None,
            },
            NavItem {
                title: "Radio Group",
                route: DocsRoute::RadioGroupPage,
                badge: None,
            },
            NavItem {
                title: "Resizable",
                route: DocsRoute::ResizablePage,
                badge: None,
            },
            NavItem {
                title: "Scroll Area",
                route: DocsRoute::ScrollAreaPage,
                badge: None,
            },
            NavItem {
                title: "Select",
                route: DocsRoute::SelectPage,
                badge: None,
            },
            NavItem {
                title: "Separator",
                route: DocsRoute::SeparatorPage,
                badge: None,
            },
            NavItem {
                title: "Side Sheet",
                route: DocsRoute::SideSheetPage,
                badge: None,
            },
            NavItem {
                title: "Skeleton",
                route: DocsRoute::SkeletonPage,
                badge: None,
            },
            NavItem {
                title: "Slider",
                route: DocsRoute::SliderPage,
                badge: None,
            },
            NavItem {
                title: "Spinner",
                route: DocsRoute::SpinnerPage,
                badge: None,
            },
            NavItem {
                title: "Switch",
                route: DocsRoute::SwitchPage,
                badge: None,
            },
            NavItem {
                title: "Table",
                route: DocsRoute::TablePage,
                badge: None,
            },
            NavItem {
                title: "Tabs",
                route: DocsRoute::TabsPage,
                badge: None,
            },
            NavItem {
                title: "Textarea",
                route: DocsRoute::TextareaPage,
                badge: None,
            },
            NavItem {
                title: "Toast",
                route: DocsRoute::ToastPage,
                badge: None,
            },
            NavItem {
                title: "Toggle",
                route: DocsRoute::TogglePage,
                badge: None,
            },
            NavItem {
                title: "Toggle Group",
                route: DocsRoute::ToggleGroupPage,
                badge: None,
            },
            NavItem {
                title: "Tooltip",
                route: DocsRoute::TooltipPage,
                badge: None,
            },
        ],
    },
];
