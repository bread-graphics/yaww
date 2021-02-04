// MIT/Apache2 License

use crate::gui_thread::{Directive, Event, Response};
use flume::Sender;
use std::{cell::RefCell, mem, ptr::NonNull};
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, HWND__},
    },
    um::winuser::*,
};

/// Data we expect to be available to every window.
pub(crate) struct WindowData {
    // event handler
    pub event_handler: Box<dyn FnMut(Event) -> crate::Result + Send + 'static>,
    pub window_count: usize,
    pub waiting: bool,
}

/// Window GUI procedure.
pub(crate) unsafe extern "system" fn yaww_wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // if the window is null, there isn't much we can do
    // forward it to DefWindowProcA
    let hwnd = match NonNull::new(hwnd) {
        Some(hwnd) => hwnd,
        None => return DefWindowProcA(hwnd, msg, wparam, lparam),
    };

    // first, we need to get the pointer to the window data, which takes the form of a pointer
    // to the window data
    // SAFETY: this only happens on one thread so we know by default that no one has mutable access to
    //         the window data if it's unsafe
    let data_pointer =
        GetWindowLongPtrA(hwnd.as_ptr(), GWLP_USERDATA) as *const RefCell<WindowData>;
    let window_data = match data_pointer.as_ref() {
        Some(dp) => dp,
        None => {
            // when we first create the window, we pass in a pointer in the lparam slot to the original
            // data, so we should probably set it here
            if msg == WM_NCCREATE {
                SetWindowLongPtrA(hwnd.as_ptr(), GWLP_USERDATA, lparam);
            }

            return DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam);
        }
    };

    log::trace!("Borrowing window data");
    let mut dw = window_data.borrow_mut();
    let res = wndproc_inner(hwnd, msg, wparam, lparam, &mut dw);
    log::trace!("Dropping window data");
    mem::drop(dw);
    res
}

fn wndproc_inner(
    hwnd: NonNull<HWND__>,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    window_data: &mut WindowData,
) -> LRESULT {
    // match on the message
    match msg {
        WM_DESTROY => {
            window_data.window_count = window_data.window_count.saturating_sub(1);
            log::debug!("Window count is now {}", window_data.window_count);
            if window_data.waiting && window_data.window_count == 0 {
                log::debug!("Posting quit message into queue");
                unsafe { PostQuitMessage(0) };
            }
        }
        _ => (),
    }

    // default behavior is to defer to the default window proc and let win32
    // handle important stuff
    unsafe { DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) }
}
