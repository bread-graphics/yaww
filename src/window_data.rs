// MIT/Apache2 License

use crate::{
    event::Event,
    server::{DirectiveThreadMessage, GuiThread},
    task::ServerTask,
};
use flume::{Receiver, Sender};
use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

// data we pass around the GUI thread
pub(crate) struct WindowData {
    // the number of windows currently active
    pub window_count: Cell<usize>,
    // send directive thread messages to the directive thread
    pub dt_action: Sender<DirectiveThreadMessage>,
    // we should be able to receive server tasks in the event
    pub task_recv: Receiver<Option<ServerTask>>,
    // container for the event handler
    pub event_handler: RefCell<Box<dyn Fn(&GuiThread, Event) + Send + 'static>>,
    // the waiting task we notify once the loop is complete
    pub waiter: RefCell<Option<ServerTask>>,
}
