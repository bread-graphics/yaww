// MIT/Apache2 License

use crate::{
    color::Color, directive::Directive, gdiobj::GdiObject, server::GuiThread, task::Task, Key,
    Point,
};
use std::borrow::Cow;
use winapi::ctypes::c_int;

pub type Dc = Key;

impl Dc {
    #[inline]
    pub fn select_object(
        self,
        gt: &GuiThread,
        obj: GdiObject,
    ) -> crate::Result<Task<crate::Result<GdiObject>>> {
        gt.send_directive(Directive::SelectObject { dc: self, obj })
    }

    #[inline]
    pub fn set_pixel(
        self,
        gt: &GuiThread,
        x: c_int,
        y: c_int,
        color: Color,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::SetPixel {
            dc: self,
            x,
            y,
            color,
        })
    }

    #[inline]
    pub fn move_to(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::MoveTo { dc: self, x, y })
    }

    #[inline]
    pub fn line_to(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::LineTo { dc: self, x, y })
    }

    #[inline]
    pub fn rectangle(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::Rectangle {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
    }

    #[inline]
    pub fn round_rect(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        width: c_int,
        height: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::RoundRect {
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
    pub fn ellipse(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::Ellipse {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
    }

    #[inline]
    pub fn chord(
        self,
        gt: &GuiThread,
        rect_left: c_int,
        rect_top: c_int,
        rect_right: c_int,
        rect_bottom: c_int,
        line_x1: c_int,
        line_y1: c_int,
        line_x2: c_int,
        line_y2: c_int,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::Chord {
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
    pub fn bezier_curve<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::BezierCurve {
            dc: self,
            points: points.into(),
        })
    }

    #[inline]
    pub fn polygon<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::Polygon {
            dc: self,
            points: points.into(),
        })
    }

    #[inline]
    pub fn polyline<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result<Task<crate::Result>> {
        gt.send_directive(Directive::Polyline {
            dc: self,
            points: points.into(),
        })
    }
}
