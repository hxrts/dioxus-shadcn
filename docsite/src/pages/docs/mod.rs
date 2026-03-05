//! Documentation pages module.

pub mod components;
pub mod installation;
pub mod intro;
pub mod navigation;

pub use installation::InstallationDoc;
pub use intro::IntroDoc;

/// Documentation route - parsed from URL path.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DocsRoute {
    #[default]
    IntroPage,
    InstallationPage,
    AccordionPage,
    AlertPage,
    AlertDialogPage,
    AspectRatioPage,
    AvatarPage,
    BadgePage,
    BreadcrumbPage,
    ButtonPage,
    CardPage,
    CheckboxPage,
    CollapsiblePage,
    DialogPage,
    DropdownPage,
    HoverCardPage,
    InputPage,
    InputOTPPage,
    LabelPage,
    PopoverPage,
    ProgressPage,
    RadioGroupPage,
    ScrollAreaPage,
    SelectPage,
    SeparatorPage,
    SideSheetPage,
    SkeletonPage,
    SliderPage,
    SwitchPage,
    TablePage,
    TabsPage,
    TextareaPage,
    ToastPage,
    TogglePage,
    ToggleGroupPage,
    TooltipPage,
}

impl DocsRoute {
    /// Parse a DocsRoute from URL segments.
    pub fn from_segments(segments: &[String]) -> Self {
        let segments: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();
        match segments.as_slice() {
            [] | [""] => Self::IntroPage,
            ["installation"] => Self::InstallationPage,
            ["components", "accordion"] => Self::AccordionPage,
            ["components", "alert"] => Self::AlertPage,
            ["components", "alert-dialog"] => Self::AlertDialogPage,
            ["components", "aspect-ratio"] => Self::AspectRatioPage,
            ["components", "avatar"] => Self::AvatarPage,
            ["components", "badge"] => Self::BadgePage,
            ["components", "breadcrumb"] => Self::BreadcrumbPage,
            ["components", "button"] => Self::ButtonPage,
            ["components", "card"] => Self::CardPage,
            ["components", "checkbox"] => Self::CheckboxPage,
            ["components", "collapsible"] => Self::CollapsiblePage,
            ["components", "dialog"] => Self::DialogPage,
            ["components", "dropdown"] => Self::DropdownPage,
            ["components", "hover-card"] => Self::HoverCardPage,
            ["components", "input"] => Self::InputPage,
            ["components", "input-otp"] => Self::InputOTPPage,
            ["components", "label"] => Self::LabelPage,
            ["components", "popover"] => Self::PopoverPage,
            ["components", "progress"] => Self::ProgressPage,
            ["components", "radio-group"] => Self::RadioGroupPage,
            ["components", "scroll-area"] => Self::ScrollAreaPage,
            ["components", "select"] => Self::SelectPage,
            ["components", "separator"] => Self::SeparatorPage,
            ["components", "side-sheet"] => Self::SideSheetPage,
            ["components", "skeleton"] => Self::SkeletonPage,
            ["components", "slider"] => Self::SliderPage,
            ["components", "switch"] => Self::SwitchPage,
            ["components", "table"] => Self::TablePage,
            ["components", "tabs"] => Self::TabsPage,
            ["components", "textarea"] => Self::TextareaPage,
            ["components", "toast"] => Self::ToastPage,
            ["components", "toggle"] => Self::TogglePage,
            ["components", "toggle-group"] => Self::ToggleGroupPage,
            ["components", "tooltip"] => Self::TooltipPage,
            _ => Self::IntroPage,
        }
    }

    /// Get the URL path for this route.
    pub fn to_path(&self) -> &'static str {
        match self {
            Self::IntroPage => "/docs",
            Self::InstallationPage => "/docs/installation",
            Self::AccordionPage => "/docs/components/accordion",
            Self::AlertPage => "/docs/components/alert",
            Self::AlertDialogPage => "/docs/components/alert-dialog",
            Self::AspectRatioPage => "/docs/components/aspect-ratio",
            Self::AvatarPage => "/docs/components/avatar",
            Self::BadgePage => "/docs/components/badge",
            Self::BreadcrumbPage => "/docs/components/breadcrumb",
            Self::ButtonPage => "/docs/components/button",
            Self::CardPage => "/docs/components/card",
            Self::CheckboxPage => "/docs/components/checkbox",
            Self::CollapsiblePage => "/docs/components/collapsible",
            Self::DialogPage => "/docs/components/dialog",
            Self::DropdownPage => "/docs/components/dropdown",
            Self::HoverCardPage => "/docs/components/hover-card",
            Self::InputPage => "/docs/components/input",
            Self::InputOTPPage => "/docs/components/input-otp",
            Self::LabelPage => "/docs/components/label",
            Self::PopoverPage => "/docs/components/popover",
            Self::ProgressPage => "/docs/components/progress",
            Self::RadioGroupPage => "/docs/components/radio-group",
            Self::ScrollAreaPage => "/docs/components/scroll-area",
            Self::SelectPage => "/docs/components/select",
            Self::SeparatorPage => "/docs/components/separator",
            Self::SideSheetPage => "/docs/components/side-sheet",
            Self::SkeletonPage => "/docs/components/skeleton",
            Self::SliderPage => "/docs/components/slider",
            Self::SwitchPage => "/docs/components/switch",
            Self::TablePage => "/docs/components/table",
            Self::TabsPage => "/docs/components/tabs",
            Self::TextareaPage => "/docs/components/textarea",
            Self::ToastPage => "/docs/components/toast",
            Self::TogglePage => "/docs/components/toggle",
            Self::ToggleGroupPage => "/docs/components/toggle-group",
            Self::TooltipPage => "/docs/components/tooltip",
        }
    }

    /// Get the title for the current route.
    pub fn title(&self) -> &'static str {
        match self {
            Self::IntroPage => "Introduction",
            Self::InstallationPage => "Installation",
            Self::AccordionPage => "Accordion",
            Self::AlertPage => "Alert",
            Self::AlertDialogPage => "Alert Dialog",
            Self::AspectRatioPage => "Aspect Ratio",
            Self::AvatarPage => "Avatar",
            Self::BadgePage => "Badge",
            Self::BreadcrumbPage => "Breadcrumb",
            Self::ButtonPage => "Button",
            Self::CardPage => "Card",
            Self::CheckboxPage => "Checkbox",
            Self::CollapsiblePage => "Collapsible",
            Self::DialogPage => "Dialog",
            Self::DropdownPage => "Dropdown",
            Self::HoverCardPage => "Hover Card",
            Self::InputPage => "Input",
            Self::InputOTPPage => "Input OTP",
            Self::LabelPage => "Label",
            Self::PopoverPage => "Popover",
            Self::ProgressPage => "Progress",
            Self::RadioGroupPage => "Radio Group",
            Self::ScrollAreaPage => "Scroll Area",
            Self::SelectPage => "Select",
            Self::SeparatorPage => "Separator",
            Self::SideSheetPage => "Side Sheet",
            Self::SkeletonPage => "Skeleton",
            Self::SliderPage => "Slider",
            Self::SwitchPage => "Switch",
            Self::TablePage => "Table",
            Self::TabsPage => "Tabs",
            Self::TextareaPage => "Textarea",
            Self::ToastPage => "Toast",
            Self::TogglePage => "Toggle",
            Self::ToggleGroupPage => "Toggle Group",
            Self::TooltipPage => "Tooltip",
        }
    }
}
