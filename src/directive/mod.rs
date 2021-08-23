// MIT/Apache2 License

use std::{iter, vec::IntoIter as VecIter};

mod process;

use crate::{
    brush::Brush,
    color::Color,
    cursor::Cursor,
    dc::{BitBltOp, Dc, PixelFormat},
    event::Event,
    gdiobj::{GdiObject, StockObject},
    glrc::Glrc,
    icon::Icon,
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
use tinyvec::{array_vec, ArrayVec, ArrayVecIterator};
use winapi::{ctypes::c_int, shared::windef::COLORREF};

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
    CreateCompatibleDc(Dc),
    DeleteDc(Dc),
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
    BitBlt {
        src: Dc,
        dst: Dc,
        src_x: c_int,
        src_y: c_int,
        dst_x: c_int,
        dst_y: c_int,
        width: c_int,
        height: c_int,
        op: BitBltOp,
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
    GetStockObject(StockObject),
    DeleteObject {
        obj: GdiObject,
    },

    // bitmap functions
    CreateCompatibleBitmap {
        dc: Dc,
        width: i32,
        height: i32,
    },
    SetPixels {
        dc: Dc,
        origin_x: i32,
        origin_y: i32,
        width: i32,
        pixels: Vec<COLORREF>,
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

#[doc(hidden)]
pub struct KeyPairThatWeCanApplyDefaultTo {
    key: NonZeroUsize,
    ty: usize,
}

impl Default for KeyPairThatWeCanApplyDefaultTo {
    #[inline]
    fn default() -> Self {
        Self {
            key: NonZeroUsize::new(usize::MAX).unwrap(),
            ty: 0,
        }
    }
}

impl From<(NonZeroUsize, usize)> for KeyPairThatWeCanApplyDefaultTo {
    #[inline]
    fn from(kp: (NonZeroUsize, usize)) -> Self {
        let (key, ty) = kp;
        Self { key, ty }
    }
}

impl KeyPairThatWeCanApplyDefaultTo {
    #[inline]
    fn as_tuple(self) -> (NonZeroUsize, usize) {
        let Self { key, ty } = self;
        (key, ty)
    }
}

/// Backing type.
type BT = [KeyPairThatWeCanApplyDefaultTo; 2];

impl BtDirective for Directive {
    type Pointers = iter::Map<
        ArrayVecIterator<BT>,
        fn(KeyPairThatWeCanApplyDefaultTo) -> (NonZeroUsize, usize),
    >;

    #[inline]
    fn pointers(&self) -> Self::Pointers {
        let list: ArrayVec<BT> = match self {
            Directive::CreateWindow { parent, .. } => {
                parent.into_iter().map(|t| t.verifiable().into()).collect()
            }
            Directive::IsChild { parent, child } => {
                array_vec![parent.verifiable().into(), child.verifiable().into()]
            }
            Directive::SetParent {
                window,
                new_parent: Some(new_parent),
            } => array_vec![window.verifiable().into(), new_parent.verifiable().into()],
            Directive::SelectObject { dc, obj } => {
                array_vec![dc.verifiable().into(), obj.verifiable().into()]
            }
            Directive::ReleaseDc { window, dc } => {
                array_vec![window.verifiable().into(), dc.verifiable().into()]
            }
            Directive::DeleteObject { obj } => array_vec![obj.verifiable().into()],
            Directive::MakeWglCurrent {
                dc: Some(dc),
                rc: Some(rc),
                ..
            } => array_vec![dc.verifiable().into(), rc.verifiable().into()],
            Directive::MakeWglCurrent { dc: Some(dc), .. } => array_vec![dc.verifiable().into()],
            Directive::MakeWglCurrent { rc: Some(rc), .. } => array_vec![rc.verifiable().into()],
            Directive::SetParent { window, .. } => array_vec![window.verifiable().into()],
            Directive::SetPixel { dc, .. }
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
            | Directive::SetPixelFormat { dc, .. } => array_vec![dc.verifiable().into()],
            Directive::ShowWindow { window, .. }
            | Directive::InvalidateRect { window, .. }
            | Directive::MoveWindow { window, .. }
            | Directive::SetWindowText { window, .. } => array_vec![window.verifiable().into()],
            Directive::CloseWindow(w)
            | Directive::GetClientRect(w)
            | Directive::GetParent(w)
            | Directive::GetWindowRect(w)
            | Directive::GetWindowText(w)
            | Directive::IsZoomed(w)
            | Directive::UpdateWindow(w) => array_vec![w.verifiable().into()],
            Directive::DestroyWglContext(w) => array_vec![w.verifiable().into()],
            Directive::SwapBuffers(w) | Directive::CreateWglContext(w) => {
                array_vec![w.verifiable().into()]
            }
            _ => array_vec![],
        };

        list.into_iter()
            .map(KeyPairThatWeCanApplyDefaultTo::as_tuple)
    }
}
