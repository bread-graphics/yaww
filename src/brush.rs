// MIT/Apache2 License

use crate::{color::Color, directive::Directive, gdiobj::GdiObject, server::GuiThread, task::Task};
use std::num::NonZeroUsize;
use winapi::um::winuser;

pub type Brush = GdiObject;

pub const DEFAULT_BRUSH: Brush = unsafe {
    Brush::from_raw(NonZeroUsize::new_unchecked(
        winuser::COLOR_WINDOW as usize + 1,
    ))
};

impl GuiThread {
    #[inline]
    pub fn create_solid_brush(&self, color: Color) -> crate::Result<Task<Brush>> {
        self.send_directive(Directive::CreateSolidBrush(color))
    }
}
