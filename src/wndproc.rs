// MIT/Apache2 License

use crate::{
    gui_thread::{handle_directive_processing, Directive, Event, Provider, Response},
    refcell::RefCell,
};
use event_listener::Event as LEvent;
use flume::{Receiver, Sender};
use std::{
    mem, panic,
    ptr::NonNull,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
};
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, HWND__},
    },
    um::winuser::*,
};

/// A "lock" on the event receiving system.
/// TODO: there's probably a better way of doing this.
pub(crate) struct DirectiveLock {
    pub relax_directive_thread: AtomicBool,
    pub done_processing: LEvent,
}

/// Data within the WindowData that demands exclusive access.
pub(crate) struct WindowDataExclusive {
    // event handler
    pub event_handler: Box<dyn FnMut(Event) -> crate::Result + Send + 'static>,
    // the current number of windows
    pub window_count: usize,
    // are we in a wait cycle?
    pub waiting: bool,
}

/// Data we expect to be available to every window.
pub(crate) struct WindowData {
    pub exclusive: RefCell<WindowDataExclusive>,
    pub provider: RefCell<Provider>,

    // tell the directive processing thread to not process any directives until we're
    // done
    pub directive_lock: Arc<DirectiveLock>,
    // a clone of the directive receiver and response sender, so we can process directives
    // and send responses after event management
    pub recv: Receiver<Directive>,
    pub send: Sender<crate::Result<Response>>,
    pub directive_send: Sender<Directive>,
}

/// Window GUI procedure.
pub(crate) unsafe extern "system" fn yaww_wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // we need to abort on panic
    struct AbortOnPanic;

    impl Drop for AbortOnPanic {
        #[inline]
        fn drop(&mut self) {
            log::error!("Panicked during wndproc, aborting...");
            std::process::abort();
        }
    }

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
    let data_pointer = GetWindowLongPtrA(hwnd.as_ptr(), GWLP_USERDATA) as *const WindowData;
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

    let _guard = AbortOnPanic;

    let res = wndproc_inner(hwnd, msg, wparam, lparam, window_data);

    mem::forget(_guard);

    match res {
        Some(res) => res,
        None => {
            // default behavior is to defer to the default window proc and let win32
            // handle important stuff
            DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam)
        }
    }
}

#[inline]
fn wndproc_inner(
    hwnd: NonNull<HWND__>,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    window_data: &WindowData,
) -> Option<LRESULT> {
    // match on the message
    match msg {
        WM_DESTROY => {
            // decrement the window count
            let mut window_data = window_data.exclusive.borrow_mut();
            window_data.window_count = window_data.window_count.saturating_sub(1);
            log::debug!("Window count is now {}", window_data.window_count);

            // if the window count has reached zero and we're in a wait cycle (read: there are no
            // more directives to the sent to the loop), send the quit message to the loop
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

#[inline]
fn use_event_loop(window_data: &WindowData, event: Event) {
    // first, tell the directive processing thread to relax
    window_data
        .directive_lock
        .relax_directive_thread
        .store(true, Ordering::SeqCst);
    if window_data.directive_send.send(Directive::Dummy).is_err() {
        log::error!("Directive thread is closed!");
        panic!("If the directive thread is closed down, then it's probably panicked");
    }

    // now that we have control over the directives, run our event handler
    let mut ex = window_data.exclusive.borrow_mut();
    if (ex.event_handler)(event).is_err() {
        unreachable!("TODO: handle errors");
    }

    // the event handler may have generated directives for us to process, so process them
    mem::drop(ex);
    window_data
        .recv
        .try_iter()
        .for_each(|dir| handle_directive_processing(dir, window_data));

    // tell the directive processing thread to start processing events again
    window_data
        .directive_lock
        .relax_directive_thread
        .store(false, Ordering::SeqCst);
    window_data
        .directive_lock
        .done_processing
        .notify_additional(1);
}
