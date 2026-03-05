//! Command component for command palettes.
//!
//! A searchable command menu with keyboard navigation, similar to VS Code's
//! command palette or macOS Spotlight.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing command state.
#[derive(Clone)]
pub struct CommandContext {
    /// The current search query.
    pub search: Signal<String>,
    /// The currently selected item index.
    pub selected_index: Signal<usize>,
    /// Total number of visible items.
    pub item_count: Signal<usize>,
    /// Callback when an item is selected.
    pub on_select: Option<Callback<String>>,
}

impl CommandContext {
    /// Update the search query.
    pub fn set_search(&mut self, query: String) {
        self.search.set(query);
        // Reset selection when search changes
        self.selected_index.set(0);
    }

    /// Move selection up.
    pub fn select_previous(&mut self) {
        let current = *self.selected_index.read();
        if current > 0 {
            self.selected_index.set(current - 1);
        }
    }

    /// Move selection down.
    pub fn select_next(&mut self) {
        let current = *self.selected_index.read();
        let count = *self.item_count.read();
        if current < count.saturating_sub(1) {
            self.selected_index.set(current + 1);
        }
    }
}

/// Props for Command.
#[derive(Props, Clone, PartialEq)]
pub struct CommandProps {
    /// Callback when an item is selected.
    #[props(default)]
    pub on_select: Option<Callback<String>>,

    /// Whether to show the search input.
    #[props(default = true)]
    pub show_input: bool,

    /// Placeholder text for search input.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Command content.
    pub children: Element,
}

/// A command palette component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Command {
///         on_select: move |value| {
///             // Handle selection
///         },
///
///         CommandInput { placeholder: "Type a command or search..." }
///         CommandList {
///             CommandEmpty { "No results found." }
///             CommandGroup { heading: "Suggestions",
///                 CommandItem { value: "calendar", "Calendar" }
///                 CommandItem { value: "search", "Search Emoji" }
///                 CommandItem { value: "calculator", "Calculator" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Command(props: CommandProps) -> Element {
    let search = use_signal(String::new);
    let selected_index = use_signal(|| 0_usize);
    let item_count = use_signal(|| 0_usize);

    let context = CommandContext {
        search,
        selected_index,
        item_count,
        on_select: props.on_select.clone(),
    };

    use_context_provider(|| context.clone());

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground \
         [&_[data-slot=command-group]:not([hidden])_~[data-slot=command-group]]:pt-0 \
         [&_[data-slot=command-group]]:px-2 {}",
        custom_class
    );

    let handle_keydown = {
        let mut context = context.clone();
        move |event: KeyboardEvent| {
            match event.key() {
                Key::ArrowUp => {
                    event.prevent_default();
                    context.select_previous();
                }
                Key::ArrowDown => {
                    event.prevent_default();
                    context.select_next();
                }
                _ => {}
            }
        }
    };

    rsx! {
        div {
            class: classes,
            "data-slot": "command",
            onkeydown: handle_keydown,
            {props.children}
        }
    }
}

/// Props for CommandDialog.
#[derive(Props, Clone, PartialEq)]
pub struct CommandDialogProps {
    /// Whether the dialog is open.
    pub open: Signal<bool>,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    /// Dialog title (screen reader only).
    #[props(default)]
    pub title: Option<String>,

    /// Dialog description (screen reader only).
    #[props(default)]
    pub description: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Command content.
    pub children: Element,
}

/// A command palette in a dialog.
#[component]
pub fn CommandDialog(props: CommandDialogProps) -> Element {
    let dialog_id = use_unique_id();
    let custom_class = props.class.as_deref().unwrap_or("");

    if !*props.open.read() {
        return rsx! {};
    }

    let title = props.title.clone().unwrap_or_else(|| "Command Palette".to_string());
    let description = props.description.clone().unwrap_or_else(|| "Search for a command to run...".to_string());

    let handle_close = {
        let mut open = props.open;
        let on_open_change = props.on_open_change.clone();
        move |_| {
            open.set(false);
            if let Some(callback) = &on_open_change {
                callback.call(false);
            }
        }
    };

    let handle_keydown = {
        let mut open = props.open;
        let on_open_change = props.on_open_change.clone();
        move |event: KeyboardEvent| {
            if event.key() == Key::Escape {
                open.set(false);
                if let Some(callback) = &on_open_change {
                    callback.call(false);
                }
            }
        }
    };

    let classes = format!(
        "fixed top-1/2 left-1/2 z-50 w-full max-w-lg -translate-x-1/2 -translate-y-1/2 \
         overflow-hidden rounded-lg border bg-popover shadow-lg animate-in fade-in-0 zoom-in-95 \
         {}",
        custom_class
    );

    rsx! {
        // Overlay
        div {
            class: "fixed inset-0 z-50 bg-black/50 animate-in fade-in-0",
            onclick: handle_close,
        }

        // Dialog
        div {
            role: "dialog",
            id: dialog_id(),
            class: classes,
            "data-slot": "command-dialog",
            "data-state": "open",
            aria_modal: "true",
            aria_labelledby: format!("{}-title", dialog_id()),
            aria_describedby: format!("{}-description", dialog_id()),
            onkeydown: handle_keydown,

            // Screen reader only title/description
            h2 {
                id: format!("{}-title", dialog_id()),
                class: "sr-only",
                "{title}"
            }
            p {
                id: format!("{}-description", dialog_id()),
                class: "sr-only",
                "{description}"
            }

            Command {
                {props.children}
            }
        }
    }
}

/// Props for CommandInput.
#[derive(Props, Clone, PartialEq)]
pub struct CommandInputProps {
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

/// The search input for the command palette.
#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let context = use_context::<CommandContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let handle_input = {
        let mut context = context.clone();
        move |event: FormEvent| {
            context.set_search(event.value());
        }
    };

    let classes = format!(
        "flex h-10 w-full bg-transparent text-sm outline-none \
         placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: "flex h-9 items-center gap-2 border-b px-3",
            "data-slot": "command-input-wrapper",

            // Search icon
            svg {
                class: "size-4 shrink-0 opacity-50",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "m21 21-4.3-4.3" }
            }

            input {
                r#type: "text",
                class: classes,
                "data-slot": "command-input",
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                value: context.search.read().clone(),
                oninput: handle_input,
                autofocus: true,
            }
        }
    }
}

/// Props for CommandList.
#[derive(Props, Clone, PartialEq)]
pub struct CommandListProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// List content.
    pub children: Element,
}

/// The scrollable list of command items.
#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "max-h-[300px] scroll-py-1 overflow-x-hidden overflow-y-auto \
         [&_[data-slot=command-group-heading]]:px-2 [&_[data-slot=command-group-heading]]:font-medium \
         [&_[data-slot=command-group-heading]]:text-muted-foreground {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "command-list",
            role: "listbox",
            {props.children}
        }
    }
}

/// Props for CommandEmpty.
#[derive(Props, Clone, PartialEq)]
pub struct CommandEmptyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Empty state content.
    pub children: Element,
}

/// Shown when no results match the search.
#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    let context = use_context::<CommandContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    // Only show when there's a search query and no items
    let search = context.search.read().clone();
    let item_count = *context.item_count.read();

    if search.is_empty() || item_count > 0 {
        return rsx! {};
    }

    let classes = format!("py-6 text-center text-sm {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "command-empty",
            {props.children}
        }
    }
}

/// Props for CommandGroup.
#[derive(Props, Clone, PartialEq)]
pub struct CommandGroupProps {
    /// Group heading.
    #[props(default)]
    pub heading: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group content.
    pub children: Element,
}

/// A group of related command items.
#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("overflow-hidden p-1 text-foreground [&_[data-slot=command-group-heading]]:py-1.5 {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "command-group",
            role: "group",

            if let Some(heading) = &props.heading {
                div {
                    class: "px-2 py-1.5 text-xs font-medium text-muted-foreground",
                    "data-slot": "command-group-heading",
                    "{heading}"
                }
            }

            {props.children}
        }
    }
}

/// Props for CommandItem.
#[derive(Props, Clone, PartialEq)]
pub struct CommandItemProps {
    /// The value of this item (used for selection).
    pub value: String,

    /// Keywords for filtering (in addition to children text).
    #[props(default)]
    pub keywords: Option<Vec<String>>,

    /// Whether this item is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Callback when this item is selected.
    #[props(default)]
    pub on_select: Option<Callback<String>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Item content.
    pub children: Element,
}

/// A selectable command item.
#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let context = use_context::<CommandContext>();
    let mut item_index = use_signal(|| 0_usize);
    let custom_class = props.class.as_deref().unwrap_or("");

    // Check if this item matches the search
    let search = context.search.read().to_lowercase();
    let value_lower = props.value.to_lowercase();
    let keywords = props.keywords.clone().unwrap_or_default();

    let matches = search.is_empty()
        || value_lower.contains(&search)
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

    let is_selected = *context.selected_index.read() == *item_index.read();

    let classes = format!(
        "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm \
         outline-hidden select-none \
         data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 \
         data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
         {}",
        custom_class
    );

    let handle_click = {
        let value = props.value.clone();
        let on_select = props.on_select.clone();
        let context_on_select = context.on_select.clone();
        let disabled = props.disabled;
        move |_| {
            if !disabled {
                if let Some(callback) = &on_select {
                    callback.call(value.clone());
                }
                if let Some(callback) = &context_on_select {
                    callback.call(value.clone());
                }
            }
        }
    };

    let handle_keydown = {
        let value = props.value.clone();
        let on_select = props.on_select.clone();
        let context_on_select = context.on_select.clone();
        let disabled = props.disabled;
        move |event: KeyboardEvent| {
            if !disabled && event.key() == Key::Enter {
                event.prevent_default();
                if let Some(callback) = &on_select {
                    callback.call(value.clone());
                }
                if let Some(callback) = &context_on_select {
                    callback.call(value.clone());
                }
            }
        }
    };

    rsx! {
        div {
            role: "option",
            class: classes,
            "data-slot": "command-item",
            "data-value": props.value.clone(),
            "data-selected": is_selected.to_string(),
            "data-disabled": props.disabled.to_string(),
            aria_selected: is_selected.to_string(),
            aria_disabled: props.disabled.to_string(),
            tabindex: if props.disabled { "-1" } else { "0" },
            onclick: handle_click,
            onkeydown: handle_keydown,

            {props.children}
        }
    }
}

/// Props for CommandSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct CommandSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between command groups or items.
#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("-mx-1 h-px bg-border {}", custom_class);

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "command-separator",
        }
    }
}

/// Props for CommandShortcut.
#[derive(Props, Clone, PartialEq)]
pub struct CommandShortcutProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Shortcut content.
    pub children: Element,
}

/// Displays a keyboard shortcut for a command item.
#[component]
pub fn CommandShortcut(props: CommandShortcutProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("ml-auto text-xs tracking-widest text-muted-foreground {}", custom_class);

    rsx! {
        span {
            class: classes,
            "data-slot": "command-shortcut",
            {props.children}
        }
    }
}

/// Hook to access the command context.
pub fn use_command() -> CommandContext {
    use_context::<CommandContext>()
}
