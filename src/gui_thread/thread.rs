// MIT/Apache2 License

use super::{process_directive, Directive, Event, Provider, Response};
use crate::{refcell::RefCell, wndproc::WindowData};
use event_listener::Event as LEvent;
use flume::{Receiver, Sender};
use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
    pin::Pin,
    ptr::{self, NonNull},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};
use winapi::{
    shared::minwindef::UINT,
    um::{processthreadsapi::GetCurrentThreadId, winuser},
};

// We support creating more than one GUI thread. In order to append numbers to the
// end, to differentiate the thread names, this number exists.
static GUI_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
const WM_YAWW_DIRECTIVE: UINT = winuser::WM_USER + 0x1337;

/// Thread entry point
#[inline]
pub(crate) fn create(
    recv: Receiver<Directive>,
    send: Sender<crate::Result<Response>>,
    stop_event: Arc<LEvent>,
) {
    // spawn the thread
    let index = GUI_THREAD_COUNT.fetch_add(1, Ordering::AcqRel);
    thread::Builder::new()
        .name(format!("yaww-gui-thread-{}", index))
        .spawn(move || {
            // set up a spot on the stack where the event handler will always be
            // double indirection here is used so we have a single-wide pointer no matter what
            // In order to keep santicity of data, a RefCell is used to lock down access
            // Note: since this is single threaded, we could, in theory, replace this with an UnsafeCell
            //       and just pass along the pointer to cut out the overhead of RefCell. I'll try this once
            //       this crate is in a good enough place to run benchmarks.
            let window_data = RefCell::new(WindowData {
                event_handler: Box::new(|ev| {
                    log::warn!("Event ignored: {:?}", ev);
                    Ok(())
                }),
                window_count: 0,
                waiting: false,
            });

            // translates keys to Win32 pointers and vice versa
            let mut provider = Provider::new();

            // get the current thread id so we can pass that into the receiver thread
            let sender_thread_id = unsafe { GetCurrentThreadId() };

            // spawn another thread dedicated to receiving directives and putting them on the
            // message queue
            thread::Builder::new()
                .name(format!("yaww-gui-thread-recv-{}", index))
                .spawn(move || {
                    loop {
                        let directive = match recv.recv() {
                            Ok(directive) => directive,
                            // if it's failed, just quit the thread
                            Err(_) => return,
                        };

                        // create a new message containing the directive
                        // first, put it on the heap
                        let directive = Box::new(directive);
                        let directive = Box::into_raw(directive);
                        // then, post a message in the thread with the message loop
                        unsafe {
                            winuser::PostThreadMessageA(
                                sender_thread_id,
                                WM_YAWW_DIRECTIVE,
                                directive as *mut _ as usize,
                                0,
                            );
                        }
                    }
                })
                .expect("Unable to spawn recv gui thread");

            // read in the message loop
            let mut message = MaybeUninit::<winuser::MSG>::uninit();
            loop {
                let res =
                    unsafe { winuser::GetMessageA(message.as_mut_ptr(), ptr::null_mut(), 0, 0) };
                match res {
                    -1 => unreachable!("TODO: handle errors"),
                    0 => break,
                    _ => {
                        let msg = unsafe {
                            mem::replace(&mut message, MaybeUninit::uninit()).assume_init()
                        };

                        if msg.message == WM_YAWW_DIRECTIVE {
                            // since PostThreadMessageA posts messages with a null hwnd, we have to handle it
                            // in the primary event loop without dispatching it
                            // SAFETY: see the above thread, the wparam is always a pointer to the directive
                            let directive = unsafe { Box::from_raw(msg.wParam as *mut Directive) };

                            // if we are beginning a wait cycle, say as such
                            if let Directive::BeginWait = &*directive {
                                log::trace!("Borrowing window data");
                                let mut wd = window_data.borrow_mut();
                                wd.waiting = true;
                                log::trace!("Dropping window data");
                                mem::drop(wd);
                                continue;
                            }

                            let is_empty = directive.is_empty();
                            let res = process_directive(*directive, &mut provider, &window_data);
                            if res.is_err() || !is_empty {
                                if send.send(res).is_err() {
                                    // the other end of the connection has closed, so we should shut down
                                    // shop as well
                                    // this is easy to do just by sending a quit message into the
                                    // input stream
                                    unsafe { winuser::PostQuitMessage(0) };
                                }
                            }
                        } else {
                            unsafe {
                                winuser::TranslateMessage(&msg);
                                winuser::DispatchMessageA(&msg);
                            }
                        }
                    }
                }
            }

            log::info!("Leaving GUI thread main loop...");

            stop_event.notify(std::usize::MAX);
        })
        .expect("Unable to spawn Yaww GUI Thread");
}
