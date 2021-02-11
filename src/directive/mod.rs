// MIT/Apache2 License

mod process;

use crate::{
    brush::Brush,
    cursor::Cursor,
    event::Event,
    icon::Icon,
    menu::Menu,
    server::GuiThread,
    util::DebugContainer,
    window::{ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
    window_class::ClassStyle,
};
use std::{borrow::Cow, ffi::CStr};
use winapi::ctypes::c_int;

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

    // window functions
    CreateWindow {
        class_name: Cow<'static, CStr>,
        window_name: Option<Cow<'static, CStr>>,
        style: WindowStyle,
        extended_style: ExtendedWindowStyle,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        parent: Option<Window>,
        menu: Option<Menu>,
    },
    ShowWindow {
        window: Window,
        command: ShowWindowCommand,
    },
}
