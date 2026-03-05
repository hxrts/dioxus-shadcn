//! Pagination component for navigating through pages.
//!
//! A set of components for building pagination controls.

use dioxus::prelude::*;

/// Props for Pagination.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Pagination content.
    pub children: Element,
}

/// Root pagination container.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Pagination {
///         PaginationContent {
///             PaginationItem {
///                 PaginationPrevious { href: "#" }
///             }
///             PaginationItem {
///                 PaginationLink { href: "#", "1" }
///             }
///             PaginationItem {
///                 PaginationLink { href: "#", is_active: true, "2" }
///             }
///             PaginationItem {
///                 PaginationEllipsis {}
///             }
///             PaginationItem {
///                 PaginationNext { href: "#" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("mx-auto flex w-full justify-center {}", custom_class);

    rsx! {
        nav {
            role: "navigation",
            class: classes,
            "data-slot": "pagination",
            aria_label: "pagination",
            {props.children}
        }
    }
}

/// Props for PaginationContent.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Pagination items.
    pub children: Element,
}

/// Container for pagination items.
#[component]
pub fn PaginationContent(props: PaginationContentProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex flex-row items-center gap-1 {}", custom_class);

    rsx! {
        ul {
            class: classes,
            "data-slot": "pagination-content",
            {props.children}
        }
    }
}

/// Props for PaginationItem.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A single pagination item wrapper.
#[component]
pub fn PaginationItem(props: PaginationItemProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        li {
            class: custom_class,
            "data-slot": "pagination-item",
            {props.children}
        }
    }
}

/// Props for PaginationLink.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    /// The href for the link.
    #[props(default)]
    pub href: Option<String>,

    /// Whether this is the active/current page.
    #[props(default)]
    pub is_active: bool,

    /// Size variant.
    #[props(default)]
    pub size: PaginationLinkSize,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Link content (page number).
    pub children: Element,
}

/// Size variants for pagination links.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PaginationLinkSize {
    #[default]
    Default,
    Icon,
}

/// A link to a specific page.
#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let size_class = match props.size {
        PaginationLinkSize::Default => "h-9 w-9",
        PaginationLinkSize::Icon => "h-9 px-4",
    };

    let active_class = if props.is_active {
        "border border-input bg-background"
    } else {
        "hover:bg-accent hover:text-accent-foreground"
    };

    let classes = format!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm \
         font-medium ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 \
         {} {} {}",
        size_class, active_class, custom_class
    );

    let href_for_click = props.href.clone();
    let handle_click = {
        let on_click = props.on_click.clone();
        move |event: MouseEvent| {
            if href_for_click.is_none() {
                event.prevent_default();
            }
            if let Some(callback) = &on_click {
                callback.call(());
            }
        }
    };

    rsx! {
        a {
            href: props.href.clone().unwrap_or_else(|| "#".to_string()),
            class: classes,
            "data-slot": "pagination-link",
            "data-active": props.is_active.to_string(),
            aria_current: if props.is_active { "page" } else { "" },
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for PaginationPrevious.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationPreviousProps {
    /// The href for the link.
    #[props(default)]
    pub href: Option<String>,

    /// Whether to show the label text.
    #[props(default = true)]
    pub show_label: bool,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Link to the previous page.
#[component]
pub fn PaginationPrevious(props: PaginationPreviousProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex items-center justify-center gap-1 whitespace-nowrap rounded-md text-sm \
         font-medium ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 \
         hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 \
         {}",
        custom_class
    );

    let href_for_click = props.href.clone();
    let handle_click = {
        let on_click = props.on_click.clone();
        move |event: MouseEvent| {
            if href_for_click.is_none() {
                event.prevent_default();
            }
            if let Some(callback) = &on_click {
                callback.call(());
            }
        }
    };

    rsx! {
        a {
            href: props.href.clone().unwrap_or_else(|| "#".to_string()),
            class: classes,
            "data-slot": "pagination-previous",
            aria_label: "Go to previous page",
            onclick: handle_click,

            // Chevron left icon
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m15 18-6-6 6-6" }
            }

            if props.show_label {
                span { "Previous" }
            }
        }
    }
}

/// Props for PaginationNext.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationNextProps {
    /// The href for the link.
    #[props(default)]
    pub href: Option<String>,

    /// Whether to show the label text.
    #[props(default = true)]
    pub show_label: bool,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Link to the next page.
#[component]
pub fn PaginationNext(props: PaginationNextProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "inline-flex items-center justify-center gap-1 whitespace-nowrap rounded-md text-sm \
         font-medium ring-offset-background transition-colors focus-visible:outline-none \
         focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 \
         disabled:pointer-events-none disabled:opacity-50 \
         hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 \
         {}",
        custom_class
    );

    let href_for_click = props.href.clone();
    let handle_click = {
        let on_click = props.on_click.clone();
        move |event: MouseEvent| {
            if href_for_click.is_none() {
                event.prevent_default();
            }
            if let Some(callback) = &on_click {
                callback.call(());
            }
        }
    };

    rsx! {
        a {
            href: props.href.clone().unwrap_or_else(|| "#".to_string()),
            class: classes,
            "data-slot": "pagination-next",
            aria_label: "Go to next page",
            onclick: handle_click,

            if props.show_label {
                span { "Next" }
            }

            // Chevron right icon
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

/// Props for PaginationEllipsis.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationEllipsisProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Ellipsis indicating skipped pages.
#[component]
pub fn PaginationEllipsis(props: PaginationEllipsisProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex h-9 w-9 items-center justify-center {}",
        custom_class
    );

    rsx! {
        span {
            class: classes,
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",

            // More horizontal icon
            svg {
                class: "size-4",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "1" }
                circle { cx: "19", cy: "12", r: "1" }
                circle { cx: "5", cy: "12", r: "1" }
            }

            span { class: "sr-only", "More pages" }
        }
    }
}
