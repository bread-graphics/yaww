// MIT/Apache2 License

use crate::{
    color::Color,
    directive::Directive,
    gdiobj::{AsGdiObject, GdiObject, GDI_IDENTIFIER},
    server::SendsDirective,
    task::Task,
};
use breadthread::key_type;
use winapi::{ctypes::c_int, shared::windef::HPEN__, um::wingdi};

key_type! {
    /// Draws lines.
    pub struct Pen(HPEN__) : [PenType, GDI_IDENTIFIER];
}

impl AsGdiObject for Pen {
    #[inline]
    fn into_gdi_object(self) -> GdiObject {
        GdiObject::from_raw(self.into_raw())
    }

    #[inline]
    fn from_gdi_object(obj: GdiObject) -> Self {
        Self::from_raw(obj.into_raw())
    }
}

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

pub trait PenFunctions {
    fn create_pen(
        &self,
        style: PenStyle,
        width: c_int,
        color: Color,
    ) -> crate::Result<Task<crate::Result<Pen>>>;
}

impl<S: SendsDirective> PenFunctions for S {
    #[inline]
    fn create_pen(
        &self,
        style: PenStyle,
        width: c_int,
        color: Color,
    ) -> crate::Result<Task<crate::Result<Pen>>> {
        self.send(Directive::CreatePen {
            style,
            width,
            color,
        })
    }
}
