// MIT/Apache2 License

// Requires the "async" feature

use async_executor::Executor;
use async_io::{block_on, Timer};
use easy_parallel::Parallel;
use flume::{Receiver, Sender};
use futures_lite::{
    future::{self, Future},
    prelude::*,
};
use std::{ffi::CStr, mem, ops, pin::Pin, time};
use yaww::{
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, WindowStyle},
    GuiThread, Result,
};

const CLASS_NAME: ConstCstr<'static> = ConstCstr::new(&*b"examples_basic_class\0");
const WINDOW_NAME: ConstCstr<'static> = ConstCstr::new(&*b"YAWW Example\0");

#[inline]
async fn gui(recv: Receiver<()>) -> Result {
    let gui_thread = GuiThread::new();
    gui_thread
        .register_class_async(
            &*CLASS_NAME,
            None,
            ClassStyle::empty(),
            None,
            None,
            None,
            None,
        )
        .await?;
    let win = gui_thread
        .create_window_async(
            &*CLASS_NAME,
            &*WINDOW_NAME,
            WindowStyle::OVERLAPPED_WINDOW,
            ExtendedWindowStyle::empty(),
            0,
            0,
            640,
            400,
            None,
            None,
        )
        .await?;
    win.show_async(&gui_thread, ShowWindowCommand::SHOW).await?;
    gui_thread
        .set_event_handler_async(|_| future::ready(Ok(())))
        .await?;

    future::or(gui_thread.clone().wait_async(), async {
        loop {
            recv.recv_async().await.unwrap();
            win.move_window_async(&gui_thread, 20, 20, 640, 400, true)
                .await
                .unwrap();
        }
    })
    .await
}

#[inline]
async fn timer(send: Sender<()>) {
    let mut timer = Timer::interval(time::Duration::from_secs(2));

    loop {
        // wait two seconds
        timer.next().await;

        // send a message to the window
        if send.send_async(()).await.is_err() {
            break;
        }
    }
}

#[inline]
async fn entry(ex: &Executor<'_>) -> Result {
    let (send, recv) = flume::unbounded();
    let task1 = ex.spawn(gui(recv));
    let task2 = ex.spawn(timer(send));
    future::zip(task1, task2).await.0
}

fn main() -> Result {
    let ex = Executor::new();
    let (signal, shutdown) = flume::unbounded::<()>();

    Parallel::new()
        .each(0..2, |_| block_on(ex.run(shutdown.recv_async())))
        .finish(|| {
            block_on(async {
                let res = entry(&ex).await;
                mem::drop(signal);
                res
            })
        })
        .1?;

    Ok(())
}

struct ConstCstr<'a> {
    inner: &'a [u8],
}

impl<'a> ConstCstr<'a> {
    #[inline]
    const fn new(c: &'a [u8]) -> Self {
        Self { inner: c }
    }
}

impl<'a> ops::Deref for ConstCstr<'a> {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        CStr::from_bytes_with_nul(self.inner).unwrap()
    }
}
