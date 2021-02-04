// MIT/Apache2 License

use crate::{
    gui_thread::{Directive, Event, Response},
    refcell::{RefCell, RefMut},
};
use flume::Sender;
use std::{mem, ptr::NonNull, sync::Mutex};
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
                // convert lparam to a CREATESTRUCT pointer
                if lparam != 0 {
                    let create_params = &*(lparam as *const CREATESTRUCTA);
                    SetWindowLongPtrA(
                        hwnd.as_ptr(),
                        GWLP_USERDATA,
                        create_params.lpCreateParams as _,
                    );
                }
            }

            return DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam);
        }
    };

    let mut dw = Some(window_data.borrow_mut());
    let res = wndproc_inner(hwnd, msg, wparam, lparam, &mut dw);
    mem::drop(dw);

    match res {
        Some(res) => res,
        None => {
            // default behavior is to defer to the default window proc and let win32
            // handle important stuff
            DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam)
        }
    }
}

fn wndproc_inner(
    hwnd: NonNull<HWND__>,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    window_data: &mut Option<RefMut<'_, WindowData>>,
) -> Option<LRESULT> {
    // match on the message
    match msg {
        WM_DESTROY => {
            let mut window_data = window_data.take().unwrap();
            window_data.window_count = window_data.window_count.saturating_sub(1);
            log::debug!("Window count is now {}", window_data.window_count);
            if window_data.waiting && window_data.window_count == 0 {
                log::debug!("Posting quit message into queue");
                unsafe { PostQuitMessage(0) };
                return Some(0);
            }
        }
        _ => (),
    }

    None
}
