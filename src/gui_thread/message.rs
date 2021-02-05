// MIT/Apache2 License

use crate::{
    brush::Brush,
    cursor::Cursor,
    icon::Icon,
    menu::Menu,
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
};
use std::{borrow::Cow, ffi::CStr, fmt};
use winapi::ctypes::c_int;

#[derive(Debug, Clone)]
pub enum Event {
    Paint,
}

pub enum Directive {
    SetEventHandler(Box<dyn FnMut(Event) -> crate::Result + Send + 'static>),
    RegisterClass {
        class_name: Cow<'static, CStr>,
        style: ClassStyle,
        icon: Option<Icon>,
        small_icon: Option<Icon>,
        cursor: Option<Cursor>,
        background: Option<Brush>,
        menu_name: Option<Cow<'static, CStr>>,
    },
    CreateWindow {
        class_name: Cow<'static, CStr>,
        window_name: Cow<'static, CStr>,
        style: WindowStyle,
        extended_style: ExtendedWindowStyle,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        parent: Option<Window>,
        menu: Option<Menu>,
    },
    ShowWindow {
        window: Window,
        command: ShowWindowCommand,
    },
    MoveWindow {
        window: Window,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        repaint: bool,
    },
    BeginWait,
    #[doc(hidden)]
    Dummy,
}
// TODO: fmt::debug

impl Directive {
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::SetEventHandler(_)
            | Self::ShowWindow { .. }
            | Self::MoveWindow { .. }
            | Self::BeginWait
            | Self::Dummy => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Response {
    Empty,
    Window(Window),
}

impl Response {
    #[inline]
    pub fn unwrap_window(self) -> crate::Result<Window> {
        match self {
            Self::Window(wnd) => Ok(wnd),
            _ => Err(crate::Error::TypeMismatch),
        }
    }
}
