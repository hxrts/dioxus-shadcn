use dioxus::prelude::*;
use dioxus_primitives::hover_card::{
    HoverCard as PrimitiveHoverCard, HoverCardContent as PrimitiveHoverCardContent,
    HoverCardTrigger as PrimitiveHoverCardTrigger,
};

use dioxus::html::GlobalAttributesExtension;
pub use dioxus_primitives::{ContentAlign as HoverCardAlign, ContentSide as HoverCardSide};

/// HoverCard main container, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let default_classes = "relative inline-block";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCard {
            class: class,
            "data-slot": "hover-card",
            {props.children}
        }
    }
}

/// HoverCardTrigger: The element that triggers the hover card, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let default_classes = "cursor-pointer focus-visible:outline-none";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCardTrigger {
            class: class,
            "data-slot": "hover-card-trigger",
            {props.children}
        }
    }
}

/// HoverCardContent: The floating card content, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub side: Option<HoverCardSide>,
    #[props(default)]
    pub align: Option<HoverCardAlign>,
    pub children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let default_classes = "pointer-events-none opacity-0 data-[state=open]:pointer-events-auto data-[state=open]:opacity-100 absolute top-full z-50 transition-all duration-200 py-2";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCardContent {
            class: class,
            side: props.side.unwrap_or(HoverCardSide::Top),
            align: props.align.unwrap_or(HoverCardAlign::Center),
            "data-slot": "hover-card-content",
            div {
                class: "w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-hidden \
                        data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
                        data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
                        data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
                        data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
                {props.children}
            }
        }
    }
}
