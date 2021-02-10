// MIT/Apache2 License

mod process;

use crate::{
    brush::Brush,
    cursor::Cursor,
    event::Event,
    icon::Icon,
    server::GuiThread,
    util::DebugContainer,
    window::{ExtendedWindowStyle, WindowStyle},
    window_class::ClassStyle,
};
use std::{borrow::Cow, ffi::CStr};

#[derive(Debug)]
pub(crate) enum Directive {
    // utility functions
    SetEventHandler(DebugContainer<Box<dyn Fn(&GuiThread, Event) + Send + 'static>>),

    // class functions
    RegisterClass {
        style: ClassStyle,
        icon: Option<Icon>,
        small_icon: Option<Icon>,
        cursor: Option<Cursor>,
        background: Option<Brush>,
        class_name: Cow<'static, CStr>,
        menu_name: Option<Cow<'static, CStr>>,
    },
}
