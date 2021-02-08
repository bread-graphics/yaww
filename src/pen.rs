// MIT/Apache2 License

use crate::{
    color::Color,
    gdiobj::GdiObject,
    gui_thread::{Directive, GuiThread},
};
use winapi::ctypes::c_int;

pub type Pen = GdiObject;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PenStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Null,
    InsideFrame,
}

impl GuiThread {
    #[inline]
    pub fn create_pen(&self, style: PenStyle, width: c_int, color: Color) -> crate::Result<Pen> {
        let res = self.send_directive(Directive::CreatePen {
            style,
            width,
            color,
        })?;
        res.unwrap_key()
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_pen_async(
        &self,
        style: PenStyle,
        width: c_int,
        color: Color,
    ) -> crate::Result<Pen> {
        let res = self
            .send_directive_async(Directive::CreatePen {
                style,
                width,
                color,
            })
            .await?;
        res.unwrap_key()
    }
}
