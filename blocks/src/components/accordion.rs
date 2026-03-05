use crate::use_unique_id;
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion as PrimitiveAccordion, AccordionContent as PrimitiveAccordionContent,
    AccordionItem as PrimitiveAccordionItem, AccordionTrigger as PrimitiveAccordionTrigger,
};
use lucide_dioxus::ChevronDown;

/// Props for the Accordion component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Whether multiple items can be open at the same time
    #[props(default)]
    pub allow_multiple_open: bool,

    /// Whether the accordion is oriented horizontally
    #[props(default)]
    pub horizontal: bool,

    /// Optional additional classes for the accordion
    #[props(default)]
    pub class: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the Accordion component
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let accordion_classes = vec![
        // Base classes
        "w-full",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAccordion {
            class: accordion_classes,
            allow_multiple_open: props.allow_multiple_open,
            horizontal: props.horizontal,
            "data-slot": "accordion",

            {props.children}
        }
    }
}

/// Props for the AccordionItem component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    /// The index of this item in the accordion
    pub index: usize,

    /// Optional additional classes for the item
    #[props(default)]
    pub class: Option<String>,

    /// Optional callback when the item's open state changes
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,

    /// Optional callback when the trigger is clicked
    #[props(default)]
    pub on_trigger_click: Option<EventHandler<()>>,

    /// Optional ID for the item element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the AccordionItem component
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let id_value = use_memo(move || props.id.clone().unwrap_or_else(|| item_id.peek().clone()));

    let item_classes = vec![
        // Base classes matching shadcn
        "border-b last:border-b-0",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAccordionItem {
            id: id_value.peek().clone(),
            class: item_classes,
            index: props.index,
            "data-slot": "accordion-item",
            on_change: move |open| {
                if let Some(handler) = &props.on_change {
                    handler.call(open);
                }
            },
            on_trigger_click: move || {
                if let Some(handler) = &props.on_trigger_click {
                    handler.call(());
                }
            },

            {props.children}
        }
    }
}

/// Props for the AccordionTrigger component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    /// Optional additional classes for the trigger
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the trigger element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the AccordionTrigger component
#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    // Generate unique ID if not provided
    let trigger_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| trigger_id.peek().clone())
    });

    let custom_class = props.class.as_deref().unwrap_or("");

    let trigger_classes = format!(
        "flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium \
         transition-all outline-none hover:underline \
         focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:pointer-events-none disabled:opacity-50 \
         [&[data-state=open]>svg]:rotate-180 {}",
        custom_class
    );

    rsx! {
        h3 {
            class: "flex",
            "data-slot": "accordion-header",

            PrimitiveAccordionTrigger {
                id: id_value.peek().clone(),
                class: trigger_classes,
                "data-slot": "accordion-trigger",

                {props.children}

                // Chevron icon
                ChevronDown {
                    class: "pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200"
                }
            }
        }
    }
}

/// Props for the AccordionContent component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    /// Optional additional classes for the content
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the content element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the AccordionContent component
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    // Generate unique ID if not provided
    let content_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| content_id.peek().clone())
    });

    let custom_class = props.class.as_deref().unwrap_or("");

    let content_classes = format!(
        "data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down \
         overflow-hidden text-sm {}",
        custom_class
    );

    rsx! {
        PrimitiveAccordionContent {
            id: id_value.peek().clone(),
            class: content_classes,
            "data-slot": "accordion-content",

            div {
                class: "pt-0 pb-4",
                {props.children}
            }
        }
    }
}
