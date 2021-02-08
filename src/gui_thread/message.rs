// MIT/Apache2 License

use crate::{
    bitmap::Bitmap,
    brush::Brush,
    color::Color,
    cursor::Cursor,
    dc::Dc,
    gdiobj::GdiObject,
    icon::Icon,
    menu::Menu,
    pen::Pen,
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
};
use std::{borrow::Cow, ffi::CStr, fmt};
use winapi::{
    ctypes::{c_int, c_ushort},
    shared::windef::POINT,
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

pub type Point = POINT;

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
}

pub enum Directive {
    SetEventHandler(Box<dyn FnMut(Event) -> crate::Result + Send + 'static>),

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

    // bitmap functions
    GetBitmapInfo(Bitmap),

    // misc. functions
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
            | Self::DeleteObject(_)
            | Self::DeleteDc(_)
            | Self::SetPixel { .. }
            | Self::MoveTo { .. }
            | Self::LineTo { .. }
            | Self::Rectangle { .. }
            | Self::Bezier { .. }
            | Self::Polygon { .. }
            | Self::BitBlt { .. }
            | Self::Dummy => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Response {
    Empty,
    Dc(Dc),
    GdiObject(GdiObject),
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

    #[inline]
    pub fn unwrap_dc(self) -> crate::Result<Dc> {
        match self {
            Self::Dc(dc) => Ok(dc),
            _ => Err(crate::Error::TypeMismatch),
        }
    }

    #[inline]
    pub fn unwrap_gdiobj(self) -> crate::Result<GdiObject> {
        match self {
            Self::GdiObject(o) => Ok(o),
            _ => Err(crate::Error::TypeMismatch),
        }
    }
}
