//! Combobox component for searchable selection.
//!
//! A searchable select component that combines an input with a dropdown list,
//! supporting both single and multi-select modes.

use dioxus::prelude::*;
use lucide_dioxus::{Check, ChevronDown, X};

/// Context for combobox state management.
#[derive(Clone)]
pub struct ComboboxContext {
    /// Whether the combobox is open.
    pub open: Signal<bool>,
    /// The current search query.
    pub search: Signal<String>,
    /// The selected value(s).
    pub value: Signal<Vec<String>>,
    /// Whether multi-select is enabled.
    pub multiple: bool,
    /// The currently highlighted item index.
    pub highlighted_index: Signal<usize>,
    /// Total number of visible items.
    pub item_count: Signal<usize>,
    /// Callback when selection changes.
    pub on_change: Option<Callback<Vec<String>>>,
}

impl ComboboxContext {
    /// Toggle the open state.
    pub fn toggle(&mut self) {
        let current = *self.open.read();
        self.open.set(!current);
    }

    /// Close the combobox.
    pub fn close(&mut self) {
        self.open.set(false);
    }

    /// Select an item.
    pub fn select(&mut self, item_value: String) {
        if self.multiple {
            let mut current = self.value.read().clone();
            if current.contains(&item_value) {
                current.retain(|v| v != &item_value);
            } else {
                current.push(item_value);
            }
            self.value.set(current.clone());
            if let Some(callback) = &self.on_change {
                callback.call(current);
            }
        } else {
            self.value.set(vec![item_value.clone()]);
            if let Some(callback) = &self.on_change {
                callback.call(vec![item_value]);
            }
            self.close();
        }
    }

    /// Clear all selections.
    pub fn clear(&mut self) {
        self.value.set(vec![]);
        if let Some(callback) = &self.on_change {
            callback.call(vec![]);
        }
    }

    /// Remove a specific value (for multi-select).
    pub fn remove(&mut self, item_value: &str) {
        let mut current = self.value.read().clone();
        current.retain(|v| v != item_value);
        self.value.set(current.clone());
        if let Some(callback) = &self.on_change {
            callback.call(current);
        }
    }

    /// Check if a value is selected.
    pub fn is_selected(&self, item_value: &str) -> bool {
        self.value.read().contains(&item_value.to_string())
    }

    /// Move highlight up.
    pub fn highlight_previous(&mut self) {
        let current = *self.highlighted_index.read();
        if current > 0 {
            self.highlighted_index.set(current - 1);
        }
    }

    /// Move highlight down.
    pub fn highlight_next(&mut self) {
        let current = *self.highlighted_index.read();
        let count = *self.item_count.read();
        if current < count.saturating_sub(1) {
            self.highlighted_index.set(current + 1);
        }
    }

    /// Set search query.
    pub fn set_search(&mut self, query: String) {
        self.search.set(query);
        self.highlighted_index.set(0);
    }
}

/// Side for combobox content positioning.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ComboboxSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl ComboboxSide {
    fn as_str(&self) -> &'static str {
        match self {
            ComboboxSide::Top => "top",
            ComboboxSide::Bottom => "bottom",
            ComboboxSide::Left => "left",
            ComboboxSide::Right => "right",
        }
    }

    fn animation_class(&self) -> &'static str {
        match self {
            ComboboxSide::Top => "data-open:slide-in-from-bottom-2",
            ComboboxSide::Bottom => "data-open:slide-in-from-top-2",
            ComboboxSide::Left => "data-open:slide-in-from-right-2",
            ComboboxSide::Right => "data-open:slide-in-from-left-2",
        }
    }
}

/// Alignment for combobox content.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ComboboxAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Props for Combobox.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps {
    /// Controlled open state.
    #[props(default)]
    pub open: Option<Signal<bool>>,

    /// Default open state.
    #[props(default)]
    pub default_open: bool,

    /// Controlled value(s).
    #[props(default)]
    pub value: Option<Signal<Vec<String>>>,

    /// Default value(s).
    #[props(default)]
    pub default_value: Vec<String>,

    /// Whether multi-select is enabled.
    #[props(default)]
    pub multiple: bool,

    /// Callback when selection changes.
    #[props(default)]
    pub on_change: Option<Callback<Vec<String>>>,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Combobox content.
    pub children: Element,
}

/// A searchable select component.
///
/// # Example
///
/// ```rust
/// let selected = use_signal(|| vec![]);
///
/// rsx! {
///     Combobox {
///         value: selected,
///         on_change: move |v| selected.set(v),
///
///         ComboboxInput { placeholder: "Select framework..." }
///         ComboboxContent {
///             ComboboxList {
///                 ComboboxEmpty { "No framework found." }
///                 ComboboxItem { value: "react", "React" }
///                 ComboboxItem { value: "vue", "Vue" }
///                 ComboboxItem { value: "svelte", "Svelte" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let internal_open = use_signal(|| props.default_open);
    let internal_value = use_signal(|| props.default_value.clone());

    let open = props.open.unwrap_or(internal_open);
    let value = props.value.unwrap_or(internal_value);

    let context = ComboboxContext {
        open,
        search: use_signal(String::new),
        value,
        multiple: props.multiple,
        highlighted_index: use_signal(|| 0),
        item_count: use_signal(|| 0),
        on_change: props.on_change.clone(),
    };

    use_context_provider(|| context);

    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "relative {custom_class}",
            "data-slot": "combobox",
            "data-state": if *open.read() { "open" } else { "closed" },
            {props.children}
        }
    }
}

/// Props for ComboboxValue.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxValueProps {
    /// Placeholder when nothing is selected.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Displays the selected value(s).
#[component]
pub fn ComboboxValue(props: ComboboxValueProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let values = context.value.read();
    let display_text = if values.is_empty() {
        props.placeholder.clone().unwrap_or_default()
    } else {
        values.join(", ")
    };

    let is_placeholder = values.is_empty();

    let class_str = if is_placeholder {
        format!("text-muted-foreground {}", custom_class)
    } else {
        custom_class.to_string()
    };

    rsx! {
        span {
            class: class_str,
            "data-slot": "combobox-value",
            "{display_text}"
        }
    }
}

/// Props for ComboboxTrigger.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxTriggerProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Trigger content.
    #[props(default)]
    pub children: Option<Element>,
}

/// Trigger button for the combobox.
#[component]
pub fn ComboboxTrigger(props: ComboboxTriggerProps) -> Element {
    let mut context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_click = move |_| {
        context.toggle();
    };

    rsx! {
        button {
            r#type: "button",
            class: "[&_svg:not([class*='size-'])]:size-4 {custom_class}",
            "data-slot": "combobox-trigger",
            onclick: handle_click,

            if let Some(children) = props.children {
                {children}
            }

            span {
                "data-slot": "combobox-trigger-icon",
                ChevronDown {
                    class: "pointer-events-none size-4 text-muted-foreground",
                }
            }
        }
    }
}

/// Props for ComboboxClear.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxClearProps {
    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Clear button for the combobox.
#[component]
pub fn ComboboxClear(props: ComboboxClearProps) -> Element {
    let mut context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let has_value = !context.value.read().is_empty();

    if !has_value {
        return rsx! {};
    }

    let handle_click = move |event: MouseEvent| {
        event.stop_propagation();
        context.clear();
    };

    rsx! {
        button {
            r#type: "button",
            class: "inline-flex h-6 w-6 shrink-0 items-center justify-center rounded-md \
                    text-sm font-medium whitespace-nowrap transition-colors outline-none \
                    hover:bg-accent hover:text-accent-foreground \
                    disabled:pointer-events-none disabled:opacity-50 \
                    {custom_class}",
            "data-slot": "combobox-clear",
            disabled: props.disabled,
            onclick: handle_click,

            X { class: "pointer-events-none size-4" }
        }
    }
}

/// Props for ComboboxInput.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether to show the trigger button.
    #[props(default = true)]
    pub show_trigger: bool,

    /// Whether to show the clear button.
    #[props(default)]
    pub show_clear: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Input field for the combobox.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    // Clone context for closures before reading from it
    let search_value = context.search.read().clone();

    let handle_input = {
        let mut context = context.clone();
        move |event: FormEvent| {
            context.set_search(event.value());
        }
    };

    let handle_focus = {
        let mut context = context.clone();
        move |_| {
            context.open.set(true);
        }
    };

    let handle_keydown = {
        let mut context = context.clone();
        move |event: KeyboardEvent| {
            match event.key() {
                Key::ArrowUp => {
                    event.prevent_default();
                    context.highlight_previous();
                }
                Key::ArrowDown => {
                    event.prevent_default();
                    if !*context.open.read() {
                        context.open.set(true);
                    } else {
                        context.highlight_next();
                    }
                }
                Key::Escape => {
                    context.close();
                }
                Key::Enter => {
                    // Selection handled by highlighted item
                }
                _ => {}
            }
        }
    };

    let handle_trigger_click = {
        let mut context = context.clone();
        move |_| {
            context.toggle();
        }
    };

    rsx! {
        div {
            class: "group/input-group relative flex w-full items-center rounded-md border border-input \
                    bg-transparent shadow-xs transition-[color,box-shadow] \
                    has-[input:focus]:border-ring has-[input:focus]:ring-[3px] has-[input:focus]:ring-ring/50 \
                    aria-invalid:border-destructive aria-invalid:ring-destructive/20 \
                    has-[input:disabled]:cursor-not-allowed has-[input:disabled]:opacity-50 \
                    dark:bg-input/30 \
                    {custom_class}",
            "data-slot": "input-group",

            input {
                r#type: "text",
                class: "flex-1 border-0 bg-transparent px-3 py-2 text-sm outline-none \
                        placeholder:text-muted-foreground disabled:cursor-not-allowed",
                "data-slot": "combobox-input",
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                value: search_value,
                oninput: handle_input,
                onfocus: handle_focus,
                onkeydown: handle_keydown,
            }

            div {
                class: "flex items-center px-1",
                "data-slot": "input-group-addon",

                if props.show_clear {
                    ComboboxClear { disabled: props.disabled }
                }

                if props.show_trigger {
                    button {
                        r#type: "button",
                        class: "inline-flex h-6 w-6 shrink-0 items-center justify-center \
                                rounded-md text-sm font-medium whitespace-nowrap transition-colors \
                                outline-none hover:bg-accent hover:text-accent-foreground \
                                disabled:pointer-events-none disabled:opacity-50 \
                                group-has-[data-slot=combobox-clear]/input-group:hidden",
                        "data-slot": "input-group-button",
                        disabled: props.disabled,
                        onclick: handle_trigger_click,

                        ChevronDown {
                            class: "pointer-events-none size-4 text-muted-foreground",
                        }
                    }
                }
            }
        }
    }
}

/// Props for ComboboxContent.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxContentProps {
    /// Side for positioning.
    #[props(default)]
    pub side: ComboboxSide,

    /// Side offset in pixels.
    #[props(default = 6)]
    pub side_offset: i32,

    /// Alignment.
    #[props(default)]
    pub align: ComboboxAlign,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Content children.
    pub children: Element,
}

/// Dropdown content for the combobox.
#[component]
pub fn ComboboxContent(props: ComboboxContentProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let is_open = *context.open.read();

    if !is_open {
        return rsx! {};
    }

    let animation_class = props.side.animation_class();

    let position_class = match props.side {
        ComboboxSide::Top => format!("bottom-full mb-{}px", props.side_offset),
        ComboboxSide::Bottom => format!("top-full mt-{}px", props.side_offset),
        ComboboxSide::Left => format!("right-full mr-{}px", props.side_offset),
        ComboboxSide::Right => format!("left-full ml-{}px", props.side_offset),
    };

    let align_class = match props.align {
        ComboboxAlign::Start => "left-0",
        ComboboxAlign::Center => "left-1/2 -translate-x-1/2",
        ComboboxAlign::End => "right-0",
    };

    let classes = format!(
        "group/combobox-content absolute z-50 w-full min-w-[8rem] max-h-96 \
         overflow-hidden rounded-md bg-popover text-popover-foreground shadow-md \
         ring-1 ring-foreground/10 duration-100 \
         animate-in fade-in-0 zoom-in-95 \
         {} {} {} {}",
        position_class, align_class, animation_class, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "combobox-content",
            "data-state": "open",
            "data-side": props.side.as_str(),
            {props.children}
        }
    }
}

/// Props for ComboboxList.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// List content.
    pub children: Element,
}

/// Scrollable list container for combobox items.
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "max-h-80 scroll-py-1 overflow-y-auto p-1 data-empty:p-0 \
         {}",
        custom_class
    );

    rsx! {
        div {
            role: "listbox",
            class: classes,
            "data-slot": "combobox-list",
            {props.children}
        }
    }
}

/// Props for ComboboxItem.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemProps {
    /// The value of this item.
    pub value: String,

    /// Display label (defaults to value if not provided).
    #[props(default)]
    pub label: Option<String>,

    /// Keywords for filtering.
    #[props(default)]
    pub keywords: Option<Vec<String>>,

    /// Whether the item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A selectable item in the combobox.
#[component]
pub fn ComboboxItem(props: ComboboxItemProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let mut item_index = use_signal(|| 0_usize);
    let custom_class = props.class.as_deref().unwrap_or("");

    // Check if this item matches the search
    let search = context.search.read().to_lowercase();
    let value_lower = props.value.to_lowercase();
    let label_lower = props
        .label
        .as_ref()
        .map(|l| l.to_lowercase())
        .unwrap_or_default();
    let keywords = props.keywords.clone().unwrap_or_default();

    let matches = search.is_empty()
        || value_lower.contains(&search)
        || label_lower.contains(&search)
        || keywords.iter().any(|k| k.to_lowercase().contains(&search));

    if !matches {
        return rsx! {};
    }

    // Register this item
    use_effect({
        let mut item_count = context.item_count;
        move || {
            let current = *item_count.read();
            item_index.set(current);
            item_count.set(current + 1);
        }
    });

    let is_highlighted = *context.highlighted_index.read() == *item_index.read();
    let is_selected = context.is_selected(&props.value);

    let classes = format!(
        "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm \
         outline-hidden select-none \
         data-[highlighted=true]:bg-accent data-[highlighted=true]:text-accent-foreground \
         data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {}",
        custom_class
    );

    let handle_click = {
        let value = props.value.clone();
        let disabled = props.disabled;
        let mut context = context.clone();
        move |_| {
            if !disabled {
                context.select(value.clone());
            }
        }
    };

    rsx! {
        div {
            role: "option",
            class: classes,
            "data-slot": "combobox-item",
            "data-value": props.value.clone(),
            "data-highlighted": is_highlighted.to_string(),
            "data-disabled": props.disabled.to_string(),
            aria_selected: is_selected.to_string(),
            aria_disabled: props.disabled.to_string(),
            onclick: handle_click,

            {props.children}

            if is_selected {
                span {
                    class: "pointer-events-none absolute right-2 flex size-4 items-center justify-center",
                    "data-slot": "combobox-item-indicator",
                    Check { class: "pointer-events-none size-4" }
                }
            }
        }
    }
}

/// Props for ComboboxGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group content.
    pub children: Element,
}

/// A group of related combobox items.
#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            role: "group",
            class: custom_class,
            "data-slot": "combobox-group",
            {props.children}
        }
    }
}

/// Props for ComboboxLabel.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxLabelProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Label content.
    pub children: Element,
}

/// A label for a combobox group.
#[component]
pub fn ComboboxLabel(props: ComboboxLabelProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "px-2 py-1.5 text-xs text-muted-foreground \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "combobox-label",
            {props.children}
        }
    }
}

/// Props for ComboboxEmpty.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEmptyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Empty state content.
    pub children: Element,
}

/// Shown when no items match the search.
#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let item_count = *context.item_count.read();

    // Only show when there are no items
    if item_count > 0 {
        return rsx! {};
    }

    let classes = format!(
        "flex w-full justify-center py-6 text-center text-sm text-muted-foreground \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "combobox-empty",
            {props.children}
        }
    }
}

/// Props for ComboboxSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between combobox items or groups.
#[component]
pub fn ComboboxSeparator(props: ComboboxSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("-mx-1 my-1 h-px bg-border {}", custom_class);

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "combobox-separator",
        }
    }
}

/// Props for ComboboxChips.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxChipsProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Chips content.
    pub children: Element,
}

/// Container for multi-select chips.
#[component]
pub fn ComboboxChips(props: ComboboxChipsProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex min-h-9 flex-wrap items-center gap-1.5 rounded-md border border-input \
         bg-transparent bg-clip-padding px-2.5 py-1.5 text-sm shadow-xs \
         transition-[color,box-shadow] \
         focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50 \
         has-aria-invalid:border-destructive has-aria-invalid:ring-[3px] has-aria-invalid:ring-destructive/20 \
         has-[data-slot=combobox-chip]:px-1.5 \
         dark:bg-input/30 dark:has-aria-invalid:border-destructive/50 dark:has-aria-invalid:ring-destructive/40 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "combobox-chips",
            {props.children}
        }
    }
}

/// Props for ComboboxChip.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxChipProps {
    /// The value of this chip.
    pub value: String,

    /// Whether to show the remove button.
    #[props(default = true)]
    pub show_remove: bool,

    /// Whether the chip is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Chip content.
    pub children: Element,
}

/// A chip representing a selected item in multi-select mode.
#[component]
pub fn ComboboxChip(props: ComboboxChipProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_remove = {
        let value = props.value.clone();
        let mut context = context.clone();
        move |event: MouseEvent| {
            event.stop_propagation();
            context.remove(&value);
        }
    };

    let classes = format!(
        "flex h-5.5 w-fit items-center justify-center gap-1 rounded-sm bg-muted px-1.5 \
         text-xs font-medium whitespace-nowrap text-foreground \
         has-[button:disabled]:pointer-events-none has-[button:disabled]:cursor-not-allowed \
         has-[button:disabled]:opacity-50 \
         has-[data-slot=combobox-chip-remove]:pr-0 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "combobox-chip",
            "data-value": props.value.clone(),

            {props.children}

            if props.show_remove {
                button {
                    r#type: "button",
                    class: "inline-flex h-5.5 w-5.5 shrink-0 items-center justify-center \
                            rounded-sm text-sm font-medium whitespace-nowrap transition-colors \
                            outline-none hover:bg-accent hover:text-accent-foreground \
                            -ml-1 opacity-50 hover:opacity-100 \
                            disabled:pointer-events-none disabled:opacity-50",
                    "data-slot": "combobox-chip-remove",
                    disabled: props.disabled,
                    onclick: handle_remove,

                    X { class: "pointer-events-none size-3" }
                }
            }
        }
    }
}

/// Props for ComboboxChipsInput.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxChipsInputProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Input field within a chips container.
#[component]
pub fn ComboboxChipsInput(props: ComboboxChipsInputProps) -> Element {
    let context = use_context::<ComboboxContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_input = {
        let mut context = context.clone();
        move |event: FormEvent| {
            context.set_search(event.value());
        }
    };

    let handle_focus = {
        let mut context = context.clone();
        move |_| {
            context.open.set(true);
        }
    };

    let handle_keydown = {
        let mut context = context.clone();
        move |event: KeyboardEvent| {
            match event.key() {
                Key::ArrowUp => {
                    event.prevent_default();
                    context.highlight_previous();
                }
                Key::ArrowDown => {
                    event.prevent_default();
                    if !*context.open.read() {
                        context.open.set(true);
                    } else {
                        context.highlight_next();
                    }
                }
                Key::Escape => {
                    context.close();
                }
                Key::Backspace if context.search.read().is_empty() => {
                    // Remove last chip if input is empty
                    let values = context.value.read().clone();
                    if let Some(last) = values.last() {
                        context.remove(last);
                    }
                }
                Key::Backspace => {}
                _ => {}
            }
        }
    };

    let classes = format!("min-w-16 flex-1 outline-none {}", custom_class);

    rsx! {
        input {
            r#type: "text",
            class: classes,
            "data-slot": "combobox-chips-input",
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            value: context.search.read().clone(),
            oninput: handle_input,
            onfocus: handle_focus,
            onkeydown: handle_keydown,
        }
    }
}

/// Hook to access the combobox context.
pub fn use_combobox() -> ComboboxContext {
    use_context::<ComboboxContext>()
}
