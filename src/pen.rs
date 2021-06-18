// MIT/Apache2 License

use crate::{
    color::Color, directive::Directive, gdiobj::GdiObject, server::SendsDirective, task::Task,
};
use winapi::{ctypes::c_int, um::wingdi};

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
        self.send_directive(Directive::CreatePen {
            style,
            width,
            color,
        })
    }
}
