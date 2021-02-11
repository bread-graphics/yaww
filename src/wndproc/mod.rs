// MIT/Apache2 License

use crate::window_data::WindowData;
use std::{
    mem, process,
    ptr::{self, NonNull},
};
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, HWND__},
    },
    um::{wingdi, winuser},
};

pub(crate) unsafe extern "system" fn yaww_wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // this just wraps a panic:abort guard around the real wndproc
    // unwinding into the C ffi is UB, so aborting is for the best here
    struct Bomb;

    impl Drop for Bomb {
        #[cold]
        fn drop(&mut self) {
            process::abort();
        }
    }

    let _bomb = Bomb;

    let res = inner_wndproc(hwnd, msg, wparam, lparam);

    // disarm the bomb
    mem::forget(_bomb);

    res
}

#[inline]
fn inner_wndproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // first, check if the handle is null
    // if it's null, there's not much we can do, so we defer to the default
    let hwnd = match NonNull::new(hwnd) {
        Some(hwnd) => hwnd,
        None => return unsafe { winuser::DefWindowProcA(hwnd, msg, wparam, lparam) },
    };

    // special case for WM_NCCREATE
    if msg == winuser::WM_NCCREATE {
        // set thw GWLP_USERDATA item
        let wdata = lparam as *const winuser::CREATESTRUCTA;
        let wdata = unsafe { *wdata }.lpCreateParams;
        unsafe { winuser::SetWindowLongPtrA(hwnd.as_ptr(), winuser::GWLP_USERDATA, wdata as _) };
        return unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };
    }

    // then, try to get our window data from the GWLP_USERDATA variable in the window
    let window_data = match NonNull::new(unsafe {
        winuser::GetWindowLongPtrA(hwnd.as_ptr(), winuser::GWLP_USERDATA)
    } as *mut WindowData)
    {
        Some(window_data) => window_data,
        None => {
            // defer to the default
            return unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };
        }
    };

    match exchange_event(hwnd, msg, wparam, lparam, unsafe { window_data.as_ref() }) {
        Some(res) => res,
        None => unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) },
    }
}

#[inline]
fn exchange_event(
    hwnd: NonNull<HWND__>,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    window_data: &WindowData,
) -> Option<LRESULT> {
    match msg {
        winuser::WM_CLOSE => {
            unsafe { winuser::DestroyWindow(hwnd.as_ptr()) };
            return Some(0);
        }
        winuser::WM_DESTROY => {
            // decrement the window counter by one
            let window_count = window_data.window_count.get().saturating_sub(1);
            window_data.window_count.set(window_count);

            // if the window count is zero and we're in a wait cycle, send the quit message to the thread
            if window_count == 0 && window_data.waiter.borrow().is_some() {
                unsafe { winuser::PostQuitMessage(0) };
            }

            return Some(0);
        }
        _ => (),
    }

    None
}
