// MIT/Apache2 License

use crate::{
    brush::Brush, cursor::Cursor, directive::Directive, icon::Icon, server::GuiThread, task::Task,
};
use std::{borrow::Cow, ffi::CStr, future::Future};
use winapi::{shared::minwindef::UINT, um::winuser};

bitflags::bitflags! {
    #[doc = "Flags used for the window class."]
    pub struct ClassStyle: UINT {
        const BYTE_ALIGN_CLIENT = winuser::CS_BYTEALIGNCLIENT;
        const BYTE_ALIGN_WINDOW = winuser::CS_BYTEALIGNWINDOW;
        const CLASS_DC = winuser::CS_CLASSDC;
        const DOUBLE_CLICKS = winuser::CS_DBLCLKS;
        const DROP_SHADOW = winuser::CS_DROPSHADOW;
        const GLOBAL_CLASS = winuser::CS_GLOBALCLASS;
        const H_REDRAW = winuser::CS_HREDRAW;
        const NO_CLOSE = winuser::CS_NOCLOSE;
        const OWN_DC = winuser::CS_OWNDC;
        const PARRENT_DC = winuser::CS_PARENTDC;
        const SAVE_BITS = winuser::CS_SAVEBITS;
        const V_REDRAW = winuser::CS_VREDRAW;
    }
}

impl GuiThread {
    /// Register a class.
    #[inline]
    pub fn register_class<CN: Into<Cow<'static, CStr>>>(
        &self,
        class_name: CN,
        menu_name: Option<Cow<'static, CStr>>,
        style: ClassStyle,
        icon: Option<Icon>,
        small_icon: Option<Icon>,
        cursor: Option<Cursor>,
        background: Option<Brush>,
    ) -> crate::Result<Task<crate::Result>> {
        self.send_directive(Directive::RegisterClass {
            class_name: class_name.into(),
            menu_name,
            style,
            icon,
            small_icon,
            cursor,
            background,
        })
    }
}
