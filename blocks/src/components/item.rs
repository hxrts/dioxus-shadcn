//! Item component for list items and cards.
//!
//! A flexible item component system for building lists, cards,
//! and other repeatable content structures.

use dioxus::prelude::*;

/// Variant styles for Item.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

/// Size variants for Item.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

/// Media variant for ItemMedia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

/// Props for ItemGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ItemGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group items.
    pub children: Element,
}

/// A container for grouping items.
///
/// # Example
///
/// ```rust
/// rsx! {
///     ItemGroup {
///         Item {
///             ItemMedia { variant: ItemMediaVariant::Icon,
///                 // Icon here
///             }
///             ItemContent {
///                 ItemTitle { "Item Title" }
///                 ItemDescription { "Item description text." }
///             }
///             ItemActions {
///                 Button { "Action" }
///             }
///         }
///         ItemSeparator {}
///         Item {
///             ItemContent {
///                 ItemTitle { "Another Item" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ItemGroup(props: ItemGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-col {}",
        custom_class
    );

    rsx! {
        div {
            role: "list",
            class: classes,
            "data-slot": "item-group",
            {props.children}
        }
    }
}

/// Props for Item.
#[derive(Props, Clone, PartialEq)]
pub struct ItemProps {
    /// Visual variant.
    #[props(default)]
    pub variant: ItemVariant,

    /// Size variant.
    #[props(default)]
    pub size: ItemSize,

    /// Whether the item is interactive (clickable).
    #[props(default)]
    pub interactive: bool,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Click handler for interactive items.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A flexible item component for lists and cards.
#[component]
pub fn Item(props: ItemProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        ItemVariant::Default => "",
        ItemVariant::Outline => "rounded-lg border bg-card shadow-sm",
        ItemVariant::Muted => "rounded-lg bg-muted",
    };

    let size_class = match props.size {
        ItemSize::Default => "gap-3 p-3",
        ItemSize::Sm => "gap-2 p-2",
    };

    let interactive_class = if props.interactive {
        "cursor-pointer transition-colors hover:bg-accent focus-visible:outline-1 \
         focus-visible:ring-[3px] focus-visible:ring-ring/50"
    } else {
        ""
    };

    let disabled_class = if props.disabled {
        "pointer-events-none opacity-50"
    } else {
        ""
    };

    let classes = format!(
        "flex items-start \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {} {} {} {} {}",
        variant_class, size_class, interactive_class, disabled_class, custom_class
    );

    let handle_click = move |event: MouseEvent| {
        if !props.disabled {
            if let Some(handler) = &props.on_click {
                handler.call(event);
            }
        }
    };

    rsx! {
        div {
            role: "listitem",
            class: classes,
            "data-slot": "item",
            "data-variant": match props.variant {
                ItemVariant::Default => "default",
                ItemVariant::Outline => "outline",
                ItemVariant::Muted => "muted",
            },
            "data-size": match props.size {
                ItemSize::Default => "default",
                ItemSize::Sm => "sm",
            },
            "data-interactive": props.interactive.to_string(),
            "data-disabled": props.disabled.to_string(),
            tabindex: if props.interactive && !props.disabled { "0" } else { "-1" },
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for ItemMedia.
#[derive(Props, Clone, PartialEq)]
pub struct ItemMediaProps {
    /// Visual variant.
    #[props(default)]
    pub variant: ItemMediaVariant,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Media content (icon or image).
    pub children: Element,
}

/// Media section for an item (icon or image).
#[component]
pub fn ItemMedia(props: ItemMediaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let variant_class = match props.variant {
        ItemMediaVariant::Default => "shrink-0",
        ItemMediaVariant::Icon => {
            "flex size-8 shrink-0 items-center justify-center rounded-md border bg-background \
             [&_svg:not([class*='size-'])]:size-4"
        }
        ItemMediaVariant::Image => {
            "size-10 shrink-0 overflow-hidden rounded-md \
             [&_img]:size-full [&_img]:object-cover"
        }
    };

    let classes = format!("{} {}", variant_class, custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "item-media",
            "data-variant": match props.variant {
                ItemMediaVariant::Default => "default",
                ItemMediaVariant::Icon => "icon",
                ItemMediaVariant::Image => "image",
            },
            {props.children}
        }
    }
}

/// Props for ItemContent.
#[derive(Props, Clone, PartialEq)]
pub struct ItemContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content.
    pub children: Element,
}

/// Content container for an item.
#[component]
pub fn ItemContent(props: ItemContentProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex min-w-0 flex-1 flex-col gap-1 \
         [&+[data-slot=item-content]]:border-t [&+[data-slot=item-content]]:pt-3 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "item-content",
            {props.children}
        }
    }
}

/// Props for ItemTitle.
#[derive(Props, Clone, PartialEq)]
pub struct ItemTitleProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Title content.
    pub children: Element,
}

/// Title for an item.
#[component]
pub fn ItemTitle(props: ItemTitleProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex flex-wrap items-center gap-x-2 text-sm font-medium {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "item-title",
            {props.children}
        }
    }
}

/// Props for ItemDescription.
#[derive(Props, Clone, PartialEq)]
pub struct ItemDescriptionProps {
    /// Maximum number of lines to display.
    #[props(default)]
    pub lines: Option<u32>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Description content.
    pub children: Element,
}

/// Description text for an item.
#[component]
pub fn ItemDescription(props: ItemDescriptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let line_clamp = props.lines.map(|n| format!("line-clamp-{}", n)).unwrap_or_default();

    let classes = format!(
        "text-sm text-muted-foreground \
         [&_a]:text-foreground [&_a]:underline [&_a]:underline-offset-4 [&_a]:hover:text-primary \
         {} {}",
        line_clamp, custom_class
    );

    rsx! {
        p {
            class: classes,
            "data-slot": "item-description",
            {props.children}
        }
    }
}

/// Props for ItemActions.
#[derive(Props, Clone, PartialEq)]
pub struct ItemActionsProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Action buttons.
    pub children: Element,
}

/// Actions container for an item.
#[component]
pub fn ItemActions(props: ItemActionsProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex shrink-0 items-center gap-2 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "item-actions",
            {props.children}
        }
    }
}

/// Props for ItemHeader.
#[derive(Props, Clone, PartialEq)]
pub struct ItemHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Header section for an item.
#[component]
pub fn ItemHeader(props: ItemHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex w-full items-center justify-between gap-2 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "item-header",
            {props.children}
        }
    }
}

/// Props for ItemFooter.
#[derive(Props, Clone, PartialEq)]
pub struct ItemFooterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content.
    pub children: Element,
}

/// Footer section for an item.
#[component]
pub fn ItemFooter(props: ItemFooterProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex w-full items-center justify-between gap-2 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "item-footer",
            {props.children}
        }
    }
}

/// Props for ItemSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct ItemSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between items.
#[component]
pub fn ItemSeparator(props: ItemSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "-mx-3 h-px bg-border {}",
        custom_class
    );

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "item-separator",
        }
    }
}
