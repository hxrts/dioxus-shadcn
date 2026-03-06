//! Page modules.

pub mod blocks;
pub mod charts;
pub mod colors;
pub mod docs;
pub mod err_404;
pub mod examples;
pub mod home;
pub mod themes;

pub use blocks::Blocks;
pub use blocks::BlocksCategory;
pub use charts::{ChartType, Charts};
pub use colors::Colors;
pub use err_404::Err404;
pub use examples::{
    AuthenticationExample, DashboardExample, PlaygroundExample, RtlExample, TasksExample,
};
pub use home::Home;
pub use themes::Themes;
