//! Select component for choosing from a list of options.
//!
//! A dropdown select component with keyboard navigation and accessibility support.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing select state.
#[derive(Clone)]
pub struct SelectContext {
    /// Whether the select is open.
    pub open: Signal<bool>,
    /// The currently selected value.
    pub value: Signal<Option<String>>,
    /// The display text for the selected value.
    pub display_value: Signal<Option<String>>,
    /// Whether the select is disabled.
    pub disabled: bool,
    /// Callback when value changes.
    pub on_value_change: Option<Callback<String>>,
    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,
    /// The trigger element ID for positioning.
    pub trigger_id: String,
    /// The content element ID.
    pub content_id: String,
}

impl SelectContext {
    /// Select a value and update display.
    pub fn select(&mut self, value: String, display: String) {
        self.value.set(Some(value.clone()));
        self.display_value.set(Some(display));
        self.open.set(false);
        if let Some(callback) = &self.on_value_change {
            callback.call(value);
        }
        if let Some(callback) = &self.on_open_change {
            callback.call(false);
        }
    }

    /// Toggle the open state.
    pub fn toggle(&mut self) {
        if !self.disabled {
            let new_state = !*self.open.read();
            self.open.set(new_state);
            if let Some(callback) = &self.on_open_change {
                callback.call(new_state);
            }
        }
    }

    /// Close the select.
    pub fn close(&mut self) {
        self.open.set(false);
        if let Some(callback) = &self.on_open_change {
            callback.call(false);
        }
    }
}

/// Size variants for the select trigger.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SelectSize {
    Sm,
    #[default]
    Default,
}

/// Props for Select.
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Controlled value.
    #[props(default)]
    pub value: Option<Signal<Option<String>>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Callback when value changes.
    #[props(default)]
    pub on_value_change: Option<Callback<String>>,

    /// Controlled open state.
    #[props(default)]
    pub open: Option<Signal<bool>>,

    /// Default open state for uncontrolled mode.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Whether the select is disabled.
    #[props(default)]
    pub disabled: bool,

    /// The name for form submission.
    #[props(default)]
    pub name: Option<String>,

    /// Whether the select is required.
    #[props(default)]
    pub required: bool,

    /// Select content.
    pub children: Element,
}

/// A select component for choosing from a list of options.
///
/// # Example
///
/// ```rust
/// let selected = use_signal(|| None::<String>);
///
/// rsx! {
///     Select {
///         value: selected,
///         on_value_change: move |v| selected.set(Some(v)),
///
///         SelectTrigger {
///             SelectValue { placeholder: "Select a fruit..." }
///         }
///         SelectContent {
///             SelectGroup {
///                 SelectLabel { "Fruits" }
///                 SelectItem { value: "apple", "Apple" }
///                 SelectItem { value: "banana", "Banana" }
///                 SelectItem { value: "orange", "Orange" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Select(props: SelectProps) -> Element {
    let select_id = use_unique_id();
    let trigger_id = format!("select-trigger-{}", select_id());
    let content_id = format!("select-content-{}", select_id());

    // Internal state for uncontrolled mode
    let internal_open = use_signal(|| props.default_open);
    let internal_value = use_signal(|| props.default_value.clone());

    // Use controlled or internal state
    let open = props.open.unwrap_or(internal_open);
    let value = props.value.unwrap_or(internal_value);

    let display_value = use_signal(|| None::<String>);

    let context = SelectContext {
        open,
        value,
        display_value,
        disabled: props.disabled,
        on_value_change: props.on_value_change.clone(),
        on_open_change: props.on_open_change.clone(),
        trigger_id: trigger_id.clone(),
        content_id: content_id.clone(),
    };

    use_context_provider(|| context);

    // Handle escape key to close
    let handle_keydown = {
        let mut open = open;
        let on_open_change = props.on_open_change.clone();
        move |event: KeyboardEvent| {
            if event.key() == Key::Escape && *open.read() {
                open.set(false);
                if let Some(callback) = &on_open_change {
                    callback.call(false);
                }
            }
        }
    };

    rsx! {
        div {
            "data-slot": "select",
            "data-state": if *open.read() { "open" } else { "closed" },
            "data-disabled": props.disabled.to_string(),
            onkeydown: handle_keydown,

            {props.children}

            // Hidden input for form submission
            if let Some(name) = &props.name {
                input {
                    r#type: "hidden",
                    name: name.clone(),
                    value: value.read().clone().unwrap_or_default(),
                    required: props.required,
                }
            }
        }
    }
}

/// Props for SelectTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// Size variant.
    #[props(default)]
    pub size: SelectSize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content (typically SelectValue).
    pub children: Element,
}

/// The button that triggers the select dropdown.
#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let context = use_context::<SelectContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex w-fit items-center justify-between gap-2 rounded-md border border-input \
         bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs \
         transition-[color,box-shadow] outline-none \
         focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:cursor-not-allowed disabled:opacity-50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 \
         data-[placeholder]:text-muted-foreground \
         data-[size=default]:h-9 data-[size=sm]:h-8 \
         *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex \
         *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 \
         dark:bg-input/30 dark:hover:bg-input/50 dark:aria-invalid:ring-destructive/40 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground \
         {}",
        custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        move |_| {
            context.toggle();
        }
    };

    let handle_keydown = {
        let mut context = context.clone();
        move |event: KeyboardEvent| match event.key() {
            Key::Enter => {
                event.prevent_default();
                context.toggle();
            }
            Key::Character(ref s) if s == " " => {
                event.prevent_default();
                context.toggle();
            }
            Key::ArrowDown | Key::ArrowUp => {
                event.prevent_default();
                if !*context.open.read() {
                    context.open.set(true);
                    if let Some(callback) = &context.on_open_change {
                        callback.call(true);
                    }
                }
            }
            _ => {}
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "combobox",
            id: context.trigger_id.clone(),
            class: classes,
            "data-slot": "select-trigger",
            "data-size": match props.size {
                SelectSize::Sm => "sm",
                SelectSize::Default => "default",
            },
            "data-state": if *context.open.read() { "open" } else { "closed" },
            "data-placeholder": context.display_value.read().is_none().to_string(),
            aria_expanded: context.open.read().to_string(),
            aria_haspopup: "listbox",
            aria_controls: context.content_id.clone(),
            disabled: context.disabled,
            onclick: handle_click,
            onkeydown: handle_keydown,

            {props.children}

            // Chevron icon
            span {
                class: "pointer-events-none shrink-0",
                "data-slot": "select-icon",
                svg {
                    class: "size-4 opacity-50",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline {
                        points: "6 9 12 15 18 9",
                    }
                }
            }
        }
    }
}

/// Props for SelectValue.
#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    /// Placeholder text when no value is selected.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Displays the currently selected value or placeholder.
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let context = use_context::<SelectContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("line-clamp-1 flex items-center gap-2 {}", custom_class);

    let display = context.display_value.read().clone();
    let placeholder = props.placeholder.clone();

    rsx! {
        span {
            class: classes,
            "data-slot": "select-value",
            "data-placeholder": display.is_none().to_string(),

            if let Some(ref text) = display {
                "{text}"
            } else if let Some(placeholder) = placeholder {
                span {
                    class: "text-muted-foreground",
                    "{placeholder}"
                }
            }
        }
    }
}

/// Props for SelectContent.
#[derive(Props, Clone, PartialEq)]
pub struct SelectContentProps {
    /// Position strategy.
    #[props(default)]
    pub position: SelectContentPosition,

    /// Side offset in pixels.
    #[props(default = 4)]
    pub side_offset: i32,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content items.
    pub children: Element,
}

/// Position strategy for the select content.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SelectContentPosition {
    #[default]
    ItemAligned,
    Popper,
}

/// The dropdown content containing select items.
#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let context = use_context::<SelectContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    if !*context.open.read() {
        return rsx! {};
    }

    let position_class = match props.position {
        SelectContentPosition::ItemAligned => "",
        SelectContentPosition::Popper => "translate-y-1",
    };

    let classes = format!(
        "absolute z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border \
         bg-popover text-popover-foreground shadow-md \
         data-[state=open]:animate-in data-[state=closed]:animate-out \
         data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
         data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 \
         data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
         data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
         {} {}",
        position_class, custom_class
    );

    let context_for_overlay = context.clone();
    rsx! {
        // Overlay to catch clicks outside
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| {
                let mut ctx = context_for_overlay.clone();
                ctx.close();
            },
        }

        // Content panel
        div {
            role: "listbox",
            id: context.content_id.clone(),
            class: classes,
            "data-slot": "select-content",
            "data-state": "open",
            "data-side": "bottom",
            style: "margin-top: {props.side_offset}px;",
            tabindex: "-1",

            SelectScrollUpButton {}

            div {
                class: "p-1 max-h-80 overflow-y-auto scroll-my-1",
                "data-slot": "select-viewport",
                {props.children}
            }

            SelectScrollDownButton {}
        }
    }
}

/// Props for SelectGroup.
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group content.
    pub children: Element,
}

/// A group of related select items.
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            role: "group",
            class: custom_class,
            "data-slot": "select-group",
            {props.children}
        }
    }
}

/// Props for SelectLabel.
#[derive(Props, Clone, PartialEq)]
pub struct SelectLabelProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Label content.
    pub children: Element,
}

/// A label for a group of select items.
#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("px-2 py-1.5 text-xs text-muted-foreground {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "select-label",
            {props.children}
        }
    }
}

/// Props for SelectItem.
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemProps {
    /// The value of this item.
    pub value: String,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A selectable item within the select dropdown.
#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let context = use_context::<SelectContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let is_selected = context.value.read().as_ref() == Some(&props.value);

    let classes = format!(
        "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 \
         text-sm outline-hidden select-none \
         focus:bg-accent focus:text-accent-foreground \
         data-[disabled]:pointer-events-none data-[disabled]:opacity-50 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground \
         *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 \
         {}",
        custom_class
    );

    let handle_click = {
        let mut context = context.clone();
        let value = props.value.clone();
        let disabled = props.disabled;
        move |_| {
            if !disabled {
                // We need to get the display text from the children
                // For now, use the value as the display
                context.select(value.clone(), value.clone());
            }
        }
    };

    let handle_keydown = {
        let mut context = context.clone();
        let value = props.value.clone();
        let disabled = props.disabled;
        move |event: KeyboardEvent| {
            if !disabled {
                match event.key() {
                    Key::Enter => {
                        event.prevent_default();
                        context.select(value.clone(), value.clone());
                    }
                    Key::Character(ref s) if s == " " => {
                        event.prevent_default();
                        context.select(value.clone(), value.clone());
                    }
                    _ => {}
                }
            }
        }
    };

    rsx! {
        div {
            role: "option",
            class: classes,
            "data-slot": "select-item",
            "data-value": props.value.clone(),
            "data-state": if is_selected { "checked" } else { "unchecked" },
            "data-disabled": props.disabled.to_string(),
            aria_selected: is_selected.to_string(),
            aria_disabled: props.disabled.to_string(),
            tabindex: if props.disabled { "-1" } else { "0" },
            onclick: handle_click,
            onkeydown: handle_keydown,

            // Check indicator
            span {
                class: "absolute right-2 flex size-3.5 items-center justify-center",
                "data-slot": "select-item-indicator",

                if is_selected {
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
                        polyline {
                            points: "20 6 9 17 4 12",
                        }
                    }
                }
            }

            span {
                "data-slot": "select-item-text",
                {props.children}
            }
        }
    }
}

/// Props for SelectSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct SelectSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between select items.
#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "pointer-events-none -mx-1 my-1 h-px bg-border {}",
        custom_class
    );

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "select-separator",
        }
    }
}

/// Props for SelectScrollUpButton.
#[derive(Props, Clone, PartialEq)]
pub struct SelectScrollUpButtonProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A button that scrolls the select content up when there is overflow.
///
/// This button appears at the top of the select content when there are items
/// above the visible area that can be scrolled to.
#[component]
pub fn SelectScrollUpButton(props: SelectScrollUpButtonProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex cursor-default items-center justify-center py-1 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "select-scroll-up-button",
            aria_hidden: "true",

            // ChevronUp icon
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
                polyline {
                    points: "18 15 12 9 6 15",
                }
            }
        }
    }
}

/// Props for SelectScrollDownButton.
#[derive(Props, Clone, PartialEq)]
pub struct SelectScrollDownButtonProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A button that scrolls the select content down when there is overflow.
///
/// This button appears at the bottom of the select content when there are items
/// below the visible area that can be scrolled to.
#[component]
pub fn SelectScrollDownButton(props: SelectScrollDownButtonProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex cursor-default items-center justify-center py-1 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "select-scroll-down-button",
            aria_hidden: "true",

            // ChevronDown icon
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
                polyline {
                    points: "6 9 12 15 18 9",
                }
            }
        }
    }
}

/// Hook to access the select context.
pub fn use_select() -> SelectContext {
    use_context::<SelectContext>()
}
