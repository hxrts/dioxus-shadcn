//! Resizable component for split panel layouts.
//!
//! A set of components for creating resizable panel layouts with
//! draggable handles.

use crate::use_unique_id;
use dioxus::prelude::*;

/// Direction for the resizable panel group.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Context for resizable panel state.
#[derive(Clone, Copy)]
pub struct ResizableContext {
    pub direction: ResizableDirection,
}

/// Props for ResizablePanelGroup.
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelGroupProps {
    /// Layout direction.
    #[props(default)]
    pub direction: ResizableDirection,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Panel children.
    pub children: Element,
}

/// A container for resizable panels.
///
/// # Example
///
/// ```rust
/// rsx! {
///     ResizablePanelGroup {
///         direction: ResizableDirection::Horizontal,
///
///         ResizablePanel { default_size: 50.0,
///             div { "Left panel" }
///         }
///         ResizableHandle { with_handle: true }
///         ResizablePanel { default_size: 50.0,
///             div { "Right panel" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    use_context_provider(|| ResizableContext {
        direction: props.direction,
    });

    let direction_class = match props.direction {
        ResizableDirection::Horizontal => "flex-row",
        ResizableDirection::Vertical => "flex-col",
    };

    let classes = format!("flex h-full w-full {} {}", direction_class, custom_class);

    rsx! {
        div {
            class: classes,
            "data-slot": "resizable-panel-group",
            "data-direction": match props.direction {
                ResizableDirection::Horizontal => "horizontal",
                ResizableDirection::Vertical => "vertical",
            },
            "data-panel-group": "true",
            {props.children}
        }
    }
}

/// Props for ResizablePanel.
#[derive(Props, Clone, PartialEq)]
pub struct ResizablePanelProps {
    /// Default size as a percentage (0-100).
    #[props(default = 50.0)]
    pub default_size: f64,

    /// Minimum size as a percentage.
    #[props(default)]
    pub min_size: Option<f64>,

    /// Maximum size as a percentage.
    #[props(default)]
    pub max_size: Option<f64>,

    /// Whether the panel is collapsible.
    #[props(default)]
    pub collapsible: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Panel content.
    pub children: Element,
}

/// A resizable panel within a panel group.
#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let context = use_context::<ResizableContext>();
    let custom_class = props.class.as_deref().unwrap_or("");

    // Calculate flex basis from default size
    let style = format!(
        "flex-basis: {}%; flex-grow: 1; flex-shrink: 1;",
        props.default_size
    );

    let min_style = props.min_size.map(|s| {
        format!(
            "min-{}: {}%;",
            match context.direction {
                ResizableDirection::Horizontal => "width",
                ResizableDirection::Vertical => "height",
            },
            s
        )
    });

    let max_style = props.max_size.map(|s| {
        format!(
            "max-{}: {}%;",
            match context.direction {
                ResizableDirection::Horizontal => "width",
                ResizableDirection::Vertical => "height",
            },
            s
        )
    });

    let full_style = format!(
        "{} {} {}",
        style,
        min_style.unwrap_or_default(),
        max_style.unwrap_or_default()
    );

    let classes = format!("overflow-hidden {}", custom_class);

    rsx! {
        div {
            class: classes,
            style: full_style,
            "data-slot": "resizable-panel",
            "data-panel": "true",
            "data-collapsible": props.collapsible.to_string(),
            {props.children}
        }
    }
}

/// Props for ResizableHandle.
#[derive(Props, Clone, PartialEq)]
pub struct ResizableHandleProps {
    /// Whether to show a grip handle icon.
    #[props(default)]
    pub with_handle: bool,

    /// Whether the handle is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// A draggable handle between resizable panels.
#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let context = use_context::<ResizableContext>();
    let custom_class = props.class.as_deref().unwrap_or("");
    let handle_id = use_unique_id();

    let direction_class = match context.direction {
        ResizableDirection::Horizontal => "w-px cursor-col-resize",
        ResizableDirection::Vertical => "h-px cursor-row-resize",
    };

    let classes = format!(
        "relative flex items-center justify-center bg-border \
         after:absolute after:inset-y-0 after:left-1/2 after:-translate-x-1/2 \
         focus-visible:outline-1 focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 \
         {} {} {}",
        direction_class,
        match context.direction {
            ResizableDirection::Horizontal => "after:w-1",
            ResizableDirection::Vertical => "after:h-1",
        },
        custom_class
    );

    let direction = context.direction;
    let disabled = props.disabled;
    let handle_id_value = handle_id();
    let handle_id_for_pointer_down = handle_id_value.clone();
    let on_pointer_down = move |event: PointerEvent| {
        if disabled {
            return;
        }

        event.prevent_default();

        let coords = event.client_coordinates();
        let start_position = match direction {
            ResizableDirection::Horizontal => coords.x,
            ResizableDirection::Vertical => coords.y,
        };

        start_resize_drag(
            &handle_id_for_pointer_down,
            direction,
            start_position,
            event.pointer_id(),
        );
    };

    let handle_id_for_keydown = handle_id_value.clone();
    let on_keydown = move |event: KeyboardEvent| {
        if disabled {
            return;
        }

        let delta_percent = match direction {
            ResizableDirection::Horizontal => match event.key() {
                Key::ArrowLeft => Some(-2.0),
                Key::ArrowRight => Some(2.0),
                _ => None,
            },
            ResizableDirection::Vertical => match event.key() {
                Key::ArrowUp => Some(-2.0),
                Key::ArrowDown => Some(2.0),
                _ => None,
            },
        };

        let Some(delta_percent) = delta_percent else {
            return;
        };

        event.prevent_default();
        resize_by_keyboard(&handle_id_for_keydown, direction, delta_percent);
    };

    rsx! {
        div {
            id: handle_id_value,
            class: classes,
            "data-slot": "resizable-handle",
            "data-panel-resize-handle": "true",
            "data-disabled": props.disabled.to_string(),
            "data-direction": match context.direction {
                ResizableDirection::Horizontal => "horizontal",
                ResizableDirection::Vertical => "vertical",
            },
            role: "separator",
            aria_orientation: match context.direction {
                ResizableDirection::Horizontal => "vertical",
                ResizableDirection::Vertical => "horizontal",
            },
            tabindex: if props.disabled { "-1" } else { "0" },
            onpointerdown: on_pointer_down,
            onkeydown: on_keydown,

            if props.with_handle {
                div {
                    class: "z-10 flex h-4 w-3 items-center justify-center rounded-sm border bg-border",

                    // GripVertical icon
                    svg {
                        class: "size-2.5",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",

                        circle { cx: "9", cy: "12", r: "1" }
                        circle { cx: "9", cy: "5", r: "1" }
                        circle { cx: "9", cy: "19", r: "1" }
                        circle { cx: "15", cy: "12", r: "1" }
                        circle { cx: "15", cy: "5", r: "1" }
                        circle { cx: "15", cy: "19", r: "1" }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn start_resize_drag(
    handle_id: &str,
    direction: ResizableDirection,
    start_position: f64,
    pointer_id: i32,
) {
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    use web_sys::{HtmlElement, PointerEvent as WebPointerEvent, window};

    let Some(window) = window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let Some(handle) = document
        .get_element_by_id(handle_id)
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let _ = handle.set_pointer_capture(pointer_id);

    let Some(prev) = handle
        .previous_element_sibling()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };

    let Some(next) = handle
        .next_element_sibling()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };

    let Some(parent) = handle
        .parent_element()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };

    let parent_rect = parent.get_bounding_client_rect();
    let total_size = match direction {
        ResizableDirection::Horizontal => parent_rect.width(),
        ResizableDirection::Vertical => parent_rect.height(),
    };
    if total_size <= 0.0 {
        return;
    }

    let last_position = Rc::new(RefCell::new(start_position));
    let move_listener: Rc<RefCell<Option<Closure<dyn FnMut(WebPointerEvent)>>>> =
        Rc::new(RefCell::new(None));
    let up_listener: Rc<RefCell<Option<Closure<dyn FnMut(WebPointerEvent)>>>> =
        Rc::new(RefCell::new(None));

    let move_listener_for_up = Rc::clone(&move_listener);
    let up_listener_for_up = Rc::clone(&up_listener);
    let document_for_up = document.clone();
    let handle_for_up = handle.clone();

    let prev_for_move = prev.clone();
    let next_for_move = next.clone();
    let last_position_for_move = Rc::clone(&last_position);
    let move_closure = Closure::wrap(Box::new(move |event: WebPointerEvent| {
        if event.pointer_id() != pointer_id {
            return;
        }
        event.prevent_default();

        let current = match direction {
            ResizableDirection::Horizontal => event.client_x() as f64,
            ResizableDirection::Vertical => event.client_y() as f64,
        };

        let mut last = last_position_for_move.borrow_mut();
        let delta = current - *last;
        if delta.abs() < f64::EPSILON {
            return;
        }
        *last = current;

        apply_resize_delta(&prev_for_move, &next_for_move, total_size, direction, delta);
    }) as Box<dyn FnMut(WebPointerEvent)>);

    let up_closure = Closure::wrap(Box::new(move |event: WebPointerEvent| {
        if event.pointer_id() != pointer_id {
            return;
        }
        if let Some(mv) = move_listener_for_up.borrow().as_ref() {
            let _ = document_for_up
                .remove_event_listener_with_callback("pointermove", mv.as_ref().unchecked_ref());
        }
        if let Some(up) = up_listener_for_up.borrow().as_ref() {
            let _ = document_for_up
                .remove_event_listener_with_callback("pointerup", up.as_ref().unchecked_ref());
            let _ = document_for_up
                .remove_event_listener_with_callback("pointercancel", up.as_ref().unchecked_ref());
        }
        let _ = handle_for_up.release_pointer_capture(pointer_id);
        *move_listener_for_up.borrow_mut() = None;
        *up_listener_for_up.borrow_mut() = None;
    }) as Box<dyn FnMut(WebPointerEvent)>);

    let _ = document
        .add_event_listener_with_callback("pointermove", move_closure.as_ref().unchecked_ref());
    let _ =
        document.add_event_listener_with_callback("pointerup", up_closure.as_ref().unchecked_ref());
    let _ = document
        .add_event_listener_with_callback("pointercancel", up_closure.as_ref().unchecked_ref());

    *move_listener.borrow_mut() = Some(move_closure);
    *up_listener.borrow_mut() = Some(up_closure);
}

#[cfg(not(target_arch = "wasm32"))]
fn start_resize_drag(
    _handle_id: &str,
    _direction: ResizableDirection,
    _start_position: f64,
    _pointer_id: i32,
) {
}

#[cfg(target_arch = "wasm32")]
fn resize_by_keyboard(handle_id: &str, direction: ResizableDirection, delta_percent: f64) {
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlElement, window};

    let Some(window) = window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let Some(handle) = document
        .get_element_by_id(handle_id)
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let Some(prev) = handle
        .previous_element_sibling()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let Some(next) = handle
        .next_element_sibling()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let Some(parent) = handle
        .parent_element()
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };

    let parent_rect = parent.get_bounding_client_rect();
    let total_size = match direction {
        ResizableDirection::Horizontal => parent_rect.width(),
        ResizableDirection::Vertical => parent_rect.height(),
    };
    if total_size <= 0.0 {
        return;
    }

    let delta = (delta_percent / 100.0) * total_size;
    apply_resize_delta(&prev, &next, total_size, direction, delta);
}

#[cfg(not(target_arch = "wasm32"))]
fn resize_by_keyboard(_handle_id: &str, _direction: ResizableDirection, _delta_percent: f64) {}

#[cfg(target_arch = "wasm32")]
fn apply_resize_delta(
    prev: &web_sys::HtmlElement,
    next: &web_sys::HtmlElement,
    total_size: f64,
    direction: ResizableDirection,
    delta: f64,
) {
    let prev_rect = prev.get_bounding_client_rect();
    let next_rect = next.get_bounding_client_rect();
    let prev_size = match direction {
        ResizableDirection::Horizontal => prev_rect.width(),
        ResizableDirection::Vertical => prev_rect.height(),
    };
    let next_size = match direction {
        ResizableDirection::Horizontal => next_rect.width(),
        ResizableDirection::Vertical => next_rect.height(),
    };
    let pair_total = prev_size + next_size;
    if pair_total <= 0.0 {
        return;
    }

    let min_size = total_size * 0.1;
    let mut new_prev = prev_size + delta;
    let mut new_next = next_size - delta;

    if new_prev < min_size {
        new_prev = min_size;
        new_next = pair_total - min_size;
    }
    if new_next < min_size {
        new_next = min_size;
        new_prev = pair_total - min_size;
    }

    let axis = match direction {
        ResizableDirection::Horizontal => "width",
        ResizableDirection::Vertical => "height",
    };
    let prev_percent = (new_prev / total_size) * 100.0;
    let next_percent = (new_next / total_size) * 100.0;

    let _ = prev
        .style()
        .set_property("flex-basis", &format!("{prev_percent:.4}%"));
    let _ = next
        .style()
        .set_property("flex-basis", &format!("{next_percent:.4}%"));
    let _ = prev.style().set_property("flex-grow", "0");
    let _ = next.style().set_property("flex-grow", "0");
    let _ = prev.style().set_property(&format!("min-{axis}"), "0");
    let _ = next.style().set_property(&format!("min-{axis}"), "0");
}
