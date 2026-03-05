//! InputGroup component for inputs with addons.
//!
//! A composable component system for creating input groups with
//! integrated prefixes, suffixes, buttons, and text elements.

use dioxus::prelude::*;

/// Alignment position for addons.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum InputGroupAlign {
    /// Start of inline axis (left in LTR)
    #[default]
    InlineStart,
    /// End of inline axis (right in LTR)
    InlineEnd,
    /// Start of block axis (top)
    BlockStart,
    /// End of block axis (bottom)
    BlockEnd,
}

impl InputGroupAlign {
    fn as_str(&self) -> &'static str {
        match self {
            InputGroupAlign::InlineStart => "inline-start",
            InputGroupAlign::InlineEnd => "inline-end",
            InputGroupAlign::BlockStart => "block-start",
            InputGroupAlign::BlockEnd => "block-end",
        }
    }
}

/// Props for InputGroup.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupProps {
    /// Whether the group has an error state.
    #[props(default)]
    pub invalid: bool,

    /// Whether the group is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group content.
    pub children: Element,
}

/// A container for grouping inputs with addons.
///
/// # Example
///
/// ```rust
/// rsx! {
///     InputGroup {
///         InputGroupText { "$" }
///         InputGroupInput { placeholder: "Amount" }
///         InputGroupText { ".00" }
///     }
/// }
/// ```
#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "group/input-group relative flex w-full items-center rounded-md border border-input \
         bg-transparent shadow-xs transition-[color,box-shadow] \
         has-[input:focus]:border-ring has-[input:focus]:ring-[3px] has-[input:focus]:ring-ring/50 \
         has-[textarea:focus]:border-ring has-[textarea:focus]:ring-[3px] has-[textarea:focus]:ring-ring/50 \
         aria-invalid:border-destructive aria-invalid:ring-destructive/20 \
         dark:aria-invalid:ring-destructive/40 \
         has-[input:disabled]:cursor-not-allowed has-[input:disabled]:opacity-50 \
         dark:bg-input/30 \
         has-[*[data-align=block-start]]:flex-col has-[*[data-align=block-end]]:flex-col \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "input-group",
            "aria-invalid": props.invalid.to_string(),
            "aria-disabled": props.disabled.to_string(),
            {props.children}
        }
    }
}

/// Props for InputGroupAddon.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupAddonProps {
    /// Alignment position.
    #[props(default)]
    pub align: InputGroupAlign,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Addon content.
    pub children: Element,
}

/// A wrapper for prefix/suffix content in an input group.
#[component]
pub fn InputGroupAddon(props: InputGroupAddonProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let align_class = match props.align {
        InputGroupAlign::InlineStart => "border-r",
        InputGroupAlign::InlineEnd => "border-l",
        InputGroupAlign::BlockStart => "border-b w-full",
        InputGroupAlign::BlockEnd => "border-t w-full",
    };

    let classes = format!(
        "flex items-center justify-center border-input bg-muted px-3 text-sm text-muted-foreground \
         [&_svg:not([class*='size-'])]:size-4 \
         {} {}",
        align_class, custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "input-group-addon",
            "data-align": props.align.as_str(),
            {props.children}
        }
    }
}

/// Size variants for InputGroupButton.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum InputGroupButtonSize {
    Xs,
    #[default]
    Sm,
    IconXs,
    IconSm,
}

/// Props for InputGroupButton.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupButtonProps {
    /// Size variant.
    #[props(default)]
    pub size: InputGroupButtonSize,

    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Click handler.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Button content.
    pub children: Element,
}

/// A button within an input group.
#[component]
pub fn InputGroupButton(props: InputGroupButtonProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let size_class = match props.size {
        InputGroupButtonSize::Xs => "h-6 px-2 text-xs",
        InputGroupButtonSize::Sm => "h-8 px-3 text-sm",
        InputGroupButtonSize::IconXs => "h-6 w-6",
        InputGroupButtonSize::IconSm => "h-8 w-8",
    };

    let classes = format!(
        "inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium \
         whitespace-nowrap transition-colors outline-none \
         hover:bg-accent hover:text-accent-foreground \
         focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         disabled:pointer-events-none disabled:opacity-50 \
         [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0 \
         {} {}",
        size_class, custom_class
    );

    let handle_click = move |event: MouseEvent| {
        if let Some(handler) = &props.on_click {
            handler.call(event);
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: classes,
            "data-slot": "input-group-button",
            disabled: props.disabled,
            onclick: handle_click,
            {props.children}
        }
    }
}

/// Props for InputGroupText.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Text/icon content.
    pub children: Element,
}

/// A text or icon element within an input group.
#[component]
pub fn InputGroupText(props: InputGroupTextProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex items-center px-3 text-sm text-muted-foreground \
         [&_svg:not([class*='size-'])]:size-4 \
         {}",
        custom_class
    );

    rsx! {
        span {
            class: classes,
            "data-slot": "input-group-text",
            {props.children}
        }
    }
}

/// Props for InputGroupInput.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupInputProps {
    /// Input type.
    #[props(default = "text".to_string())]
    pub r#type: String,

    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Input value.
    #[props(default)]
    pub value: Option<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the input is read-only.
    #[props(default)]
    pub readonly: bool,

    /// Input name.
    #[props(default)]
    pub name: Option<String>,

    /// Input id.
    #[props(default)]
    pub id: Option<String>,

    /// Change handler.
    #[props(default)]
    pub on_input: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// An input field within an input group.
#[component]
pub fn InputGroupInput(props: InputGroupInputProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex-1 border-0 bg-transparent px-3 py-2 text-sm outline-none \
         file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground \
         placeholder:text-muted-foreground \
         disabled:cursor-not-allowed \
         {}",
        custom_class
    );

    let handle_input = move |event: FormEvent| {
        if let Some(handler) = &props.on_input {
            handler.call(event);
        }
    };

    rsx! {
        input {
            r#type: props.r#type.clone(),
            class: classes,
            "data-slot": "input-group-input",
            placeholder: props.placeholder.clone(),
            value: props.value.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            name: props.name.clone(),
            id: props.id.clone(),
            oninput: handle_input,
        }
    }
}

/// Props for InputGroupTextarea.
#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextareaProps {
    /// Placeholder text.
    #[props(default)]
    pub placeholder: Option<String>,

    /// Textarea value.
    #[props(default)]
    pub value: Option<String>,

    /// Whether the textarea is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Whether the textarea is read-only.
    #[props(default)]
    pub readonly: bool,

    /// Number of rows.
    #[props(default = 3)]
    pub rows: u32,

    /// Textarea name.
    #[props(default)]
    pub name: Option<String>,

    /// Textarea id.
    #[props(default)]
    pub id: Option<String>,

    /// Change handler.
    #[props(default)]
    pub on_input: Option<EventHandler<FormEvent>>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A textarea within an input group.
#[component]
pub fn InputGroupTextarea(props: InputGroupTextareaProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex-1 resize-none border-0 bg-transparent px-3 py-2 text-sm outline-none \
         placeholder:text-muted-foreground \
         disabled:cursor-not-allowed \
         {}",
        custom_class
    );

    let handle_input = move |event: FormEvent| {
        if let Some(handler) = &props.on_input {
            handler.call(event);
        }
    };

    rsx! {
        textarea {
            class: classes,
            "data-slot": "input-group-textarea",
            placeholder: props.placeholder.clone(),
            value: props.value.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            rows: props.rows.to_string(),
            name: props.name.clone(),
            id: props.id.clone(),
            oninput: handle_input,
        }
    }
}
