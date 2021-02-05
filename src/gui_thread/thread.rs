// MIT/Apache2 License

use super::{process_directive, Directive, Event, Provider, Response};
use crate::{
    refcell::RefCell,
    wndproc::{DirectiveLock, WindowData, WindowDataExclusive},
};
use event_listener::Event as LEvent;
use flume::{Receiver, Sender};
use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
    pin::Pin,
    ptr::{self, NonNull},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
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
    directive_sender: Sender<Directive>, // for sending dummy events
    stop_event: Arc<LEvent>,
) {
    // spawn the thread
    let index = GUI_THREAD_COUNT.fetch_add(1, Ordering::AcqRel);
    thread::Builder::new()
        .name(format!("yaww-gui-thread-{}", index))
        .spawn(move || {
            let directive_lock = Arc::new(DirectiveLock {
                relax_directive_thread: AtomicBool::new(false),
                done_processing: LEvent::new(),
            });

            // window data contains data that we need to pass to the wndproc and also to the directive handler
            // In order to keep santicity of data, a RefCell is used to lock down access
            // Note: since this is single threaded, we could, in theory, replace this with an UnsafeCell
            //       and just pass along the pointer to cut out the overhead of RefCell. I'll try this once
            //       this crate is in a good enough place to run benchmarks.
            let window_data = WindowData {
                exclusive: RefCell::new(WindowDataExclusive {
                    event_handler: Box::new(|ev| {
                        log::warn!("Event ignored: {:?}", ev);
                        Ok(())
                    }),
                    window_count: 0,
                    waiting: false,
                }),
                provider: RefCell::new(Provider::new()),
                directive_lock: directive_lock.clone(),
                recv: recv.clone(),
                send: send.clone(),
                directive_send: directive_sender,
            };

            // get the current thread id so we can pass that into the receiver thread
            let sender_thread_id = unsafe { GetCurrentThreadId() };

            // spawn another thread dedicated to receiving directives and putting them on the
            // message queue
            thread::Builder::new()
                .name(format!("yaww-gui-thread-recv-{}", index))
                .spawn(move || {
                    loop {
                        if directive_lock.relax_directive_thread.load(Ordering::SeqCst) {
                            directive_lock.done_processing.listen().wait();
                            continue;
                        }

                        let directive = match recv.recv() {
                            // dummy directives are used to flush the loop
                            Ok(Directive::Dummy) => continue,
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
                    // -1 indicates an error has occurred in GetMessageA proper
                    -1 => {
                        send.send(Err(crate::Error::win32_error(Some("GetMessageA"))))
                            .ok();

                        break;
                    }
                    // 0 indicates we have received WM_QUIT and should exit
                    0 => break,
                    _ => {
                        // SAFETY: at this point the message is guaranteed to be initialized
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
                                let mut wd = window_data.exclusive.borrow_mut();
                                wd.waiting = true;
                                log::trace!("Dropping window data");
                                mem::drop(wd);
                                continue;
                            }

                            handle_directive_processing(*directive, &window_data);
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

/// Note: this function doesn't handle BeginWait (even though in any sane configuration it
///       shouldn't need to)
#[inline]
pub(crate) fn handle_directive_processing(directive: Directive, window_data: &WindowData) {
    let is_empty = directive.is_empty();
    let res = process_directive(directive, window_data);
    if res.is_err() || !is_empty {
        if window_data.send.send(res).is_err() {
            // the other end of the connection has closed, so we should shut down
            // shop as well
            // this is easy to do just by sending a quit message into the
            // input stream
            unsafe { winuser::PostQuitMessage(0) };
        }
    }
}
