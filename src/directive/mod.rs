// MIT/Apache2 License

use std::{iter, vec::IntoIter as VecIter};

mod process;

use crate::{
    brush::Brush,
    color::Color,
    cursor::Cursor,
    dc::{Dc, PixelFormat},
    event::Event,
    gdiobj::GdiObject,
    glrc::Glrc,
    icon::Icon,
    key::Key,
    menu::Menu,
    monitor::Monitor,
    pen::PenStyle,
    server::GuiThread,
    util::DebugContainer,
    window::{ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
    window_class::ClassStyle,
    Point, Rectangle,
};
use breadthread::Directive as BtDirective;
use std::{borrow::Cow, ffi::CStr, num::NonZeroUsize};
use winapi::ctypes::c_int;

#[derive(Debug)]
#[doc(hidden)]
pub enum Directive {
    // monitor functions
    GetMonitors,
    GetDefaultMonitor,

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
        base_class: Option<Cow<'static, CStr>>,
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
    InvalidateRect {
        window: Window,
        rect: Option<Rectangle>,
        erase: bool,
    },
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
        dont_move: bool,
        dont_resize: bool,
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
    UpdateWindow(Window),

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
    Arc {
        dc: Dc,
        rect_left: c_int,
        rect_top: c_int,
        rect_right: c_int,
        rect_bottom: c_int,
        arc_start_x: c_int,
        arc_start_y: c_int,
        arc_end_x: c_int,
        arc_end_y: c_int,
    },
    Ellipse {
        dc: Dc,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
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
    SwapBuffers(Dc),

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

    // wgl functions
    ChoosePixelFormat {
        dc: Dc,
        pixel_format: Cow<'static, PixelFormat>,
    },
    SetPixelFormat {
        dc: Dc,
        format_id: c_int,
        pixel_format: Cow<'static, PixelFormat>,
    },
    CreateWglContext(Dc),
    MakeWglCurrent {
        dc: Option<Dc>,
        rc: Option<Glrc>,
    },
    DestroyWglContext(Glrc),
    GetWglProcAddress(Cow<'static, CStr>),
}

impl BtDirective for Directive {
    type Pointers = iter::Map<VecIter<Key>, fn(Key) -> NonZeroUsize>;

    #[inline]
    fn pointers(&self) -> Self::Pointers {
        match self {
            Directive::CreateWindow { parent, .. } => {
                parent.into_iter().copied().collect::<Vec<Key>>()
            }
            Directive::IsChild { parent, child } => vec![*parent, *child],
            Directive::SetParent {
                window,
                new_parent: Some(new_parent),
            } => vec![*window, *new_parent],
            Directive::SelectObject { dc, obj } => vec![*dc, *obj],
            Directive::ReleaseDc { window, dc } => vec![*window, *dc],
            Directive::DeleteObject { obj } => vec![*obj],
            Directive::MakeWglCurrent {
                dc: Some(dc),
                rc: Some(rc),
                ..
            } => vec![*dc, *rc],
            Directive::MakeWglCurrent { dc: Some(dc), .. } => vec![*dc],
            Directive::MakeWglCurrent { rc: Some(rc), .. } => vec![*rc],
            Directive::SetParent { window, .. } => vec![*window],
            /*            Directive::SetPixel { dc, .. }
            | Directive::MoveTo { dc, .. }
            | Directive::LineTo { dc, .. }
            | Directive::Rectangle { dc, .. }
            | Directive::RoundRect { dc, .. }
            | Directive::Arc { dc, .. }
            | Directive::Ellipse { dc, .. }
            | Directive::BezierCurve { dc, .. }
            | Directive::Polygon { dc, .. }
            | Directive::Polyline { dc, .. }
            | Directive::ChoosePixelFormat { dc, .. }
            | Directive::SetPixelFormat { dc, .. } => vec![dc],*/
            Directive::ShowWindow { window, .. }
            | Directive::InvalidateRect { window, .. }
            | Directive::MoveWindow { window, .. }
            | Directive::SetWindowText { window, .. } => vec![*window],
            Directive::CloseWindow(w)
            | Directive::GetClientRect(w)
            | Directive::GetParent(w)
            | Directive::GetWindowRect(w)
            | Directive::GetWindowText(w)
            | Directive::IsZoomed(w)
            | Directive::UpdateWindow(w)
            | Directive::SwapBuffers(w)
            | Directive::CreateWglContext(w)
            | Directive::DestroyWglContext(w) => vec![*w],
            _ => vec![],
        }
        .into_iter()
        .map(|k| k.into())
    }
}
