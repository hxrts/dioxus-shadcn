//! Carousel component for image/content slideshows.
//!
//! A carousel implementation inspired by embla-carousel, using CSS scroll-snap
//! for native smooth scrolling behavior with keyboard navigation support.

use dioxus::prelude::*;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique carousel IDs.
static CAROUSEL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Carousel orientation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    /// Returns the CSS axis value.
    pub fn axis(&self) -> &'static str {
        match self {
            CarouselOrientation::Horizontal => "x",
            CarouselOrientation::Vertical => "y",
        }
    }

    /// Returns the data attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            CarouselOrientation::Horizontal => "horizontal",
            CarouselOrientation::Vertical => "vertical",
        }
    }
}

/// Carousel alignment options.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CarouselAlign {
    Start,
    #[default]
    Center,
    End,
}

impl CarouselAlign {
    /// Returns the scroll-snap-align value.
    pub fn as_str(&self) -> &'static str {
        match self {
            CarouselAlign::Start => "start",
            CarouselAlign::Center => "center",
            CarouselAlign::End => "end",
        }
    }
}

/// Options for configuring carousel behavior.
#[derive(Clone, PartialEq, Debug)]
pub struct CarouselOptions {
    /// Alignment of slides within the viewport.
    pub align: CarouselAlign,
    /// Whether the carousel loops infinitely.
    pub loop_: bool,
    /// Whether drag/scroll is free or snaps.
    pub drag_free: bool,
    /// Number of slides to scroll at once.
    pub slides_to_scroll: usize,
    /// Starting slide index.
    pub start_index: usize,
    /// Whether to contain scroll (prevent empty space).
    pub contain_scroll: bool,
}

impl Default for CarouselOptions {
    fn default() -> Self {
        Self {
            align: CarouselAlign::Center,
            loop_: false,
            drag_free: false,
            slides_to_scroll: 1,
            start_index: 0,
            contain_scroll: true,
        }
    }
}

/// Context for carousel state management.
#[derive(Clone, Copy)]
pub struct CarouselContext {
    /// The carousel orientation.
    pub orientation: CarouselOrientation,
    /// Alignment option.
    pub align: CarouselAlign,
    /// Whether looping is enabled.
    pub loop_: bool,
    /// Whether drag is free.
    pub drag_free: bool,
    /// Slides to scroll at once.
    pub slides_to_scroll: usize,
    /// Current selected slide index.
    pub selected_index: Signal<usize>,
    /// Total number of slides.
    pub slide_count: Signal<usize>,
    /// Whether we can scroll to previous.
    pub can_scroll_prev: Signal<bool>,
    /// Whether we can scroll to next.
    pub can_scroll_next: Signal<bool>,
    /// Unique ID for the carousel content element.
    pub content_id: Signal<String>,
}

impl CarouselContext {
    /// Scroll to the previous slide.
    pub fn scroll_prev(&self) {
        let current = *self.selected_index.read();
        let count = *self.slide_count.read();
        if count == 0 {
            return;
        }

        let new_index = if current == 0 {
            if self.loop_ {
                count.saturating_sub(1)
            } else {
                0
            }
        } else {
            current.saturating_sub(self.slides_to_scroll)
        };

        self.scroll_to(new_index);
    }

    /// Scroll to the next slide.
    pub fn scroll_next(&self) {
        let current = *self.selected_index.read();
        let count = *self.slide_count.read();
        if count == 0 {
            return;
        }

        let new_index = if current >= count.saturating_sub(1) {
            if self.loop_ {
                0
            } else {
                count.saturating_sub(1)
            }
        } else {
            (current + self.slides_to_scroll).min(count.saturating_sub(1))
        };

        self.scroll_to(new_index);
    }

    /// Scroll to a specific slide index.
    pub fn scroll_to(&self, index: usize) {
        let count = *self.slide_count.read();
        if count == 0 || index >= count {
            return;
        }

        // Update selected index (this triggers the effect in CarouselContent)
        let mut selected = self.selected_index;
        selected.set(index);
    }

    /// Update can_scroll states based on current position.
    pub fn update_scroll_states(&self) {
        let current = *self.selected_index.read();
        let count = *self.slide_count.read();
        let mut can_prev = self.can_scroll_prev;
        let mut can_next = self.can_scroll_next;

        if self.loop_ {
            can_prev.set(count > 0);
            can_next.set(count > 0);
        } else {
            can_prev.set(current > 0);
            can_next.set(current < count.saturating_sub(1));
        }
    }
}

/// Hook to access carousel context.
pub fn use_carousel() -> CarouselContext {
    use_context::<CarouselContext>()
}

/// Props for the Carousel component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Carousel orientation (horizontal or vertical).
    #[props(default)]
    pub orientation: CarouselOrientation,

    /// Carousel options.
    #[props(default)]
    pub opts: Option<CarouselOptions>,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Child elements.
    pub children: Element,
}

/// A carousel/slider component.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Carousel {
///         CarouselContent {
///             CarouselItem { "Slide 1" }
///             CarouselItem { "Slide 2" }
///             CarouselItem { "Slide 3" }
///         }
///         CarouselPrevious {}
///         CarouselNext {}
///     }
/// }
/// ```
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let options = props.opts.clone().unwrap_or_default();
    let orientation = props.orientation;

    // Generate a unique ID for the content element
    let content_id = use_signal(|| {
        let id = CAROUSEL_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("carousel-content-{}", id)
    });

    // State signals
    let selected_index = use_signal(|| options.start_index);
    let slide_count = use_signal(|| 0usize);
    let can_scroll_prev = use_signal(|| false);
    let can_scroll_next = use_signal(|| true);

    // Create context
    let context = CarouselContext {
        orientation,
        align: options.align,
        loop_: options.loop_,
        drag_free: options.drag_free,
        slides_to_scroll: options.slides_to_scroll,
        selected_index,
        slide_count,
        can_scroll_prev,
        can_scroll_next,
        content_id,
    };

    // Provide context
    use_context_provider(|| context);

    // Update scroll states when selection changes
    use_effect(move || {
        let _ = *selected_index.read();
        let _ = *slide_count.read();
        context.update_scroll_states();
    });

    // Keyboard handlers
    let handle_keydown = move |event: KeyboardEvent| {
        match event.key() {
            Key::ArrowLeft => {
                event.prevent_default();
                if orientation == CarouselOrientation::Horizontal {
                    context.scroll_prev();
                }
            }
            Key::ArrowRight => {
                event.prevent_default();
                if orientation == CarouselOrientation::Horizontal {
                    context.scroll_next();
                }
            }
            Key::ArrowUp => {
                event.prevent_default();
                if orientation == CarouselOrientation::Vertical {
                    context.scroll_prev();
                }
            }
            Key::ArrowDown => {
                event.prevent_default();
                if orientation == CarouselOrientation::Vertical {
                    context.scroll_next();
                }
            }
            _ => {}
        }
    };

    let classes = format!("relative {}", custom_class);

    rsx! {
        div {
            class: classes,
            role: "region",
            aria_roledescription: "carousel",
            "data-slot": "carousel",
            "data-orientation": orientation.as_str(),
            tabindex: "0",
            onkeydown: handle_keydown,

            {props.children}
        }
    }
}

/// Props for CarouselContent.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselContentProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Child slides.
    pub children: Element,
}

/// Container for carousel slides.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");
    let mut slide_count = context.slide_count;
    let selected_index = context.selected_index;
    let content_id = context.content_id;

    // Scroll to selected slide when index changes
    use_effect(move || {
        let index = *selected_index.read();
        let id = content_id.read().clone();

        // Use eval to scroll to the slide
        let js = format!(
            r#"
            (function() {{
                const container = document.getElementById('{}');
                if (!container) return;
                const slides = container.children;
                if (slides.length === 0) return;
                if ({} >= slides.length) return;
                const target = slides[{}];
                target.scrollIntoView({{ behavior: 'smooth', inline: 'center', block: 'nearest' }});
            }})();
            "#,
            id, index, index
        );

        spawn(async move {
            let _ = document::eval(&js).await;
        });
    });

    // Count slides on mount
    let count_id = content_id.read().clone();
    use_effect(move || {
        let id = count_id.clone();
        let js = format!(
            r#"
            (function() {{
                const container = document.getElementById('{}');
                if (!container) return 0;
                return container.children.length;
            }})();
            "#,
            id
        );

        spawn(async move {
            if let Ok(count) = document::eval(&js).await {
                if let Some(n) = count.as_f64() {
                    slide_count.set(n as usize);
                }
            }
        });
    });

    let snap_type = if context.drag_free {
        "none"
    } else {
        match context.orientation {
            CarouselOrientation::Horizontal => "x mandatory",
            CarouselOrientation::Vertical => "y mandatory",
        }
    };

    let flex_direction = match context.orientation {
        CarouselOrientation::Horizontal => "",
        CarouselOrientation::Vertical => "flex-col",
    };

    let margin_class = match context.orientation {
        CarouselOrientation::Horizontal => "-ml-4",
        CarouselOrientation::Vertical => "-mt-4",
    };

    let overflow_class = match context.orientation {
        CarouselOrientation::Horizontal => "overflow-x-auto overflow-y-hidden",
        CarouselOrientation::Vertical => "overflow-y-auto overflow-x-hidden",
    };

    let classes = format!(
        "flex {} {} {} {}",
        flex_direction, margin_class, custom_class, overflow_class
    );

    let style = format!(
        "scroll-snap-type: {}; scroll-behavior: smooth; -webkit-overflow-scrolling: touch; scrollbar-width: none; -ms-overflow-style: none;",
        snap_type
    );

    let id = content_id.read().clone();

    rsx! {
        div {
            class: "overflow-hidden",
            "data-slot": "carousel-content",

            div {
                id: id,
                class: classes,
                style: "{style} &::-webkit-scrollbar {{ display: none; }}",

                {props.children}
            }
        }
    }
}

/// Props for CarouselItem.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Child content.
    pub children: Element,
}

/// An individual carousel slide.
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let padding_class = match context.orientation {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };

    let snap_align = context.align.as_str();

    let classes = format!(
        "min-w-0 shrink-0 grow-0 basis-full {} {}",
        padding_class, custom_class
    );

    let style = format!("scroll-snap-align: {};", snap_align);

    rsx! {
        div {
            role: "group",
            aria_roledescription: "slide",
            class: classes,
            style: style,
            "data-slot": "carousel-item",

            {props.children}
        }
    }
}

/// Props for CarouselPrevious.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselPreviousProps {
    /// Button variant.
    #[props(default = ButtonVariant::Outline)]
    pub variant: ButtonVariant,

    /// Button size.
    #[props(default = ButtonSize::Icon)]
    pub size: ButtonSize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Navigation button to go to previous slide.
#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let can_scroll = *context.can_scroll_prev.read();

    let position_class = match context.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -left-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-top-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let classes = format!(
        "absolute size-8 rounded-full {} {}",
        position_class, custom_class
    );

    rsx! {
        Button {
            variant: props.variant,
            size: props.size,
            class: classes,
            disabled: !can_scroll,
            on_click: Callback::new(move |_| context.scroll_prev()),
            "data-slot": "carousel-previous",

            // ArrowLeft icon
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

                path { d: "m12 19-7-7 7-7" }
                path { d: "M19 12H5" }
            }

            span { class: "sr-only", "Previous slide" }
        }
    }
}

/// Props for CarouselNext.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselNextProps {
    /// Button variant.
    #[props(default = ButtonVariant::Outline)]
    pub variant: ButtonVariant,

    /// Button size.
    #[props(default = ButtonSize::Icon)]
    pub size: ButtonSize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Navigation button to go to next slide.
#[component]
pub fn CarouselNext(props: CarouselNextProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let can_scroll = *context.can_scroll_next.read();

    let position_class = match context.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -right-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-bottom-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let classes = format!(
        "absolute size-8 rounded-full {} {}",
        position_class, custom_class
    );

    rsx! {
        Button {
            variant: props.variant,
            size: props.size,
            class: classes,
            disabled: !can_scroll,
            on_click: Callback::new(move |_| context.scroll_next()),
            "data-slot": "carousel-next",

            // ArrowRight icon
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

                path { d: "M5 12h14" }
                path { d: "m12 5 7 7-7 7" }
            }

            span { class: "sr-only", "Next slide" }
        }
    }
}

/// Props for CarouselDots.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselDotsProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Dot indicators showing current slide position.
#[component]
pub fn CarouselDots(props: CarouselDotsProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let selected = *context.selected_index.read();
    let count = *context.slide_count.read();

    let classes = format!(
        "flex items-center justify-center gap-2 mt-4 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            role: "tablist",
            "data-slot": "carousel-dots",

            for i in 0..count {
                button {
                    key: "{i}",
                    class: format!(
                        "size-2 rounded-full transition-colors {}",
                        if i == selected { "bg-primary" } else { "bg-muted-foreground/30 hover:bg-muted-foreground/50" }
                    ),
                    role: "tab",
                    "aria-selected": (i == selected).to_string(),
                    "aria-label": format!("Go to slide {}", i + 1),
                    "data-slot": "carousel-dot",
                    "data-active": (i == selected).to_string(),
                    onclick: move |_| context.scroll_to(i),
                }
            }
        }
    }
}

/// Props for CarouselCounter.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselCounterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
}

/// Text counter showing "X of Y" position.
#[component]
pub fn CarouselCounter(props: CarouselCounterProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let selected = *context.selected_index.read();
    let count = *context.slide_count.read();

    let classes = format!(
        "text-sm text-muted-foreground text-center mt-4 {}",
        custom_class
    );

    rsx! {
        div {
            class: classes,
            "data-slot": "carousel-counter",

            "{selected + 1} of {count}"
        }
    }
}

/// Props for CarouselThumbnails.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselThumbnailsProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Thumbnail children (should match slide count).
    pub children: Element,
}

/// Thumbnail navigation for carousel.
#[component]
pub fn CarouselThumbnails(props: CarouselThumbnailsProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let direction_class = match context.orientation {
        CarouselOrientation::Horizontal => "flex-row",
        CarouselOrientation::Vertical => "flex-col",
    };

    let classes = format!(
        "flex gap-2 mt-4 justify-center {} {}",
        direction_class, custom_class
    );

    rsx! {
        div {
            class: classes,
            role: "tablist",
            "data-slot": "carousel-thumbnails",

            {props.children}
        }
    }
}

/// Props for CarouselThumbnail.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselThumbnailProps {
    /// Index of the slide this thumbnail represents.
    pub index: usize,

    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Thumbnail content (usually an image).
    pub children: Element,
}

/// An individual thumbnail button.
#[component]
pub fn CarouselThumbnail(props: CarouselThumbnailProps) -> Element {
    let context = use_carousel();
    let custom_class = props.class.as_deref().unwrap_or("");

    let selected = *context.selected_index.read();
    let is_active = props.index == selected;
    let index = props.index;

    let classes = format!(
        "relative overflow-hidden rounded-md border-2 transition-colors cursor-pointer {} {}",
        if is_active { "border-primary" } else { "border-transparent hover:border-muted-foreground/50" },
        custom_class
    );

    rsx! {
        button {
            class: classes,
            role: "tab",
            "aria-selected": is_active.to_string(),
            "aria-label": format!("Go to slide {}", props.index + 1),
            "data-slot": "carousel-thumbnail",
            "data-active": is_active.to_string(),
            onclick: move |_| context.scroll_to(index),

            {props.children}
        }
    }
}
