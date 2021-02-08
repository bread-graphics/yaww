// MIT/Apache2 License

use crate::{
    color::Color,
    gdiobj::GdiObject,
    gui_thread::{Directive, GuiThread, Key, Point, RasterOperation},
};
use std::borrow::Cow;
use winapi::ctypes::c_int;

pub type Dc = Key;

impl Dc {
    #[inline]
    pub fn create_compatible_dc(self, gt: &GuiThread) -> crate::Result<Dc> {
        let res = gt.send_directive(Directive::CreateCompatibleDc(self))?;
        res.unwrap_key()
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_compatible_dc_async(self, gt: &GuiThread) -> crate::Result<Dc> {
        let res = gt
            .send_directive_async(Directive::CreateCompatibleDc(self))
            .await?;
        res.unwrap_key()
    }

    #[inline]
    pub fn select_object(self, gt: &GuiThread, object: GdiObject) -> crate::Result<GdiObject> {
        let res = gt.send_directive(Directive::SelectObject { dc: self, object })?;
        res.unwrap_key()
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn select_object_async(
        self,
        gt: &GuiThread,
        object: GdiObject,
    ) -> crate::Result<GdiObject> {
        let res = gt
            .send_directive_async(Directive::SelectObject { dc: self, object })
            .await?;
        res.unwrap_key()
    }

    #[inline]
    pub fn set_pixel(self, gt: &GuiThread, x: c_int, y: c_int, color: Color) -> crate::Result {
        gt.send_directive(Directive::SetPixel {
            dc: self,
            x,
            y,
            color,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_pixel_async(
        self,
        gt: &GuiThread,
        x: c_int,
        y: c_int,
        color: Color,
    ) -> crate::Result {
        gt.send_directive_async(Directive::SetPixel {
            dc: self,
            x,
            y,
            color,
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn move_to(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result {
        gt.send_directive(Directive::MoveTo { dc: self, x, y })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn move_to_async(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result {
        gt.send_directive_async(Directive::MoveTo { dc: self, x, y })
            .await?;
        Ok(())
    }

    #[inline]
    pub fn line_to(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result {
        gt.send_directive(Directive::LineTo { dc: self, x, y })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn line_to_async(self, gt: &GuiThread, x: c_int, y: c_int) -> crate::Result {
        gt.send_directive_async(Directive::LineTo { dc: self, x, y })
            .await?;
        Ok(())
    }

    #[inline]
    pub fn rectangle(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result {
        gt.send_directive(Directive::Rectangle {
            dc: self,
            left,
            top,
            right,
            bottom,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn rectangle_async(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result {
        gt.send_directive_async(Directive::Rectangle {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn bezier<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive(Directive::Bezier {
            dc: self,
            points: points.into(),
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn bezier_async<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive_async(Directive::Bezier {
            dc: self,
            points: points.into(),
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn polygon<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive(Directive::Polygon {
            dc: self,
            points: points.into(),
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn polygon_async<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive_async(Directive::Polygon {
            dc: self,
            points: points.into(),
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn polyline<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive(Directive::Polyline {
            dc: self,
            points: points.into(),
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn polyline_async<Pts: Into<Cow<'static, [Point]>>>(
        self,
        gt: &GuiThread,
        points: Pts,
    ) -> crate::Result {
        gt.send_directive_async(Directive::Polyline {
            dc: self,
            points: points.into(),
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn ellipse(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result {
        gt.send_directive(Directive::Ellipse {
            dc: self,
            left,
            top,
            right,
            bottom,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn ellipse_async(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> crate::Result {
        gt.send_directive_async(Directive::Ellipse {
            dc: self,
            left,
            top,
            right,
            bottom,
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn round_rect(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        ellipse_width: c_int,
        ellipse_height: c_int,
    ) -> crate::Result {
        gt.send_directive(Directive::RoundRect {
            dc: self,
            left,
            top,
            right,
            bottom,
            ellipse_width,
            ellipse_height,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn round_rect_async(
        self,
        gt: &GuiThread,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        ellipse_width: c_int,
        ellipse_height: c_int,
    ) -> crate::Result {
        gt.send_directive_async(Directive::RoundRect {
            dc: self,
            left,
            top,
            right,
            bottom,
            ellipse_width,
            ellipse_height,
        })
        .await?;
        Ok(())
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
    ) -> crate::Result {
        gt.send_directive(Directive::Chord {
            dc: self,
            rect_left,
            rect_top,
            rect_right,
            rect_bottom,
            line_x1,
            line_x2,
            line_y1,
            line_y2,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn chord_async(
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
    ) -> crate::Result {
        gt.send_directive_async(Directive::Chord {
            dc: self,
            rect_left,
            rect_top,
            rect_right,
            rect_bottom,
            line_x1,
            line_x2,
            line_y1,
            line_y2,
        })
        .await?;
        Ok(())
    }

    #[inline]
    pub fn bit_blt(
        gt: &GuiThread,
        src: Dc,
        dest: Dc,
        srcx: c_int,
        srcy: c_int,
        destx: c_int,
        desty: c_int,
        width: c_int,
        height: c_int,
        op: RasterOperation,
    ) -> crate::Result {
        gt.send_directive(Directive::BitBlt {
            src,
            dest,
            srcx,
            srcy,
            destx,
            desty,
            width,
            height,
            op,
        })?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn bit_blt_async(
        gt: &GuiThread,
        src: Dc,
        dest: Dc,
        srcx: c_int,
        srcy: c_int,
        destx: c_int,
        desty: c_int,
        width: c_int,
        height: c_int,
        op: RasterOperation,
    ) -> crate::Result {
        gt.send_directive_async(Directive::BitBlt {
            src,
            dest,
            srcx,
            srcy,
            destx,
            desty,
            width,
            height,
            op,
        })
        .await?;
        Ok(())
    }
}
