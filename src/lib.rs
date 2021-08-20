// MIT/Apache2 License

#![cfg(windows)]
#![allow(unused_unsafe)] // this is done on purpose

pub mod bitmap;
pub mod brush;
pub mod color;
pub mod cursor;
pub mod dc;
pub mod event;
pub mod gdiobj;
pub mod glrc;
pub mod icon;
pub mod menu;
pub mod monitor;
pub mod pen;
pub mod task;
pub mod window;
pub mod window_class;

pub(crate) mod directive;
pub(crate) mod error;
pub(crate) mod future;
pub(crate) mod key;
pub(crate) mod server;
pub(crate) mod util;
pub(crate) mod vkey;
pub(crate) mod wndproc;

//#[cfg(feature = "direct2d")]
//pub mod d2d1;
//#[cfg(feature = "direct2d")]
//pub(crate) mod ptr;

pub use color::*;
pub use error::*;
pub use event::*;
pub use key::*;
pub use server::{GuiThread, GuiThreadHandle, PinnedGuiThreadHandle, SendsDirective};
pub use task::Task;
pub use window::*;
pub use window_class::*;

use winapi::shared::ntdef::LONG;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rectangle {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Point {
    pub x: LONG,
    pub y: LONG,
}

pub mod prelude {
    pub use super::{
        brush::BrushFunctions, glrc::WglFunctions, monitor::MonitorFunctions, pen::PenFunctions,
        window_class::WcFunctions,
    };
}
