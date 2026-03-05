//! Table components for displaying tabular data.

use dioxus::prelude::*;

/// Props for Table.
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Table content.
    pub children: Element,
}

/// A responsive table container.
///
/// # Example
///
/// ```rust
/// rsx! {
///     Table {
///         TableHeader {
///             TableRow {
///                 TableHead { "Name" }
///                 TableHead { "Email" }
///                 TableHead { class: "text-right", "Amount" }
///             }
///         }
///         TableBody {
///             TableRow {
///                 TableCell { "John Doe" }
///                 TableCell { "john@example.com" }
///                 TableCell { class: "text-right", "$250.00" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Table(props: TableProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "relative w-full overflow-auto",
            "data-slot": "table-container",

            table {
                class: "w-full caption-bottom text-sm {custom_class}",
                "data-slot": "table",
                {props.children}
            }
        }
    }
}

/// Props for TableHeader.
#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header content.
    pub children: Element,
}

/// Table header section containing column headings.
#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        thead {
            class: "[&_tr]:border-b {custom_class}",
            "data-slot": "table-header",
            {props.children}
        }
    }
}

/// Props for TableBody.
#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Body content.
    pub children: Element,
}

/// Table body section containing data rows.
#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        tbody {
            class: "[&_tr:last-child]:border-0 {custom_class}",
            "data-slot": "table-body",
            {props.children}
        }
    }
}

/// Props for TableFooter.
#[derive(Props, Clone, PartialEq)]
pub struct TableFooterProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content.
    pub children: Element,
}

/// Table footer section.
#[component]
pub fn TableFooter(props: TableFooterProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        tfoot {
            class: "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 {custom_class}",
            "data-slot": "table-footer",
            {props.children}
        }
    }
}

/// Props for TableRow.
#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Whether this row is selected.
    #[props(default)]
    pub selected: bool,

    /// Row content.
    pub children: Element,
}

/// A table row.
#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "border-b transition-colors hover:bg-muted/50 {} {}",
        if props.selected { "bg-muted" } else { "" },
        custom_class
    );

    rsx! {
        tr {
            class: classes,
            "data-slot": "table-row",
            "data-selected": props.selected.to_string(),
            {props.children}
        }
    }
}

/// Props for TableHead.
#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Header cell content.
    pub children: Element,
}

/// A table header cell.
#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "h-12 px-4 text-left align-middle font-medium text-muted-foreground \
         [&:has([role=checkbox])]:pr-0 {}",
        custom_class
    );

    rsx! {
        th {
            class: classes,
            "data-slot": "table-head",
            {props.children}
        }
    }
}

/// Props for TableCell.
#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Cell content.
    pub children: Element,
}

/// A table data cell.
#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!(
        "p-4 align-middle [&:has([role=checkbox])]:pr-0 {}",
        custom_class
    );

    rsx! {
        td {
            class: classes,
            "data-slot": "table-cell",
            {props.children}
        }
    }
}

/// Props for TableCaption.
#[derive(Props, Clone, PartialEq)]
pub struct TableCaptionProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: Option<String>,

    /// Caption content.
    pub children: Element,
}

/// A table caption for describing the table.
#[component]
pub fn TableCaption(props: TableCaptionProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");

    let classes = format!("mt-4 text-sm text-muted-foreground {}", custom_class);

    rsx! {
        caption {
            class: classes,
            "data-slot": "table-caption",
            {props.children}
        }
    }
}
