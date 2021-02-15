// MIT/Apache2 License

//! This file defines a thread dedicated to running the event handlers provided by the user.

use crate::{event::Event, server::GuiThread, task::ServerTask};
use flume::{Receiver, Sender};
use std::{sync::Arc, thread};

pub(crate) type TESTuple = (
    Arc<dyn Fn(&GuiThread, Event) + Send + Sync + 'static>,
    Event,
    Sender<Option<ServerTask>>,
);

#[inline]
pub(crate) fn event_handler_handler(t: TESTuple) {
    // call in an infinite loop
    let (event_handler, event, sender) = t;

    // create a GUI thread from the sender
    let inferior = GuiThread::inferior_copy(sender);

    // call the event handler with the event
    unsafe { (&*event_handler)(&inferior, event) };

    // send a dummy task down the pipe
    inferior.into_inner().send(None).ok();
}
