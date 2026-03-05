//! InputOTP component for one-time password input.
//!
//! A component for entering OTP codes with individual character slots.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Context for managing OTP input state.
#[derive(Clone)]
pub struct InputOTPContext {
    /// The current OTP value.
    pub value: Signal<String>,
    /// Maximum length of the OTP.
    pub max_length: usize,
    /// Whether the input is disabled.
    pub disabled: bool,
    /// The currently focused slot index.
    pub focused_index: Signal<Option<usize>>,
    /// Callback when value changes.
    pub on_change: Option<Callback<String>>,
    /// Callback when OTP is complete.
    pub on_complete: Option<Callback<String>>,
}

impl InputOTPContext {
    /// Get character at a specific index.
    pub fn char_at(&self, index: usize) -> Option<char> {
        self.value.read().chars().nth(index)
    }

    /// Set the value.
    pub fn set_value(&mut self, value: String) {
        let truncated = value.chars().take(self.max_length).collect::<String>();
        self.value.set(truncated.clone());

        if let Some(callback) = &self.on_change {
            callback.call(truncated.clone());
        }

        if truncated.len() == self.max_length {
            if let Some(callback) = &self.on_complete {
                callback.call(truncated);
            }
        }
    }

    /// Insert a character at the current position.
    pub fn insert_char(&mut self, ch: char) {
        let mut current = self.value.read().clone();
        if current.len() < self.max_length {
            current.push(ch);
            self.set_value(current);
        }
    }

    /// Delete the last character.
    pub fn delete_char(&mut self) {
        let mut current = self.value.read().clone();
        current.pop();
        self.set_value(current);
    }

    /// Check if the OTP is complete.
    pub fn is_complete(&self) -> bool {
        self.value.read().len() == self.max_length
    }
}

/// Props for InputOTP.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPProps {
    /// Controlled value.
    #[props(default)]
    pub value: Option<Signal<String>>,

    /// Default value for uncontrolled mode.
    #[props(default)]
    pub default_value: Option<String>,

    /// Maximum number of characters.
    #[props(default = 6)]
    pub max_length: usize,

    /// Callback when value changes.
    #[props(default)]
    pub on_change: Option<Callback<String>>,

    /// Callback when OTP is complete.
    #[props(default)]
    pub on_complete: Option<Callback<String>>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Pattern for input validation (e.g., "^[0-9]+$" for digits only).
    #[props(default)]
    pub pattern: Option<String>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// OTP slots.
    pub children: Element,
}

/// A one-time password input component.
///
/// # Example
///
/// ```rust
/// let otp = use_signal(String::new);
///
/// rsx! {
///     InputOTP {
///         value: otp,
///         max_length: 6,
///         on_complete: move |code| {
///             // Verify OTP
///         },
///
///         InputOTPGroup {
///             InputOTPSlot { index: 0 }
///             InputOTPSlot { index: 1 }
///             InputOTPSlot { index: 2 }
///         }
///         InputOTPSeparator {}
///         InputOTPGroup {
///             InputOTPSlot { index: 3 }
///             InputOTPSlot { index: 4 }
///             InputOTPSlot { index: 5 }
///         }
///     }
/// }
/// ```
#[component]
pub fn InputOTP(props: InputOTPProps) -> Element {
    let otp_id = use_unique_id();

    // Internal state for uncontrolled mode
    let internal_value = use_signal(|| props.default_value.clone().unwrap_or_default());

    // Use controlled or internal state
    let value = props.value.unwrap_or(internal_value);
    let focused_index = use_signal(|| None::<usize>);

    let context = InputOTPContext {
        value,
        max_length: props.max_length,
        disabled: props.disabled,
        focused_index,
        on_change: props.on_change.clone(),
        on_complete: props.on_complete.clone(),
    };

    use_context_provider(|| context.clone());

    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "flex items-center gap-2 has-[:disabled]:opacity-50 {}",
        custom_class
    );

    let pattern = props.pattern.clone();

    let handle_keydown = {
        let mut context = context.clone();
        let _pattern = pattern.clone();
        move |event: KeyboardEvent| {
            if context.disabled {
                return;
            }

            match event.key() {
                Key::Backspace => {
                    event.prevent_default();
                    context.delete_char();
                }
                Key::Character(ref s) if s.len() == 1 => {
                    let ch = s.chars().next().unwrap();

                    // Check pattern if provided
                    if let Some(ref pat) = _pattern {
                        // Simple pattern check - just check if it's digits for now
                        if pat.contains("0-9") && !ch.is_ascii_digit() {
                            return;
                        }
                    }

                    event.prevent_default();
                    context.insert_char(ch);
                }
                _ => {}
            }
        }
    };

    let handle_paste = {
        let context = context.clone();
        let _pattern = pattern.clone();
        move |event: ClipboardEvent| {
            event.prevent_default();

            if context.disabled {
                return;
            }

            // Get pasted text - for now we'll handle this through the input event
            // In a real implementation, we'd extract clipboard data here
        }
    };

    rsx! {
        div {
            class: classes,
            "data-slot": "input-otp",
            "data-disabled": props.disabled.to_string(),

            // Hidden input for actual value management
            input {
                r#type: "text",
                id: otp_id(),
                class: "sr-only",
                value: value.read().clone(),
                maxlength: props.max_length.to_string(),
                disabled: props.disabled,
                autocomplete: "one-time-code",
                inputmode: "numeric",
                pattern: props.pattern.clone(),
            }

            // Visual slots container
            div {
                class: "flex items-center gap-2",
                role: "group",
                "data-slot": "input-otp-container",
                tabindex: if props.disabled { "-1" } else { "0" },
                onkeydown: handle_keydown,
                onpaste: handle_paste,

                {props.children}
            }
        }
    }
}

/// Props for InputOTPGroup.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPGroupProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Group slots.
    pub children: Element,
}

/// A group of OTP slots.
#[component]
pub fn InputOTPGroup(props: InputOTPGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex items-center {}", custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "input-otp-group",
            {props.children}
        }
    }
}

/// Props for InputOTPSlot.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSlotProps {
    /// The index of this slot (0-based).
    pub index: usize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A single character slot in the OTP input.
#[component]
pub fn InputOTPSlot(props: InputOTPSlotProps) -> Element {
    let context = use_context::<InputOTPContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    let char_value = context.char_at(props.index);
    let is_active = context.value.read().len() == props.index;
    let has_value = char_value.is_some();

    // Determine border styling based on position
    let _border_class = if props.index == 0 {
        "rounded-l-md border-l"
    } else {
        "border-l-0"
    };

    let classes = format!(
        "relative flex h-9 w-9 items-center justify-center border-y border-r border-input \
         text-sm shadow-xs transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md \
         aria-invalid:border-destructive \
         data-[active=true]:z-10 data-[active=true]:border-ring data-[active=true]:ring-[3px] data-[active=true]:ring-ring/50 \
         data-[active=true]:aria-invalid:border-destructive data-[active=true]:aria-invalid:ring-destructive/20 \
         dark:bg-input/30 dark:data-[active=true]:aria-invalid:ring-destructive/40 \
         {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "input-otp-slot",
            "data-active": is_active.to_string(),
            "data-filled": has_value.to_string(),

            if let Some(ch) = char_value {
                span { "{ch}" }
            } else if is_active {
                // Fake caret
                div {
                    class: "pointer-events-none absolute inset-0 flex items-center justify-center",
                    "data-slot": "input-otp-caret",

                    div {
                        class: "h-4 w-px animate-caret-blink bg-foreground duration-1000",
                    }
                }
            }
        }
    }
}

/// Props for InputOTPSeparator.
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPSeparatorProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A separator between OTP groups.
#[component]
pub fn InputOTPSeparator(props: InputOTPSeparatorProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("flex items-center {}", custom_class);

    rsx! {
        div {
            role: "separator",
            class: classes,
            "data-slot": "input-otp-separator",

            // Minus icon
            svg {
                class: "size-4 text-muted-foreground",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                line { x1: "5", y1: "12", x2: "19", y2: "12" }
            }
        }
    }
}

/// Hook to access the OTP context.
pub fn use_input_otp() -> InputOTPContext {
    use_context::<InputOTPContext>()
}
