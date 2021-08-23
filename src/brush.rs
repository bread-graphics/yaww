// MIT/Apache2 License

use crate::{
    color::Color,
    directive::Directive,
    gdiobj::{AsGdiObject, GdiObject, GDI_IDENTIFIER},
    server::GuiThread,
    server::SendsDirective,
    task::Task,
};
use breadthread::key_type;
use orphan_crippler::Receiver;
use std::num::NonZeroUsize;
use winapi::{shared::windef::HBRUSH__, um::winuser};

key_type! {
    /// A handle to a GDI brush.
    pub struct Brush(HBRUSH__) : [BrushType, GDI_IDENTIFIER];
}

impl AsGdiObject for Brush {
    #[inline]
    fn into_gdi_object(self) -> GdiObject {
        GdiObject::from_raw(self.into_raw())
    }

    #[inline]
    fn from_gdi_object(obj: GdiObject) -> Self {
        Self::from_raw(obj.into_raw())
    }
}

pub const DEFAULT_BRUSH: Brush = unsafe {
    Brush::from_raw(NonZeroUsize::new_unchecked(
        winuser::COLOR_WINDOW as usize + 1,
    ))
};

pub trait BrushFunctions {
    fn create_solid_brush(&self, color: Color) -> crate::Result<Receiver<crate::Result<Brush>>>;
}

impl<S: SendsDirective> BrushFunctions for S {
    #[inline]
    fn create_solid_brush(&self, color: Color) -> crate::Result<Receiver<crate::Result<Brush>>> {
        self.send(Directive::CreateSolidBrush(color))
    }
}
