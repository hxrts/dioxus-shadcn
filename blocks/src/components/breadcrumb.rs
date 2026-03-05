//! Breadcrumb component for navigation hierarchy.
//!
//! A set of components for building breadcrumb navigation.

use dioxus::prelude::*;

/// Props for Breadcrumb.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Breadcrumb content.
    pub children: Element,
}

/// Root breadcrumb navigation container.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Breadcrumb {
///         BreadcrumbList {
///             BreadcrumbItem {
///                 BreadcrumbLink { href: "/", "Home" }
///             }
///             BreadcrumbSeparator {}
///             BreadcrumbItem {
///                 BreadcrumbLink { href: "/docs", "Documentation" }
///             }
///             BreadcrumbSeparator {}
///             BreadcrumbItem {
///                 BreadcrumbPage { "Current Page" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        nav {
            class: custom_class,
            "data-slot": "breadcrumb",
            aria_label: "breadcrumb",
            {props.children}
        }
    }
}

/// Props for BreadcrumbList.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Breadcrumb items.
    pub children: Element,
}

/// Ordered list container for breadcrumb items.
#[component]
pub fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5 {}",
        custom_class
    );

    rsx! {
        ol {
            class: classes,
            "data-slot": "breadcrumb-list",
            {props.children}
        }
    }
}

/// Props for BreadcrumbItem.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A single breadcrumb item wrapper.
#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("inline-flex items-center gap-1.5 {}", custom_class);

    rsx! {
        li {
            class: classes,
            "data-slot": "breadcrumb-item",
            {props.children}
        }
    }
}

/// Props for BreadcrumbLink.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbLinkProps {
    /// The href for the link.
    #[props(default)]
    pub href: Option<String>,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Link content.
    pub children: Element,
}

/// A clickable link in the breadcrumb.
#[component]
pub fn BreadcrumbLink(props: BreadcrumbLinkProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "transition-colors hover:text-foreground {}",
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
            "data-slot": "breadcrumb-link",
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for BreadcrumbPage.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbPageProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Page content.
    pub children: Element,
}

/// The current page (non-clickable) in the breadcrumb.
#[component]
pub fn BreadcrumbPage(props: BreadcrumbPageProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("font-normal text-foreground {}", custom_class);

    rsx! {
        span {
            class: classes,
            "data-slot": "breadcrumb-page",
            role: "link",
            aria_current: "page",
            aria_disabled: "true",
            {props.children}
        }
    }
}

/// Props for BreadcrumbSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbSeparatorProps {
    /// Custom separator content (defaults to chevron).
    #[props(default)]
    pub children: Option<Element>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Separator between breadcrumb items.
#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("[&>svg]:size-3.5 {}", custom_class);

    rsx! {
        li {
            role: "presentation",
            class: classes,
            "data-slot": "breadcrumb-separator",
            aria_hidden: "true",

            if let Some(children) = props.children {
                {children}
            } else {
                // Default chevron right
                svg {
                    class: "size-3.5",
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
}

/// Props for BreadcrumbEllipsis.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbEllipsisProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Ellipsis indicating collapsed breadcrumb items.
#[component]
pub fn BreadcrumbEllipsis(props: BreadcrumbEllipsisProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex h-9 w-9 items-center justify-center {}",
        custom_class
    );

    rsx! {
        span {
            role: "presentation",
            class: classes,
            "data-slot": "breadcrumb-ellipsis",
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

            span { class: "sr-only", "More" }
        }
    }
}
