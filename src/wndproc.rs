// MIT/Apache2 License

use crate::{
    dc::Dc,
    event::{Event, SizeReason},
    server::{PinnedGuiThreadHandle, YawwController},
    vkey::convert_vkey_to_key as convert_vkey,
    window::Window,
};
use std::{
    mem::{self, MaybeUninit},
    num::NonZeroUsize,
    process,
    ptr::{self, NonNull},
};
use winapi::{
    shared::{
        minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, HWND__},
    },
    um::{wingdi, winuser},
};

/// Combine insertion of the subclass and the window handle.
pub(crate) struct HandleAndSubclass<'evh> {
    pub(crate) handle: Box<PinnedGuiThreadHandle<'evh>>,
    pub(crate) subclass: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
}

// root to where the win32 api calls for the wndproc
// 'evh is a parameter for PinnedGuiThreadHandle
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
fn inner_wndproc<'evh>(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // first, check if the handle is null
    // if it's null, there's not much we can do, so we defer to the default
    let hwnd = match NonNull::new(hwnd) {
        Some(hwnd) => hwnd,
        None => return unsafe { winuser::DefWindowProcA(hwnd, msg, wparam, lparam) },
    };

    // if the handle doesn't refer to a window, defer to the default procedure
    if unsafe { winuser::IsWindow(hwnd.as_ptr()) } == 0 {
        return unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };
    }

    // special case for WM_NCCREATE
    if msg == winuser::WM_NCCREATE {
        // set the GWLP_USERDATA item
        let wdata = lparam as *const winuser::CREATESTRUCTA;
        let wdata = unsafe { *wdata }.lpCreateParams as *mut HandleAndSubclass<'evh>;
        let wdata = unsafe { Box::from_raw(wdata) };

        // glob the default window proc from this pointer so we can call it later
        let HandleAndSubclass { handle, subclass } = *wdata;
        if let Some(subclass) = subclass {
            let _ = handle.with(move |controller| {
                controller.insert_subclass(
                    unsafe { NonZeroUsize::new_unchecked(hwnd.as_ptr() as usize) },
                    subclass,
                )
            });
        }
        let def_window_proc = subclass.unwrap_or(winuser::DefWindowProcA);

        unsafe {
            winuser::SetWindowLongPtrA(
                hwnd.as_ptr(),
                winuser::GWLP_USERDATA,
                Box::into_raw(handle) as _,
            )
        };
        return unsafe { def_window_proc(hwnd.as_ptr(), msg, wparam, lparam) };
    }

    // then, try to get our handle from the GWLP_USERDATA variable in the window
    let handle = unsafe { winuser::GetWindowLongPtrA(hwnd.as_ptr(), winuser::GWLP_USERDATA) }
        as *mut PinnedGuiThreadHandle<'evh>;

    if handle.is_null() {
        // defer to the default
        return unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };
    }

    // try to get the default window proc
    let handle = unsafe { Box::from_raw(handle) };
    let default_proc = handle.with(|controller| {
        controller
            .subclass(unsafe { NonZeroUsize::new_unchecked(hwnd.as_ptr() as usize) })
            .unwrap_or(winuser::DefWindowProcA)
    });

    let res = match exchange_event(hwnd, msg, wparam, lparam, &handle) {
        Some(res) => res,
        None => unsafe { default_proc(hwnd.as_ptr(), msg, wparam, lparam) },
    };

    if msg != winuser::WM_DESTROY {
        // avoid dropping handle
        mem::forget(handle);
    }

    res
}

#[inline]
fn exchange_event<'evh>(
    hwnd: NonNull<HWND__>,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    window_data: &PinnedGuiThreadHandle<'evh>,
) -> Option<LRESULT> {
    let window = Window::from_ptr_nn(hwnd.cast());

    match msg {
        winuser::WM_CLOSE => {
            handle_event(window_data, Event::Close(window));
            unsafe { winuser::DestroyWindow(hwnd.as_ptr()) };
            return Some(0);
        }
        winuser::WM_DESTROY => {
            // decrement the window counter by one
            let window_count = window_data.with(|window_data| window_data.decrement_window_count());

            // if the window count is zero, send the quit message to the thread
            if window_count == 0 {
                handle_event(window_data, Event::Quit);
                unsafe { winuser::PostQuitMessage(0) };
            }

            return Some(0);
        }
        winuser::WM_MOVE => {
            // get the x and y coordinates
            let x = LOWORD(lparam as _);
            let y = HIWORD(lparam as _);

            handle_event(
                window_data,
                Event::Move {
                    window,
                    x: x as _,
                    y: y as _,
                },
            );
        }
        winuser::WM_SIZE => {
            // get the width and height
            let width = LOWORD(lparam as _);
            let height = HIWORD(lparam as _);

            // figure out our reason
            let reason = match wparam as _ {
                winuser::SIZE_MAXHIDE => SizeReason::MaxHide,
                winuser::SIZE_MAXIMIZED => SizeReason::Maximized,
                winuser::SIZE_MINIMIZED => SizeReason::Minimized,
                winuser::SIZE_MAXSHOW => SizeReason::MaxShow,
                _ => SizeReason::None,
            };

            handle_event(
                window_data,
                Event::Size {
                    window,
                    width: width as _,
                    height: height as _,
                    reason,
                },
            );
        }
        winuser::WM_ACTIVATE => {
            let other = Window::from_ptr(lparam as *mut ());
            handle_event(
                window_data,
                match wparam as _ {
                    winuser::WA_ACTIVE => Event::Activate {
                        window,
                        with_mouse_click: false,
                        deactivated: other,
                    },
                    winuser::WA_CLICKACTIVE => Event::Activate {
                        window,
                        with_mouse_click: true,
                        deactivated: other,
                    },
                    _ => Event::Deactivate {
                        window,
                        activated: other,
                    },
                },
            );
        }
        winuser::WM_SETFOCUS => {
            handle_event(
                window_data,
                Event::SetFocus {
                    window,
                    focus_loser: Window::from_ptr(wparam as *mut ()),
                },
            );
        }
        winuser::WM_KILLFOCUS => {
            handle_event(
                window_data,
                Event::KillFocus {
                    window,
                    focus_gainer: Window::from_ptr(wparam as *mut ()),
                },
            );
        }
        winuser::WM_ENABLE => {
            handle_event(
                window_data,
                match wparam as _ {
                    0 => Event::Disabled(window),
                    _ => Event::Enabled(window),
                },
            );
        }
        winuser::WM_PAINT => {
            // begin the painting process
            let mut ps = MaybeUninit::<winuser::PAINTSTRUCT>::uninit();
            let dc = unsafe { winuser::BeginPaint(hwnd.as_ptr(), ps.as_mut_ptr()) };

            //unsafe { wingdi::LineTo(dc, 300, 300) };

            // if we can't paint, not much we can do
            let dc = match Dc::from_ptr(dc.cast()) {
                Some(dc) => dc,
                None => return None,
            };

            handle_event(window_data, Event::Paint { window, dc });

            //            unsafe { wingdi::LineTo(dc.as_ptr().as_ptr().cast(), 200, 200) };

            // end the painting process
            unsafe { winuser::EndPaint(hwnd.as_ptr(), ps.as_ptr()) };

            return Some(0);
        }
        winuser::WM_KEYDOWN | winuser::WM_KEYUP | winuser::WM_SYSKEYDOWN | winuser::WM_SYSKEYUP => {
            // first 15 bytes are the repeat count
            let repeats = lparam & 0x7FFF;
            // 24th byte is whether or not we should extend it
            let extended = lparam & (0x1 << 23) != 0;

            let key = convert_vkey(wparam as _, extended);

            handle_event(
                window_data,
                match msg {
                    winuser::WM_KEYDOWN | winuser::WM_SYSKEYDOWN => Event::KeyDown {
                        window,
                        key,
                        repeats: repeats as _,
                    },
                    winuser::WM_KEYUP | winuser::WM_SYSKEYUP => Event::KeyUp {
                        window,
                        key,
                        repeats: repeats as _,
                    },
                    _ => unreachable!(),
                },
            );
        }
        _ => (),
    }

    None
}

#[inline]
fn handle_event<'evh>(window_data: &PinnedGuiThreadHandle<'evh>, event: Event) {
    // note: this should always be on the gui thread
    if window_data.process_event(event).is_err() {
        log::error!("Failed to process event: not in bread thread");
    }
}
