// MIT/Apache2 License

mod process;

use crate::{
    brush::Brush,
    color::Color,
    cursor::Cursor,
    dc::Dc,
    event::Event,
    gdiobj::GdiObject,
    icon::Icon,
    menu::Menu,
    pen::PenStyle,
    server::GuiThread,
    util::DebugContainer,
    window::{ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
    window_class::ClassStyle,
    Point,
};
use std::{borrow::Cow, ffi::CStr};
use winapi::ctypes::c_int;

#[derive(Debug)]
pub(crate) enum Directive {
    // utility functions
    SetEventHandler(DebugContainer<Box<dyn Fn(&GuiThread, Event) + Send + Sync + 'static>>),
    BeginWait,

    // class functions
    RegisterClass {
        style: ClassStyle,
        icon: Option<Icon>,
        small_icon: Option<Icon>,
        cursor: Option<Cursor>,
        background: Option<Brush>,
        class_name: Cow<'static, CStr>,
        menu_name: Option<Cow<'static, CStr>>,
    },

    // window functions
    CreateWindow {
        class_name: Cow<'static, CStr>,
        window_name: Option<Cow<'static, CStr>>,
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
    CloseWindow(Window),
    GetClientRect(Window),
    GetDesktopWindow,
    GetParent(Window),
    GetWindowRect(Window),
    GetWindowText(Window),
    IsChild {
        parent: Window,
        child: Window,
    },
    IsZoomed(Window),
    MoveWindow {
        window: Window,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        repaint: bool,
    },
    SetParent {
        window: Window,
        new_parent: Option<Window>,
    },
    SetWindowText {
        window: Window,
        text: Cow<'static, CStr>,
    },

    // dc functions
    SelectObject {
        dc: Dc,
        obj: GdiObject,
    },
    ReleaseDc {
        window: Window,
        dc: Dc,
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
    RoundRect {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        width: c_int,
        height: c_int,
    },
    Ellipse {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    },
    BezierCurve {
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

    // pen functions
    CreatePen {
        style: PenStyle,
        width: c_int,
        color: Color,
    },

    // brush functions
    CreateSolidBrush(Color),

    // gdi object functions
    DeleteObject {
        obj: GdiObject,
    },
}
