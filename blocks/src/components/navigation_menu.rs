//! NavigationMenu component for site navigation.
//!
//! A navigation menu with support for submenus and viewport positioning.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing navigation menu state.
#[derive(Clone)]
pub struct NavigationMenuContext {
    /// Currently active item value.
    pub value: Signal<Option<String>>,
    /// Previous active item value (for motion direction).
    pub previous_value: Signal<Option<String>>,
    /// Whether to use viewport mode.
    pub use_viewport: bool,
    /// Callback when value changes.
    pub on_value_change: Option<Callback<Option<String>>>,
}

impl NavigationMenuContext {
    /// Set the active item.
    pub fn set_value(&mut self, value: Option<String>) {
        // Store previous value for motion direction
        let current = self.value.read().clone();
        self.previous_value.set(current);
        self.value.set(value.clone());
        if let Some(callback) = &self.on_value_change {
            callback.call(value);
        }
    }

    /// Check if an item is active.
    pub fn is_active(&self, item_value: &str) -> bool {
        self.value.read().as_ref() == Some(&item_value.to_string())
    }

    /// Get motion direction based on navigation.
    pub fn get_motion_direction(&self, item_value: &str) -> &'static str {
        let current = self.value.read();
        let previous = self.previous_value.read();

        match (&*current, &*previous) {
            (Some(curr), Some(prev)) if curr == item_value => {
                // Navigating to this item - determine direction based on order
                // For simplicity, use from-start as default
                "from-start"
            }
            (Some(_), Some(prev)) if prev == item_value => {
                // Navigating away from this item
                "to-end"
            }
            _ => "from-start",
        }
    }
}

/// Props for NavigationMenu.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuProps {
    /// Controlled value.
    #[props(default)]
    pub value: Option<Signal<Option<String>>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Option<Callback<Option<String>>>,

    /// Whether to use viewport mode for content.
    #[props(default = true)]
    pub use_viewport: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Navigation menu content.
    pub children: Element,
}

/// A navigation menu component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     NavigationMenu {
///         NavigationMenuList {
///             NavigationMenuItem {
///                 NavigationMenuTrigger { "Getting started" }
///                 NavigationMenuContent {
///                     // Links here
///                 }
///             }
///             NavigationMenuItem {
///                 NavigationMenuLink { href: "/docs", "Documentation" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn NavigationMenu(props: NavigationMenuProps) -> Element {
    // Internal state for uncontrolled mode
    let internal_value = use_signal(|| props.default_value.clone());
    let previous_value = use_signal(|| None);

    // Use controlled or internal state
    let value = props.value.unwrap_or(internal_value);

    let context = NavigationMenuContext {
        value,
        previous_value,
        use_viewport: props.use_viewport,
        on_value_change: props.on_value_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/navigation-menu relative z-10 flex max-w-max flex-1 items-center justify-center {}",
        custom_class
    );

    rsx! {
        nav {
            class: classes,
            "data-slot": "navigation-menu",
            {props.children}

            if props.use_viewport {
                NavigationMenuViewport {}
            }
        }
    }
}

/// Props for NavigationMenuList.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Menu items.
    pub children: Element,
}

/// Container for navigation menu items.
#[component]
pub fn NavigationMenuList(props: NavigationMenuListProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group flex flex-1 list-none items-center justify-center gap-1 {}",
        custom_class
    );

    rsx! {
        ul {
            class: classes,
            "data-slot": "navigation-menu-list",
            {props.children}
        }
    }
}

/// Props for NavigationMenuItem.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuItemProps {
    /// Unique value for this item.
    #[props(default)]
    pub value: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A single navigation menu item.
#[component]
pub fn NavigationMenuItem(props: NavigationMenuItemProps) -> Element {
    let item_id = use_unique_id();
    let value = props.value.clone().unwrap_or_else(|| item_id());
    let custom_class = props.class.as_deref().unwrap_or("");

    // Provide item value in context for child components
    use_context_provider(|| NavigationMenuItemContext {
        value: value.clone(),
    });

    rsx! {
        li {
            class: custom_class,
            "data-slot": "navigation-menu-item",
            "data-value": value,
            {props.children}
        }
    }
}

/// Context for a navigation menu item.
#[derive(Clone)]
pub struct NavigationMenuItemContext {
    pub value: String,
}

/// Props for NavigationMenuTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuTriggerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content.
    pub children: Element,
}

/// A button that triggers a submenu.
#[component]
pub fn NavigationMenuTrigger(props: NavigationMenuTriggerProps) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let item_context = use_context::<NavigationMenuItemContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let is_active = context.is_active(&item_context.value);

    let classes = format!(
        "group inline-flex h-9 w-max items-center justify-center gap-1 rounded-md bg-background px-4 \
         py-2 text-sm font-medium outline-none transition-[color,box-shadow] \
         hover:bg-accent hover:text-accent-foreground \
         focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 \
         disabled:pointer-events-none disabled:opacity-50 \
         data-[state=open]:bg-accent/50 data-[state=open]:text-accent-foreground {}",
        custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        let value = item_context.value.clone();
        move |_| {
            let current = context.value.read().clone();
            if current.as_ref() == Some(&value) {
                context.set_value(None);
            } else {
                context.set_value(Some(value.clone()));
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "navigation-menu-trigger",
            "data-state": if is_active { "open" } else { "closed" },
            "data-active": is_active.to_string(),
            aria_expanded: is_active.to_string(),
            onclick: handle_click,

            {props.children}

            // Chevron down icon
            svg {
                class: "pointer-events-none size-3 shrink-0 text-muted-foreground transition-transform duration-200 group-data-[state=open]:rotate-180",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

/// Props for NavigationMenuContent.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content.
    pub children: Element,
}

/// The content panel for a navigation menu item.
#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let item_context = use_context::<NavigationMenuItemContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let is_active = context.is_active(&item_context.value);

    if !is_active {
        return rsx! {};
    }

    let motion = context.get_motion_direction(&item_context.value);

    // Non-viewport mode: render inline with top positioning
    let inline_classes = format!(
        "left-0 top-full mt-1.5 w-full \
         data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out \
         data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out \
         data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 \
         data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 \
         md:absolute md:w-auto {}",
        custom_class
    );

    // Viewport mode: content is rendered inside NavigationMenuViewport
    let viewport_classes = format!(
        "left-0 top-0 w-full \
         data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out \
         data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out \
         data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 \
         data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 \
         md:absolute md:w-auto {}",
        custom_class
    );

    let classes = if context.use_viewport {
        viewport_classes
    } else {
        inline_classes
    };

    rsx! {
        div {
            class: classes,
            "data-slot": "navigation-menu-content",
            "data-state": "open",
            "data-motion": motion,
            {props.children}
        }
    }
}

/// Props for NavigationMenuLink.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuLinkProps {
    /// The href for the link.
    #[props(default)]
    pub href: Option<String>,

    /// Whether this link is currently active.
    #[props(default)]
    pub active: bool,

    /// Callback when clicked.
    #[props(default)]
    pub on_click: Option<Callback<()>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Link content.
    pub children: Element,
}

/// A link within the navigation menu.
#[component]
pub fn NavigationMenuLink(props: NavigationMenuLinkProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let active_class = if props.active {
        "bg-accent text-accent-foreground"
    } else {
        ""
    };

    let classes = format!(
        "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-hidden \
         transition-[color,box-shadow] hover:bg-accent hover:text-accent-foreground \
         focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 \
         {} {}",
        active_class, custom_class
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
            "data-slot": "navigation-menu-link",
            "data-active": props.active.to_string(),
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for NavigationMenuIndicator.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuIndicatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Visual indicator showing the active menu item.
#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    if context.value.read().is_none() {
        return rsx! {};
    }

    let classes = format!(
        "top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden \
         animate-in fade-in {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "navigation-menu-indicator",
            "data-state": "visible",

            div {
                class: "relative top-[60%] h-2 w-2 rotate-45 rounded-tl-sm bg-border shadow-md",
            }
        }
    }
}

/// Props for NavigationMenuViewport.
#[derive(Props, Clone, PartialEq)]
pub struct NavigationMenuViewportProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Viewport container for navigation menu content.
#[component]
pub fn NavigationMenuViewport(props: NavigationMenuViewportProps) -> Element {
    let context = use_context::<NavigationMenuContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let has_content = context.value.read().is_some();

    if !has_content {
        return rsx! {};
    }

    // Use fallback dimensions when CSS variables aren't available
    // These match typical navigation menu content sizes
    let classes = format!(
        "origin-top-center relative mt-1.5 \
         h-[var(--radix-navigation-menu-viewport-height,auto)] \
         w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-lg \
         data-[state=open]:animate-in data-[state=closed]:animate-out \
         data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 \
         data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
         md:w-[var(--radix-navigation-menu-viewport-width,auto)] {}",
        custom_class
    );

    rsx! {
        div {
            class: "absolute left-0 top-full flex justify-center w-full",
            "data-slot": "navigation-menu-viewport-position",

            div {
                class: classes,
                "data-slot": "navigation-menu-viewport",
                "data-state": "open",
                // Content is rendered by NavigationMenuContent components
            }
        }
    }
}

/// Hook to access the navigation menu context.
pub fn use_navigation_menu() -> NavigationMenuContext {
    use_context::<NavigationMenuContext>()
}
