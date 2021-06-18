// MIT/Apache2 License

use crate::{
    color::Color, directive::Directive, gdiobj::GdiObject, server::GuiThread,
    server::SendsDirective, task::Task,
};
use orphan_crippler::Receiver;
use std::num::NonZeroUsize;
use winapi::um::winuser;

pub type Brush = GdiObject;

pub const DEFAULT_BRUSH: Brush = unsafe {
    Brush::from_raw(NonZeroUsize::new_unchecked(
        winuser::COLOR_WINDOW as usize + 1,
    ))
};

pub trait BrushFunctions {
    fn create_solid_brush(&self, color: Color) -> crate::Result<Receiver<crate::Result<Brush>>>;
}

impl<S: SendsDirective> BrushFuncions for S {
    #[inline]
    fn create_solid_brush(&self, color: Color) -> crate::Result<Receiver<crate::Result<Brush>>> {
        self.send_directive(Directive::CreateSolidBrush(color))
    }
}
