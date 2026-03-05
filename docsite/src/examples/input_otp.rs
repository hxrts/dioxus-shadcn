//! InputOTP example components.

use dioxus::prelude::*;
use lumen_blocks::components::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"let otp = use_signal(String::new);

rsx! {
    InputOTP {
        value: otp,
        max_length: 6,

        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
        }
        InputOTPSeparator {}
        InputOTPGroup {
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
        }
    }
}"#;

/// Basic input OTP example.
#[component]
pub fn InputOTPBasicExample() -> Element {
    let otp = use_signal(String::new);

    rsx! {
        InputOTP {
            value: otp,
            max_length: 6,

            InputOTPGroup {
                InputOTPSlot { index: 0 }
                InputOTPSlot { index: 1 }
                InputOTPSlot { index: 2 }
            }
            InputOTPSeparator {}
            InputOTPGroup {
                InputOTPSlot { index: 3 }
                InputOTPSlot { index: 4 }
                InputOTPSlot { index: 5 }
            }
        }
    }
}

/// Source code for the pattern example.
pub const PATTERN_SOURCE: &str = r#"let otp = use_signal(String::new);

rsx! {
    InputOTP {
        value: otp,
        max_length: 4,
        pattern: "^[0-9]+$",

        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
            InputOTPSlot { index: 3 }
        }
    }
}"#;

/// Pattern-restricted input OTP example.
#[component]
pub fn InputOTPPatternExample() -> Element {
    let otp = use_signal(String::new);

    rsx! {
        div { class: "space-y-2",
            InputOTP {
                value: otp,
                max_length: 4,
                pattern: "^[0-9]+$",

                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                    InputOTPSlot { index: 3 }
                }
            }
            p { class: "text-sm text-muted-foreground", "Only numbers allowed." }
        }
    }
}

/// Source code for the disabled example.
pub const DISABLED_SOURCE: &str = r#"rsx! {
    InputOTP {
        max_length: 6,
        disabled: true,
        default_value: "123",

        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
        }
    }
}"#;

/// Disabled input OTP example.
#[component]
pub fn InputOTPDisabledExample() -> Element {
    rsx! {
        InputOTP {
            max_length: 6,
            disabled: true,
            default_value: "123".to_string(),

            InputOTPGroup {
                InputOTPSlot { index: 0 }
                InputOTPSlot { index: 1 }
                InputOTPSlot { index: 2 }
                InputOTPSlot { index: 3 }
                InputOTPSlot { index: 4 }
                InputOTPSlot { index: 5 }
            }
        }
    }
}
