// MIT/Apache2 License

use crate::{task::ServerTask, window_data::WindowData};
use flume::{Receiver, Sender, TryRecvError};
use std::{
    cell::{Cell, RefCell},
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};
use winapi::{
    shared::minwindef::UINT,
    um::{processthreadsapi, winuser},
};

static THREAD_INDEX: AtomicUsize = AtomicUsize::new(0);
const WM_YAWW_SRVTASK: UINT = winuser::WM_USER + 0x1337;

#[inline]
pub(crate) fn create(sender: Sender<Option<ServerTask>>, recv: Receiver<Option<ServerTask>>) {
    let index = THREAD_INDEX.fetch_add(1, Ordering::AcqRel);

    // spawn the gui thread
    thread::Builder::new()
        .name(format!("yaww-gui-thread-{}", index))
        .spawn(move || {
            // get the current thread's ID so we can push messages here from the other thread
            let thread_id = unsafe { processthreadsapi::GetCurrentThreadId() };

            // a thread to send stop or start signals to the directive processing thread
            let (dt_action, dt_recv) = flume::unbounded::<DirectiveThreadMessage>();

            // the window data that's passed around between windows
            // this is kept in thread memory that's valid for as long as wndprocs are being run
            let window_data = WindowData {
                window_count: Cell::new(0),
                waiter: RefCell::new(None),
                dt_action,
                task_send: sender,
                task_recv: recv.clone(),
                event_handler: RefCell::new(Box::new(|_, ev| {
                    log::warn!("Event ignored: {:?}", ev)
                })),
            };

            // start the directive processing thread. this thread pushes new server tasks onto the
            // message queue, until it receives a message telling it to stop
            thread::Builder::new()
                .name(format!("yaww-directive-thread-{}", index))
                .spawn(move || {
                    'dtloop: loop {
                        // first, try to get data from the special signal queue
                        match dt_recv.try_recv() {
                            // if the queue is empty, move on to processing directives
                            Err(TryRecvError::Empty) => (),
                            // if the queue is dead, the parent thread has likely terminated, or we're being told
                            // to terminate
                            Err(TryRecvError::Disconnected) => break 'dtloop,
                            // if we're being told to start without stopping, just keep going
                            Ok(DirectiveThreadMessage::Start) => (),
                            // if we're being told to stop, wait for a start signal
                            Ok(DirectiveThreadMessage::Stop) => 'stoploop: loop {
                                // wait for the next message by blocking the thread
                                match dt_recv.recv() {
                                    // if the queue is dead, terminate the thread
                                    Err(_) => break 'dtloop,
                                    // if we're being told to stop again, just continue
                                    Ok(DirectiveThreadMessage::Stop) => (),
                                    // if we're being told to start, break the stoploop and continue the dtloop
                                    Ok(DirectiveThreadMessage::Start) => break 'stoploop,
                                }
                            },
                        }

                        // now, load a server-side task from the receiver
                        let srvtask = match recv.recv() {
                            Ok(Some(d)) => d,
                            Ok(None) => {
                                // this is a dummy task, used to force this thread to process the above loop
                                continue 'dtloop;
                            }
                            Err(_) => break 'dtloop,
                        };

                        // put the task on the heap to pass across the FFI boundary
                        let srvtask = Box::new(srvtask);
                        let taskraw = Box::into_raw(srvtask);

                        // send the task to the main thread by posting it in the message queue
                        unsafe {
                            winuser::PostThreadMessageA(thread_id, WM_YAWW_SRVTASK, 0, taskraw as _)
                        };
                    }
                })
                .expect("Failed to spawn directive thread");

            // begin processing messages on the message queue
            let mut msg = MaybeUninit::<winuser::MSG>::uninit();
            loop {
                // get the message
                let res = unsafe { winuser::GetMessageA(msg.as_mut_ptr(), ptr::null_mut(), 0, 0) };

                match res {
                    -1 => {
                        // an error occurred during GetMessageA
                        unimplemented!("TODO: handle error");
                    }
                    0 => {
                        // we just got the quit message and we need to break
                        break;
                    }
                    _ => {
                        // get the message
                        let msg = unsafe { msg.assume_init() };

                        if msg.message == WM_YAWW_SRVTASK {
                            // reconstruct the task from the lparam pointer
                            // SAFETY: unchecked because if we have YAWW_SRVTASK we know it's us
                            let taskptr = msg.lParam as *mut ServerTask;
                            let task = unsafe { Box::from_raw(taskptr) };

                            // try to get the directive from the task
                            let directive = task.directive();

                            // process the directive
                            directive.process(&window_data, *task);
                        } else {
                            // translate and dispatch the message
                            unsafe {
                                winuser::TranslateMessage(&msg);
                                winuser::DispatchMessageA(&msg);
                            }
                        }
                    }
                }
            }

            // before we die, unwait the thread, absorb the error
            let mut waiter = window_data.waiter.borrow_mut();
            if let Some(waiter) = waiter.take() {
                waiter.complete::<()>(());
            }
            mem::drop(waiter);
        })
        .expect("Failed to spawn GUI thread");
}

#[derive(Copy, Clone)]
pub(crate) enum DirectiveThreadMessage {
    Stop,
    Start,
}
