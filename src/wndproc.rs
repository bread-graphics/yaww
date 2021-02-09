// MIT/Apache2 License

use crate::{
    gui_thread::{
        handle_directive_processing, Directive, Event, KeyType, Provider, Response, SpecialResize,
    },
    refcell::RefCell,
    window::Window,
};
use event_listener::Event as LEvent;
use flume::{Receiver, Sender};
use std::{
    cell::Cell,
    mem::{self, MaybeUninit},
    panic,
    ptr::NonNull,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
};
use winapi::{
    shared::{
        minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HWND, HWND__},
    },
    um::winuser::*,
};

/// A "lock" on the event receiving system.
/// TODO: there's probably a better way of doing this.
pub(crate) struct DirectiveLock {
    pub is_done_processing: AtomicBool,
    pub done_processing: LEvent,
    pub is_directive_thread_relaxed: AtomicBool,
    pub directive_thread_relaxed: LEvent,
}

/// Data we expect to be available to every window.
pub(crate) struct WindowData {
    // translates keys to win32 pointers. we make sure this is never reentrant
    pub provider: RefCell<Provider>,

    // the event handler. this needs to be an Fn() instead of an FnMut(), since the event handling process
    // can sometimes be reentrant.
    pub event_handler: RefCell<Arc<dyn Fn(Event) -> crate::Result + Send + Sync + 'static>>,

    // the current number of windows
    pub window_count: Cell<usize>,
    // are we in a wait cycle?
    pub waiting: Cell<bool>,

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
    // translate this hwnd to a window key
    let window = unsafe { Window::from_pointer(hwnd.cast(), KeyType::Window) };

    // match on the message
    match msg {
        WM_DESTROY => {
            // while we're here we also get rid of the key in the provider
            window_data.provider.borrow_mut().remove_key(hwnd.cast());

            // decrement the window count
            let window_count = window_data.window_count.get().saturating_sub(1);
            window_data.window_count.set(window_count);
            log::debug!("Window count is now {}", window_count);

            // if the window count has reached zero and we're in a wait cycle (read: there are no
            // more directives to the sent to the loop), send the quit message to the loop
            // NOTE: on rustc v1.51.0 either I or the compiler is fucking up and omitting the call to check
            //       waiting, log::trace! gets around that
            log::trace!("Are we waiting? {}", window_data.waiting.get());
            if window_data.waiting.get() && window_count == 0 {
                log::debug!("Posting quit message into queue");
                use_event_loop(window_data, Event::Quit);
                unsafe { PostQuitMessage(0) };
                return Some(0);
            }
        }
        WM_CLOSE => {
            // notify of a close event
            use_event_loop(window_data, Event::Close(window));

            // destroy the window once it is closed
            unsafe { DestroyWindow(hwnd.as_ptr()) };
        }
        WM_MOVE => {
            let x = LOWORD(lparam as _);
            let y = HIWORD(lparam as _);
            use_event_loop(window_data, Event::Move { window, x, y });
        }
        WM_SIZE => {
            let width = LOWORD(lparam as _);
            let height = HIWORD(lparam as _);
            let special = match wparam {
                SIZE_MINIMIZED => Some(SpecialResize::Minimized),
                SIZE_MAXIMIZED => Some(SpecialResize::Maximized),
                SIZE_MAXHIDE => Some(SpecialResize::MaxHide),
                SIZE_MAXSHOW => Some(SpecialResize::MaxShow),
                _ => None,
            };

            use_event_loop(
                window_data,
                Event::Size {
                    window,
                    width,
                    height,
                    special,
                },
            );
        }
        WM_ACTIVATE => {
            let event = match wparam as _ {
                WA_ACTIVE => Event::Activate {
                    window,
                    from_mouse_click: false,
                },
                WA_CLICKACTIVE => Event::Activate {
                    window,
                    from_mouse_click: true,
                },
                WA_INACTIVE => Event::Deactivate(window),
                _ => return None, // server error?
            };

            use_event_loop(window_data, event);
        }
        WM_SETFOCUS => {
            use_event_loop(window_data, Event::SetFocus(window));
        }
        WM_KILLFOCUS => {
            use_event_loop(window_data, Event::KillFocus(window));
        }
        WM_ENABLE => {
            use_event_loop(
                window_data,
                if wparam == 0 {
                    Event::Disable(window)
                } else {
                    Event::Enable(window)
                },
            );
        }
        WM_PAINT => {
            // run DefWindowProcA ahead of time, so the background is painted already
            unsafe { DefWindowProcA(hwnd.as_ptr(), msg, wparam, lparam) };

            // create the painter object
            let mut ps = MaybeUninit::<PAINTSTRUCT>::uninit();
            let hdc = unsafe { BeginPaint(hwnd.as_ptr(), ps.as_mut_ptr()) };
            let hdc = match NonNull::new(hdc) {
                Some(hdc) => hdc,
                // we can't do anything with a null dc, just return if this is the case
                None => return Some(0),
            };

            // convert the HDC to a key
            let drawer = window_data
                .provider
                .borrow_mut()
                .create_key(hdc.cast(), KeyType::Dc, false)
                .unwrap();
            use_event_loop(window_data, Event::Paint { window, dc: drawer });
            window_data.provider.borrow_mut().remove_key(hdc.cast());

            // end the paint now
            unsafe { EndPaint(hwnd.as_ptr(), ps.as_ptr()) };

            // suppress the DefWindowProcA at the end
            return Some(0);
        }
        _ => (),
    }

    None
}

#[inline]
fn use_event_loop(window_data: &WindowData, event: Event) {
    // first, tell the directive processing thread to relax
    if window_data
        .directive_send
        .send(Directive::RelaxDirectiveThread)
        .is_err()
    {
        log::error!("Directive thread is closed!");
        panic!("If the directive thread is closed down, then it's probably panicked");
    }

    // wait until the directive thread acknowledges our power
    loop {
        if window_data
            .directive_lock
            .is_directive_thread_relaxed
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            break;
        }

        let listener = window_data.directive_lock.directive_thread_relaxed.listen();

        if window_data
            .directive_lock
            .is_directive_thread_relaxed
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            break;
        }

        listener.wait();
    }

    // now that we have control over the directives, run our event handler
    // note that we need to run it in a separate thread, since we're doing event processing on this thread
    // and we don't want to block unless we have to
    // to this end, clone the directive sender so we can send the DeferEventProcessing directive and stop
    // the loop below
    // note: since event_handler is an Arc, cloning it prevents the mutable access needed to set it
    let event_handler = window_data.event_handler.borrow().clone();
    let directive_send = window_data.directive_send.clone();
    let send = window_data.send.clone();

    // spawn a scoped thread, so we can borrow variables from the stack
    let handle = thread::Builder::new()
        .name(format!("yaww-event-handler-{}", {
            static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);
            THREAD_COUNTER.fetch_add(1, Ordering::AcqRel)
        }))
        .spawn(move || {
            if let Err(e) = (event_handler)(event) {
                send.send(Err(e)).expect("Agh!");
            }
            mem::drop(event_handler);
            directive_send
                .send(Directive::DeferEventProcessing)
                .unwrap();
        })
        .expect("Failed to spawn event handler thread");

    // the event handler may have generated directives for us to process, so process them
    for dir in window_data.recv.iter() {
        if let Directive::DeferEventProcessing = dir {
            break;
        } else {
            handle_directive_processing(dir, window_data);
        }
    }
    handle.join().unwrap();

    // tell the directive processing thread to start processing events again
    window_data
        .directive_lock
        .is_done_processing
        .store(true, Ordering::SeqCst);
    window_data
        .directive_lock
        .done_processing
        .notify(std::usize::MAX);
}
