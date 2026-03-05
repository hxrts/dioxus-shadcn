//! Table example components.

use dioxus::prelude::*;
use lumen_blocks::components::table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

/// Source code for the basic example.
pub const BASIC_SOURCE: &str = r#"rsx! {
    Table {
        TableCaption { "A list of your recent invoices." }
        TableHeader {
            TableRow {
                TableHead { class: "w-[100px]", "Invoice" }
                TableHead { "Status" }
                TableHead { "Method" }
                TableHead { class: "text-right", "Amount" }
            }
        }
        TableBody {
            TableRow {
                TableCell { class: "font-medium", "INV001" }
                TableCell { "Paid" }
                TableCell { "Credit Card" }
                TableCell { class: "text-right", "$250.00" }
            }
            TableRow {
                TableCell { class: "font-medium", "INV002" }
                TableCell { "Pending" }
                TableCell { "PayPal" }
                TableCell { class: "text-right", "$150.00" }
            }
            TableRow {
                TableCell { class: "font-medium", "INV003" }
                TableCell { "Unpaid" }
                TableCell { "Bank Transfer" }
                TableCell { class: "text-right", "$350.00" }
            }
        }
        TableFooter {
            TableRow {
                TableCell { "Total" }
                TableCell { }
                TableCell { }
                TableCell { class: "text-right", "$750.00" }
            }
        }
    }
}"#;

/// Basic table example.
#[component]
pub fn TableBasicExample() -> Element {
    rsx! {
        Table {
            TableCaption { "A list of your recent invoices." }
            TableHeader {
                TableRow {
                    TableHead { class: "w-[100px]", "Invoice" }
                    TableHead { "Status" }
                    TableHead { "Method" }
                    TableHead { class: "text-right", "Amount" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { class: "font-medium", "INV001" }
                    TableCell { "Paid" }
                    TableCell { "Credit Card" }
                    TableCell { class: "text-right", "$250.00" }
                }
                TableRow {
                    TableCell { class: "font-medium", "INV002" }
                    TableCell { "Pending" }
                    TableCell { "PayPal" }
                    TableCell { class: "text-right", "$150.00" }
                }
                TableRow {
                    TableCell { class: "font-medium", "INV003" }
                    TableCell { "Unpaid" }
                    TableCell { "Bank Transfer" }
                    TableCell { class: "text-right", "$350.00" }
                }
            }
            TableFooter {
                TableRow {
                    TableCell { colspan: "3", "Total" }
                    TableCell { class: "text-right", "$750.00" }
                }
            }
        }
    }
}

/// Source code for the selected rows example.
pub const SELECTED_SOURCE: &str = r#"rsx! {
    Table {
        TableHeader {
            TableRow {
                TableHead { "Name" }
                TableHead { "Email" }
                TableHead { "Role" }
            }
        }
        TableBody {
            TableRow { selected: true,
                TableCell { "John Doe" }
                TableCell { "john@example.com" }
                TableCell { "Admin" }
            }
            TableRow {
                TableCell { "Jane Smith" }
                TableCell { "jane@example.com" }
                TableCell { "User" }
            }
            TableRow { selected: true,
                TableCell { "Bob Johnson" }
                TableCell { "bob@example.com" }
                TableCell { "Editor" }
            }
        }
    }
}"#;

/// Table with selected rows example.
#[component]
pub fn TableSelectedExample() -> Element {
    rsx! {
        Table {
            TableHeader {
                TableRow {
                    TableHead { "Name" }
                    TableHead { "Email" }
                    TableHead { "Role" }
                }
            }
            TableBody {
                TableRow { selected: true,
                    TableCell { "John Doe" }
                    TableCell { "john@example.com" }
                    TableCell { "Admin" }
                }
                TableRow {
                    TableCell { "Jane Smith" }
                    TableCell { "jane@example.com" }
                    TableCell { "User" }
                }
                TableRow { selected: true,
                    TableCell { "Bob Johnson" }
                    TableCell { "bob@example.com" }
                    TableCell { "Editor" }
                }
            }
        }
    }
}
