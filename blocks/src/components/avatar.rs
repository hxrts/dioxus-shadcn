use crate::use_unique_id;
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_primitives::avatar::{
    Avatar as PrimitiveAvatar, AvatarFallback as PrimitiveAvatarFallback,
    AvatarImage as PrimitiveAvatarImage, AvatarState,
};

/// Avatar size variants matching shadcn-ui.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AvatarSize {
    /// Small size (24px / size-6)
    Sm,
    /// Default size (32px / size-8)
    #[default]
    Default,
    /// Large size (40px / size-10)
    Lg,
}

/// Props for the Avatar component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Size variant for the avatar.
    #[props(default)]
    pub size: AvatarSize,

    /// Optional additional classes for the avatar
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the avatar element
    #[props(default)]
    pub id: Option<String>,

    /// Optional callback when the avatar state changes
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the Avatar component
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // Generate unique ID if not provided
    let avatar_id = use_unique_id();
    let id_value = use_memo(move || props.id.clone().unwrap_or_else(|| avatar_id.peek().clone()));

    let data_size = match props.size {
        AvatarSize::Sm => "sm",
        AvatarSize::Default => "default",
        AvatarSize::Lg => "lg",
    };

    let custom_class = props.class.as_deref().unwrap_or("");

    // Use data-attribute styling for sizes to match shadcn
    let avatar_classes = format!(
        "group/avatar relative flex size-8 shrink-0 overflow-hidden rounded-full select-none \
         data-[size=lg]:size-10 data-[size=sm]:size-6 {}",
        custom_class
    );

    rsx! {
        PrimitiveAvatar {
            id: id_value.peek().clone(),
            class: avatar_classes,
            "data-slot": "avatar",
            "data-size": data_size,
            on_state_change: move |state| {
                if let Some(handler) = &props.on_state_change {
                    handler.call(state);
                }
            },

            {props.children}
        }
    }
}

/// Props for the AvatarImage component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    /// The source URL of the image
    pub src: String,

    /// Alt text for the image
    pub alt: String,

    /// Optional additional classes for the image
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the image element
    #[props(default)]
    pub id: Option<String>,
}

/// Styled wrapper for the AvatarImage component
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    // Generate unique ID if not provided
    let image_id = use_unique_id();
    let id_value = use_memo(move || props.id.clone().unwrap_or_else(|| image_id.peek().clone()));

    let image_classes = vec![
        // Base classes - fill the container and maintain aspect ratio (matches shadcn)
        "aspect-square size-full",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAvatarImage {
            id: id_value.peek().clone(),
            class: image_classes,
            "data-slot": "avatar-image",
            src: props.src,
            alt: props.alt,
        }
    }
}

/// Props for the AvatarFallback component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Optional additional classes for the fallback
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the fallback element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements (typically text or icon)
    pub children: Element,
}

/// Styled wrapper for the AvatarFallback component
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    // Generate unique ID if not provided
    let fallback_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| fallback_id.peek().clone())
    });

    let fallback_classes = vec![
        // Base classes - center content and style text
        "flex size-full items-center justify-center rounded-full bg-muted text-sm text-muted-foreground",
        // Responsive text size based on parent avatar size
        "group-data-[size=sm]/avatar:text-xs",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAvatarFallback {
            id: id_value.peek().clone(),
            class: fallback_classes,
            "data-slot": "avatar-fallback",

            {props.children}
        }
    }
}

/// Props for the AvatarBadge component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarBadgeProps {
    /// Optional additional classes for the badge
    #[props(default)]
    pub class: Option<String>,

    /// Child elements (typically an icon or status indicator)
    #[props(default)]
    pub children: Element,
}

/// A badge that appears on the avatar, typically used for status indicators.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Avatar {
///         size: AvatarSize::Default,
///         AvatarImage { src: "https://example.com/avatar.jpg", alt: "User" }
///         AvatarFallback { "JD" }
///         AvatarBadge {
///             // Online status indicator (green dot)
///         }
///     }
/// }
/// ```
#[component]
pub fn AvatarBadge(props: AvatarBadgeProps) -> Element {
    let badge_classes = vec![
        // Base positioning and styling
        "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full",
        "bg-primary text-primary-foreground ring-2 ring-background select-none",
        // Size responsive to parent avatar size
        "group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&>svg]:hidden",
        "group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&>svg]:size-2",
        "group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&>svg]:size-2",
        // Additional classes
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        span {
            class: badge_classes,
            "data-slot": "avatar-badge",
            {props.children}
        }
    }
}

/// Props for the AvatarGroup component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Optional additional classes for the group
    #[props(default)]
    pub class: Option<String>,

    /// Child elements (typically Avatar components)
    pub children: Element,
}

/// A container for displaying multiple avatars in a stacked layout.
///
/// # Example
///
/// ```rust
/// rsx! {
///     AvatarGroup {
///         Avatar {
///             AvatarImage { src: "https://example.com/user1.jpg", alt: "User 1" }
///             AvatarFallback { "U1" }
///         }
///         Avatar {
///             AvatarImage { src: "https://example.com/user2.jpg", alt: "User 2" }
///             AvatarFallback { "U2" }
///         }
///         AvatarGroupCount { "+3" }
///     }
/// }
/// ```
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let group_classes = vec![
        // Base layout with negative spacing for overlap
        "group/avatar-group flex -space-x-2",
        // Add ring to child avatars for visual separation
        "*:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background",
        // Additional classes
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        div {
            class: group_classes,
            "data-slot": "avatar-group",
            {props.children}
        }
    }
}

/// Props for the AvatarGroupCount component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupCountProps {
    /// Optional additional classes for the count
    #[props(default)]
    pub class: Option<String>,

    /// Child elements (typically the count text like "+3")
    pub children: Element,
}

/// A count indicator for avatar groups showing additional members.
///
/// # Example
///
/// ```rust
/// rsx! {
///     AvatarGroup {
///         Avatar { /* ... */ }
///         Avatar { /* ... */ }
///         AvatarGroupCount { "+5" }
///     }
/// }
/// ```
#[component]
pub fn AvatarGroupCount(props: AvatarGroupCountProps) -> Element {
    let count_classes = vec![
        // Base styling matching avatar dimensions
        "relative flex size-8 shrink-0 items-center justify-center rounded-full",
        "bg-muted text-sm text-muted-foreground ring-2 ring-background",
        // Size responsive to sibling avatars in the group
        "group-has-data-[size=lg]/avatar-group:size-10",
        "group-has-data-[size=sm]/avatar-group:size-6",
        // SVG sizing
        "[&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3",
        // Additional classes
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        div {
            class: count_classes,
            "data-slot": "avatar-group-count",
            {props.children}
        }
    }
}
