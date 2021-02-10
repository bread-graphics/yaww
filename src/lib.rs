// MIT/Apache2 License

#![cfg(windows)]
#![allow(unused_unsafe)] // this is done on purpose

pub mod brush;
pub mod cursor;
pub mod icon;
pub mod window;
pub mod window_class;

pub(crate) mod directive;
pub(crate) mod error;
pub(crate) mod event;
pub(crate) mod key;
pub(crate) mod server;
pub(crate) mod task;
pub(crate) mod util;
pub(crate) mod window_data;
pub(crate) mod wndproc;

pub use error::*;
pub use key::*;
pub use server::GuiThread;
pub use window::*;
pub use window_class::*;
