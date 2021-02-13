// MIT/Apache2 License

#![cfg(windows)]
#![allow(unused_unsafe)] // this is done on purpose

pub mod brush;
pub mod cursor;
pub mod dc;
pub mod icon;
pub mod menu;
pub mod task;
pub mod window;
pub mod window_class;

pub(crate) mod directive;
pub(crate) mod error;
pub(crate) mod event;
pub(crate) mod future;
pub(crate) mod key;
pub(crate) mod server;
pub(crate) mod util;
pub(crate) mod window_data;
pub(crate) mod wndproc;

pub use error::*;
pub use key::*;
pub use server::GuiThread;
pub use task::Task;
pub use window::*;
pub use window_class::*;

use winapi::shared::ntdef::LONG;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Rectangle {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
