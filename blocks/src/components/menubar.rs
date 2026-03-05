//! Menubar component matching shadcn-ui patterns.
//!
//! A horizontal menu bar with support for menus, submenus, checkbox/radio items, and shortcuts.

use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    Menubar as PrimitiveMenubar, MenubarContent as PrimitiveMenubarContent,
    MenubarItem as PrimitiveMenubarItem, MenubarMenu as PrimitiveMenubarMenu,
    MenubarTrigger as PrimitiveMenubarTrigger,
};
use lucide_dioxus::{Check, ChevronRight, Circle};

// ============================================================================
// Context for radio groups
// ============================================================================

#[derive(Clone, PartialEq)]
struct MenubarRadioGroupContext {
    value: Signal<String>,
    on_change: EventHandler<String>,
}

// ============================================================================
// Context for submenu state
// ============================================================================

#[derive(Clone)]
struct MenubarSubContext {
    open: Signal<bool>,
}

// ============================================================================
// Main Components
// ============================================================================

/// Menubar main container, styled with Tailwind.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let default_classes =
        "flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubar {
            class: class,
            "data-slot": "menubar",
            {props.children}
        }
    }
}

/// MenubarMenu: A single menu in the menubar.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    #[props(default)]
    pub class: Option<String>,
    pub index: ReadSignal<usize>,
    pub children: Element,
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let default_classes = "relative group flex flex-col items-stretch";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarMenu {
            class: class,
            index: props.index,
            "data-slot": "menubar-menu",
            {props.children}
        }
    }
}

/// MenubarTrigger: The button that opens a menu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let default_classes = "flex items-center rounded-sm px-3 py-1 text-sm font-medium outline-hidden \
        transition-[color,box-shadow] focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=open]:bg-accent data-[state=open]:text-accent-foreground";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarTrigger {
            class: class,
            "data-slot": "menubar-trigger",
            {props.children}
        }
    }
}

/// MenubarContent: The dropdown content for a menu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let default_classes = "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md \
        data-[state=open]:animate-in data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
        data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 \
        data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
        data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
        absolute left-0 top-full";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarContent {
            class: class,
            "data-slot": "menubar-content",
            {props.children}
        }
    }
}

/// MenubarItem: An item in a menu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// The index of this item within the MenubarContent.
    pub index: ReadSignal<usize>,
    #[props(default)]
    pub class: Option<String>,
    pub value: String,
    #[props(default)]
    pub on_select: Callback<String>,
    /// Whether this is a destructive action.
    #[props(default)]
    pub destructive: bool,
    /// Whether to show inset padding (for items without indicator).
    #[props(default)]
    pub inset: bool,
    pub children: Element,
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let inset_class = if props.inset { "pl-8" } else { "" };
    let destructive_class = if props.destructive {
        "text-destructive focus:bg-destructive/10 focus:text-destructive dark:focus:bg-destructive/20"
    } else {
        ""
    };

    let default_classes = format!(
        "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 \
        text-sm outline-hidden transition-colors focus:bg-accent focus:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50 \
        [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground \
        {} {}",
        inset_class, destructive_class
    );

    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes
    };

    rsx! {
        PrimitiveMenubarItem {
            index: props.index,
            class: class,
            value: props.value.clone(),
            on_select: props.on_select,
            "data-slot": "menubar-item",
            {props.children}
        }
    }
}

// ============================================================================
// Separator
// ============================================================================

/// MenubarSeparator: A horizontal divider between items.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSeparatorProps {
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn MenubarSeparator(props: MenubarSeparatorProps) -> Element {
    let default_classes = "-mx-1 my-1 h-px bg-border";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        div {
            class: class,
            "data-slot": "menubar-separator",
            role: "separator",
            aria_orientation: "horizontal",
        }
    }
}

// ============================================================================
// Label
// ============================================================================

/// MenubarLabel: A non-interactive label for grouping items.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarLabelProps {
    #[props(default)]
    pub class: Option<String>,
    /// Whether to show inset padding.
    #[props(default)]
    pub inset: bool,
    pub children: Element,
}

#[component]
pub fn MenubarLabel(props: MenubarLabelProps) -> Element {
    let inset_class = if props.inset { "pl-8" } else { "" };
    let default_classes = format!("px-2 py-1.5 text-sm font-medium {}", inset_class);
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes
    };

    rsx! {
        div {
            class: class,
            "data-slot": "menubar-label",
            {props.children}
        }
    }
}

// ============================================================================
// Shortcut
// ============================================================================

/// MenubarShortcut: A keyboard shortcut hint.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarShortcutProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarShortcut(props: MenubarShortcutProps) -> Element {
    let default_classes = "ml-auto text-xs tracking-widest text-muted-foreground";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    rsx! {
        span {
            class: class,
            "data-slot": "menubar-shortcut",
            {props.children}
        }
    }
}

// ============================================================================
// Checkbox Item
// ============================================================================

/// MenubarCheckboxItem: A toggleable checkbox item.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarCheckboxItemProps {
    pub index: ReadSignal<usize>,
    /// Whether the checkbox is checked.
    pub checked: bool,
    /// Callback when the checked state changes.
    #[props(default)]
    pub on_checked_change: Option<Callback<bool>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub disabled: bool,
    pub children: Element,
}

#[component]
pub fn MenubarCheckboxItem(props: MenubarCheckboxItemProps) -> Element {
    let default_classes = "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 \
        text-sm outline-hidden transition-colors focus:bg-accent focus:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    let checked = props.checked;
    let on_checked_change = props.on_checked_change.clone();
    let handle_select = move |_: String| {
        if let Some(callback) = &on_checked_change {
            callback.call(!checked);
        }
    };

    rsx! {
        PrimitiveMenubarItem {
            index: props.index,
            class: class,
            value: "checkbox".to_string(),
            on_select: handle_select,
            "data-slot": "menubar-checkbox-item",
            "data-state": if props.checked { "checked" } else { "unchecked" },

            // Indicator
            span {
                class: "absolute left-2 flex size-3.5 items-center justify-center",
                "data-slot": "menubar-item-indicator",

                if props.checked {
                    Check { class: "size-4" }
                }
            }

            {props.children}
        }
    }
}

// ============================================================================
// Radio Group
// ============================================================================

/// MenubarRadioGroup: A group of mutually exclusive radio items.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarRadioGroupProps {
    /// The current selected value.
    pub value: Signal<String>,
    /// Callback when the value changes.
    #[props(default)]
    pub on_value_change: Option<EventHandler<String>>,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarRadioGroup(props: MenubarRadioGroupProps) -> Element {
    // Provide context for radio items
    if let Some(handler) = &props.on_value_change {
        use_context_provider(|| MenubarRadioGroupContext {
            value: props.value,
            on_change: *handler,
        });
    }

    let class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: class,
            role: "radiogroup",
            "data-slot": "menubar-radio-group",
            {props.children}
        }
    }
}

/// MenubarRadioItem: A radio item within a radio group.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarRadioItemProps {
    pub index: ReadSignal<usize>,
    /// The value of this radio item.
    pub value: String,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub disabled: bool,
    pub children: Element,
}

#[component]
pub fn MenubarRadioItem(props: MenubarRadioItemProps) -> Element {
    let context = use_context::<MenubarRadioGroupContext>();
    let is_selected = *context.value.read() == props.value;

    let default_classes = "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 \
        text-sm outline-hidden transition-colors focus:bg-accent focus:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    let value_for_handler = props.value.clone();
    let context_clone = context.clone();
    let handle_select = move |_: String| {
        context_clone.on_change.call(value_for_handler.clone());
    };

    rsx! {
        PrimitiveMenubarItem {
            index: props.index,
            class: class,
            value: props.value.clone(),
            on_select: handle_select,
            "data-slot": "menubar-radio-item",
            "data-state": if is_selected { "checked" } else { "unchecked" },

            // Indicator
            span {
                class: "absolute left-2 flex size-3.5 items-center justify-center",
                "data-slot": "menubar-item-indicator",

                if is_selected {
                    Circle { class: "size-2 fill-current" }
                }
            }

            {props.children}
        }
    }
}

// ============================================================================
// Submenu Components
// ============================================================================

/// MenubarSub: Container for a submenu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarSub(props: MenubarSubProps) -> Element {
    let open = use_signal(|| false);
    use_context_provider(|| MenubarSubContext { open });

    let class = props.class.as_deref().unwrap_or("relative");

    rsx! {
        div {
            class: class,
            "data-slot": "menubar-sub",
            "data-state": if *open.read() { "open" } else { "closed" },
            {props.children}
        }
    }
}

/// MenubarSubTrigger: The item that opens a submenu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubTriggerProps {
    pub index: ReadSignal<usize>,
    #[props(default)]
    pub class: Option<String>,
    /// Whether to show inset padding.
    #[props(default)]
    pub inset: bool,
    pub children: Element,
}

#[component]
pub fn MenubarSubTrigger(props: MenubarSubTriggerProps) -> Element {
    let context = use_context::<MenubarSubContext>();
    let is_open = *context.open.read();

    let inset_class = if props.inset { "pl-8" } else { "" };
    let default_classes = format!(
        "flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 \
        text-sm outline-hidden focus:bg-accent focus:text-accent-foreground \
        data-[state=open]:bg-accent data-[state=open]:text-accent-foreground {}",
        inset_class
    );
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes
    };

    let mut ctx = context.clone();
    let handle_mouse_enter = move |_| {
        ctx.open.set(true);
    };

    let mut ctx2 = context.clone();
    let handle_mouse_leave = move |_| {
        ctx2.open.set(false);
    };

    rsx! {
        div {
            class: class,
            "data-slot": "menubar-sub-trigger",
            "data-state": if is_open { "open" } else { "closed" },
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            {props.children}

            ChevronRight { class: "ml-auto size-4" }
        }
    }
}

/// MenubarSubContent: The content of a submenu.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarSubContentProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarSubContent(props: MenubarSubContentProps) -> Element {
    let context = use_context::<MenubarSubContext>();
    let is_open = *context.open.read();

    if !is_open {
        return rsx! {};
    }

    let default_classes = "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg \
        data-[state=open]:animate-in data-[state=closed]:animate-out \
        data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 \
        data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 \
        data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
        data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
        absolute left-full top-0 ml-1";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", default_classes, extra)
    } else {
        default_classes.to_string()
    };

    let mut ctx = context.clone();
    let handle_mouse_enter = move |_| {
        ctx.open.set(true);
    };

    let mut ctx2 = context.clone();
    let handle_mouse_leave = move |_| {
        ctx2.open.set(false);
    };

    rsx! {
        div {
            class: class,
            "data-slot": "menubar-sub-content",
            "data-state": "open",
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            {props.children}
        }
    }
}

/// MenubarGroup: A group of related items.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarGroupProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarGroup(props: MenubarGroupProps) -> Element {
    let class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: class,
            role: "group",
            "data-slot": "menubar-group",
            {props.children}
        }
    }
}
