// MIT/Apache2 License

//! This file defines a thread dedicated to running the event handlers provided by the user.

use crate::{event::Event, server::GuiThread, task::ServerTask};
use flume::{Receiver, Sender};
use once_cell::sync::Lazy;
use std::thread;

pub(crate) struct ThreadSafeEVH(pub *const (dyn Fn(&GuiThread, Event) + Send + Sync + 'static));

unsafe impl Send for ThreadSafeEVH {}

pub(crate) type TESTuple = (ThreadSafeEVH, Event, Sender<Option<ServerTask>>);

#[inline]
fn event_handler_handler(handler_receiver: Receiver<TESTuple>) {
    // call in an infinite loop
    loop {
        let (event_handler, event, sender) = match handler_receiver.recv() {
            Ok(e) => e,
            Err(_) => break,
        };

        // create a GUI thread from the sender
        let inferior = GuiThread::inferior_copy(sender);

        // call the event handler with the event
        unsafe { (&*event_handler.0)(&inferior, event) };

        // send a dummy task down the pipe
        if let Err(_) = inferior.into_inner().send(None) {
            break;
        }
    }
}

pub(crate) static EVENT_HANDLER_THREAD: Lazy<Sender<TESTuple>> = Lazy::new(|| {
    let (tes_send, tes_recv) = flume::unbounded();

    thread::Builder::new()
        .name("yaww-event-handler".to_string())
        .spawn(move || event_handler_handler(tes_recv))
        .expect("Failed to spawn event thread");
    tes_send
});
