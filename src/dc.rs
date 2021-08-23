// MIT/Apache2 License

use crate::{
    bitmap::Bitmap, color::Color, directive::Directive, gdiobj::{AsGdiObject, GdiObject}, server::SendsDirective,
    task::Task, Point,
};
use breadthread::key_type;
use std::{borrow::Cow, mem};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{BYTE, DWORD},
        windef::{COLORREF, HDC__},
    },
    um::wingdi,
};

key_type! {
    /// A drawing context. This either points to a window, to a location in memory, to a screen, or some other
    /// exotic location.
    pub struct Dc(HDC__) : [DcType, 0x991];
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelFormat {
    pub flags: PixelFormatFlags,
    pub ty: PixelType,
    pub color_bits: BYTE,
    pub red_bits: BYTE,
    pub red_shift: BYTE,
    pub green_bits: BYTE,
    pub green_shift: BYTE,
    pub blue_bits: BYTE,
    pub blue_shift: BYTE,
    pub alpha_bits: BYTE,
    pub alpha_shift: BYTE,
    pub accum_bits: BYTE,
    pub accum_red_bits: BYTE,
    pub accum_green_bits: BYTE,
    pub accum_blue_bits: BYTE,
    pub accum_alpha_bits: BYTE,
    pub depth_bits: BYTE,
    pub stencil_bits: BYTE,
    pub aux_buffers: BYTE,
    pub visible_mask: DWORD,
}

impl From<PixelFormat> for wingdi::PIXELFORMATDESCRIPTOR {
    #[inline]
    fn from(pf: PixelFormat) -> wingdi::PIXELFORMATDESCRIPTOR {
        let PixelFormat {
            flags,
            ty,
            color_bits,
            red_bits,
            red_shift,
            green_bits,
            green_shift,
            blue_bits,
            blue_shift,
            alpha_bits,
            alpha_shift,
            accum_bits,
            accum_red_bits,
            accum_green_bits,
            accum_blue_bits,
            accum_alpha_bits,
            depth_bits,
            stencil_bits,
            aux_buffers,
            visible_mask,
        } = pf;

        wingdi::PIXELFORMATDESCRIPTOR {
            nSize: mem::size_of::<wingdi::PIXELFORMATDESCRIPTOR>() as _,
            nVersion: 1,
            dwFlags: flags.bits(),
            iPixelType: match ty {
                PixelType::Rgba => wingdi::PFD_TYPE_RGBA,
                PixelType::ColorIndex => wingdi::PFD_TYPE_COLORINDEX,
            },
            cColorBits: color_bits,
            cRedBits: red_bits,
            cRedShift: red_shift,
            cGreenBits: green_bits,
            cGreenShift: green_shift,
            cBlueBits: blue_bits,
            cBlueShift: blue_shift,
            cAlphaBits: alpha_bits,
            cAlphaShift: alpha_shift,
            cAccumBits: accum_bits,
            cAccumRedBits: accum_red_bits,
            cAccumGreenBits: accum_green_bits,
            cAccumBlueBits: accum_blue_bits,
            cAccumAlphaBits: accum_alpha_bits,
            cDepthBits: depth_bits,
            cStencilBits: stencil_bits,
            cAuxBuffers: aux_buffers,
            iLayerType: 0,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: visible_mask,
            dwDamageMask: 0,
        }
    }
}

bitflags::bitflags! {
    pub struct PixelFormatFlags: DWORD {
        const DRAW_TO_WINDOW = wingdi::PFD_DRAW_TO_WINDOW;
        const DRAW_TO_BITMAP = wingdi::PFD_DRAW_TO_BITMAP;
        const SUPPORT_GDI = wingdi::PFD_SUPPORT_GDI;
        const SUPPORT_OPENGL = wingdi::PFD_SUPPORT_OPENGL;
        const GENERIC_ACCELERATED = wingdi::PFD_GENERIC_ACCELERATED;
        const GENERIC_FORMAT = wingdi::PFD_GENERIC_FORMAT;
        const NEED_PALETTE = wingdi::PFD_NEED_PALETTE;
        const NEED_SYSTEM_PALETTE = wingdi::PFD_NEED_SYSTEM_PALETTE;
        const DOUBLEBUFFER = wingdi::PFD_DOUBLEBUFFER;
        const STEREO = wingdi::PFD_STEREO;
        const SWAP_LAYER_BUFFERS = wingdi::PFD_SWAP_LAYER_BUFFERS;
        const DEPTH_DONT_CARE = wingdi::PFD_DEPTH_DONTCARE;
        const DOUBLEBUFFER_DONT_CARE = wingdi::PFD_DOUBLEBUFFER_DONTCARE;
        const STEREO_DONT_CARE = wingdi::PFD_STEREO_DONTCARE;
        const SWAP_COPY = wingdi::PFD_SWAP_COPY;
        const SWAP_EXCHANGE = wingdi::PFD_SWAP_EXCHANGE;
    }
}

impl Default for PixelFormatFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelType {
    Rgba,
    ColorIndex,
}

impl Default for PixelType {
    #[inline]
    fn default() -> Self {
        Self::Rgba
    }
}

impl From<PixelType> for BYTE {
    #[inline]
    fn from(pt: PixelType) -> BYTE {
        match pt {
            PixelType::Rgba => wingdi::PFD_TYPE_RGBA,
            PixelType::ColorIndex => wingdi::PFD_TYPE_COLORINDEX,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BitBltOp {
    Blackness,
    CaptureBlt,
    DstInvert,
    MergeCopy,
    MergePaint,
    NoMirrorBitmap,
    NotSrcCopy,
    NotSrcErase,
    PatCopy,
    PatInvert,
    PatPaint,
    SrcAnd,
    SrcCopy,
    SrcErase,
    SrcInvert,
    SrcPaint,
    Whiteness,
}

impl Dc {
    #[inline]
    pub fn create_compatible_dc<S: SendsDirective>(
        self,
        gt: &S,
    ) -> crate::Result<Task<crate::Result<Dc>>> {
        gt.send(Directive::CreateCompatibleDc(self))
    }

    #[inline]
    pub fn delete<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<()>> {
        gt.send(Directive::DeleteDc(self))
    }

    #[inline]
    pub fn create_compatible_bitmap<S: SendsDirective>(
        self,
        gt: &S,
        width: i32,
        height: i32,
    ) -> crate::Result<Task<crate::Result<Bitmap>>> {
        gt.send(Directive::CreateCompatibleBitmap {
            dc: self,
            width,
            height,
        })
    }

    #[inline]
    pub fn draw_pixels<S: SendsDirective, Pixels: IntoIterator<Item = COLORREF>>(
        self,
        gt: &S,
        origin_x: i32,
        origin_y: i32,
        width: i32,
        pixels: Pixels,
    ) -> crate::Result<Task<crate::Result<()>>> {
        gt.send(Directive::SetPixels {
            dc: self,
            origin_x,
            origin_y,
            width,
            pixels: pixels.into_iter().collect(),
        })
    }

    #[inline]
    pub fn select_object<S: SendsDirective, O: AsGdiObject>(
        self,
        gt: &S,
        obj: O,
    ) -> crate::Result<Task<crate::Result<GdiObject>>> {
        gt.send(Directive::SelectObject { dc: self, obj: obj.into_gdi_object() })
    }

    #[inline]
    pub fn swap_buffers<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::SwapBuffers(self))
    }

    #[inline]
    pub fn set_pixel<S: SendsDirective>(
        self,
        gt: &S,
        x: c_int,
        y: c_int,
        color: Color,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::SetPixel {
            dc: self,
            x,
            y,
            color,
        })
    }

    #[inline]
    pub fn move_to<S: SendsDirective>(
        self,
        gt: &S,
        x: c_int,
        y: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::MoveTo { dc: self, x, y })
    }

    #[inline]
    pub fn line_to<S: SendsDirective>(
        self,
        gt: &S,
        x: c_int,
        y: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::LineTo { dc: self, x, y })
    }

    #[inline]
    pub fn rectangle<S: SendsDirective>(
        self,
        gt: &S,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Rectangle {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
    }

    #[inline]
    pub fn round_rect<S: SendsDirective>(
        self,
        gt: &S,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        width: c_int,
        height: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::RoundRect {
            dc: self,
            left,
            top,
            right,
            bottom,
            width,
            height,
        })
    }

    #[inline]
    pub fn arc<S: SendsDirective>(
        self,
        gt: &S,
        rect_left: c_int,
        rect_top: c_int,
        rect_right: c_int,
        rect_bottom: c_int,
        arc_start_x: c_int,
        arc_start_y: c_int,
        arc_end_x: c_int,
        arc_end_y: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Arc {
            dc: self,
            rect_left,
            rect_top,
            rect_right,
            rect_bottom,
            arc_start_x,
            arc_start_y,
            arc_end_x,
            arc_end_y,
        })
    }

    #[inline]
    pub fn ellipse<S: SendsDirective>(
        self,
        gt: &S,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Ellipse {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
    }

    #[inline]
    pub fn chord<S: SendsDirective>(
        self,
        gt: &S,
        rect_left: c_int,
        rect_top: c_int,
        rect_right: c_int,
        rect_bottom: c_int,
        line_x1: c_int,
        line_y1: c_int,
        line_x2: c_int,
        line_y2: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Chord {
            dc: self,
            rect_left,
            rect_top,
            rect_right,
            rect_bottom,
            line_x1,
            line_y1,
            line_x2,
            line_y2,
        })
    }

    #[inline]
    pub fn bezier_curve<S: SendsDirective, Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &S,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::BezierCurve {
            dc: self,
            points: points.into(),
        })
    }

    #[inline]
    pub fn polygon<S: SendsDirective, Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &S,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Polygon {
            dc: self,
            points: points.into(),
        })
    }

    #[inline]
    pub fn polyline<S: SendsDirective, Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &S,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::Polyline {
            dc: self,
            points: points.into(),
        })
    }

    #[inline]
    pub fn bit_blt<S: SendsDirective>(self, gt: &S, src_x: c_int, src_y: c_int, width: c_int, height: c_int, dst: Dc, dst_x: c_int, dst_y: c_int, op: BitBltOp) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::BitBlt { src: self, dst, src_x, src_y, width, height, dst_x, dst_y, op })
    }
}
