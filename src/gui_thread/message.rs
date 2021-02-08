// MIT/Apache2 License

use super::Key;
use crate::{
    bitmap::Bitmap,
    brush::Brush,
    color::Color,
    cursor::Cursor,
    dc::Dc,
    gdiobj::GdiObject,
    icon::Icon,
    menu::Menu,
    pen::{Pen, PenStyle},
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
};
use std::{borrow::Cow, ffi::CStr, fmt, sync::Arc};
use winapi::{
    ctypes::{c_int, c_ushort},
    shared::{ntdef::LONG, windef::POINT},
    um::wingdi,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpecialResize {
    Maximized,
    Minimized,
    MaxHide,
    MaxShow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RasterOperation {
    SrcCopy,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Point {
    pub x: LONG,
    pub y: LONG,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugContainer<T>(pub T);

impl<T> fmt::Debug for DebugContainer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<cannot format>")
    }
}

impl<T> From<T> for DebugContainer<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self(t)
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Close(Window),
    Move {
        window: Window,
        x: c_ushort,
        y: c_ushort,
    },
    Size {
        window: Window,
        width: c_ushort,
        height: c_ushort,
        special: Option<SpecialResize>,
    },
    Activate {
        window: Window,
        from_mouse_click: bool,
    },
    Deactivate(Window),
    SetFocus(Window),
    KillFocus(Window),
    Enable(Window),
    Disable(Window),
    Paint {
        window: Window,
        dc: Dc,
    },
    Quit,
}

#[derive(Debug)]
pub enum Directive {
    SetEventHandler(DebugContainer<Box<dyn Fn(Event) -> crate::Result + Send + Sync + 'static>>),

    // class functions
    RegisterClass {
        class_name: Cow<'static, CStr>,
        style: ClassStyle,
        icon: Option<Icon>,
        small_icon: Option<Icon>,
        cursor: Option<Cursor>,
        background: Option<Brush>,
        menu_name: Option<Cow<'static, CStr>>,
    },

    // window functions
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

    // gdi object functions
    DeleteObject(GdiObject),

    // dc functions
    CreateCompatibleDc(Dc),
    DeleteDc(Dc),
    SelectObject {
        dc: Dc,
        object: GdiObject,
    },
    SetPixel {
        dc: Dc,
        x: c_int,
        y: c_int,
        color: Color,
    },
    MoveTo {
        dc: Dc,
        x: c_int,
        y: c_int,
    },
    LineTo {
        dc: Dc,
        x: c_int,
        y: c_int,
    },
    Rectangle {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    },
    Bezier {
        dc: Dc,
        points: Cow<'static, [Point]>,
    },
    Polygon {
        dc: Dc,
        points: Cow<'static, [Point]>,
    },
    Polyline {
        dc: Dc,
        points: Cow<'static, [Point]>,
    },
    Ellipse {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    },
    RoundRect {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        ellipse_width: c_int,
        ellipse_height: c_int,
    },
    Chord {
        dc: Dc,
        rect_left: c_int,
        rect_top: c_int,
        rect_right: c_int,
        rect_bottom: c_int,
        line_x1: c_int,
        line_y1: c_int,
        line_x2: c_int,
        line_y2: c_int,
    },
    BitBlt {
        src: Dc,
        dest: Dc,
        srcx: c_int,
        srcy: c_int,
        destx: c_int,
        desty: c_int,
        width: c_int,
        height: c_int,
        op: RasterOperation,
    },

    // pen functions
    CreatePen {
        style: PenStyle,
        width: c_int,
        color: Color,
    },

    // bitmap functions
    GetBitmapInfo(Bitmap),

    // misc. functions
    BeginWait,
    #[doc(hidden)]
    Dummy,
    #[doc(hidden)]
    DeferEventProcessing,
}
// TODO: fmt::debug

impl Directive {
    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum Response {
    Empty,
    Key(Key),
}

impl Response {
    #[inline]
    pub fn unwrap_key(self) -> crate::Result<Key> {
        match self {
            Response::Key(k) => Ok(k),
            _ => Err(crate::Error::TypeMismatch),
        }
    }
}
