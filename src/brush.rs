// MIT/Apache2 License

use crate::{
    color::Color,
    gui_thread::{Directive, GuiThread, Key},
};

pub type Brush = Key;

impl GuiThread {
    #[inline]
    pub fn create_solid_brush(&self, color: Color) -> crate::Result<Brush> {
        self.send_directive(Directive::CreateSolidBrush(color))
            .and_then(|b| b.unwrap_key())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_solid_brush_async(&self, color: Color) -> crate::Result<Brush> {
        self.send_directive_async(Directive::CreateSolidBrush(color))
            .await
            .and_then(|b| b.unwrap_key())
    }
}
