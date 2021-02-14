// MIT/Apache2 License

use crate::{dc::Dc, window::Window};
use gluten_keyboard::Key;
use std::{cell::RefCell, rc::Rc};
use winapi::ctypes::c_int;

#[derive(Debug)]
pub enum Event {
    Quit,
    Close(Window),
    Move {
        window: Window,
        x: c_int,
        y: c_int,
    },
    Size {
        window: Window,
        width: c_int,
        height: c_int,
        reason: SizeReason,
    },
    Activate {
        window: Window,
        with_mouse_click: bool,
        deactivated: Option<Window>,
    },
    Deactivate {
        window: Window,
        activated: Option<Window>,
    },
    SetFocus {
        window: Window,
        focus_loser: Option<Window>,
    },
    KillFocus {
        window: Window,
        focus_gainer: Option<Window>,
    },
    Enabled(Window),
    Disabled(Window),
    Paint {
        window: Window,
        dc: Dc,
    },
    KeyDown {
        window: Window,
        key: Option<Key>,
        repeats: u16,
    },
    KeyUp {
        window: Window,
        key: Option<Key>,
        repeats: u16,
    },
    ButtonDown {
        window: Window,
        button: MouseButton,
        x: c_int,
        y: c_int,
    },
    ButtonUp {
        window: Window,
        button: MouseButton,
        x: c_int,
        y: c_int,
    },
    MouseMove {
        window: Window,
        x: c_int,
        y: c_int,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SizeReason {
    None,
    Minimized,
    Maximized,
    MaxShow,
    MaxHide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}
