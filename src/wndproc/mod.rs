// MIT/Apache2 License

use crate::{
    dc::Dc,
    event::{Event, SizeReason},
    server::DirectiveThreadMessage,
    window::Window,
    window_data::WindowData,
};
use std::{
    mem::{self, MaybeUninit},
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

mod event;

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

    // if the handle doesn't refer to a window, defer to the default procedure
    if unsafe { winuser::IsWindow(hwnd.as_ptr()) } == 0 {
        return unsafe { winuser::DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };
    }

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
    let window = Window::from_ptr_nn(hwnd.cast());

    match msg {
        winuser::WM_CLOSE => {
            handle_event(window_data, Event::Close(window));
            unsafe { winuser::DestroyWindow(hwnd.as_ptr()) };
            return Some(0);
        }
        winuser::WM_DESTROY => {
            // decrement the window counter by one
            let window_count = window_data.window_count.get().saturating_sub(1);
            window_data.window_count.set(window_count);

            // if the window count is zero and we're in a wait cycle, send the quit message to the thread
            if window_count == 0 && window_data.waiter.borrow().is_some() {
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

            // if we can't paint, not much we can do
            let dc = match Dc::from_ptr(dc.cast()) {
                Some(dc) => dc,
                None => return None,
            };

            handle_event(window_data, Event::Paint { window, dc });

            // end the painting process
            unsafe { winuser::EndPaint(hwnd.as_ptr(), ps.as_ptr()) };

            return Some(0);
        }
        _ => (),
    }

    None
}

#[inline]
fn handle_event(window_data: &WindowData, event: Event) {
    let r = window_data.event_handler.borrow();

    // tell the directive processing thread to relax, and send a dummy message to ensure it gets the message
    window_data
        .dt_action
        .send(DirectiveThreadMessage::Stop)
        .ok();
    window_data.task_send.send(None).ok();

    // offload the event handler onto the event handling thread
    let event_handler_ptr = event::ThreadSafeEVH(&*r);
    event::EVENT_HANDLER_THREAD
        .send((event_handler_ptr, event, window_data.task_send.clone()))
        .ok();

    // begin processing events in this thread until the event handler finishes
    loop {
        match window_data.task_recv.recv() {
            // Ok(None) indicates it's time to stop
            Ok(None) | Err(_) => break,
            Ok(Some(tsk)) => {
                let directive = tsk.directive();
                directive.process(&window_data, tsk);
            }
        }
    }

    window_data
        .dt_action
        .send(DirectiveThreadMessage::Start)
        .ok();
}
