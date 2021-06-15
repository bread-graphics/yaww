// MIT/Apache2 License

use crate::{
    event::Event,
    server::{DirectiveThreadMessage, GuiThread},
    task::ServerTask,
};
use flume::{Receiver, Sender};
use std::{
    cell::{Cell, RefCell},
    ptr::NonNull,
    sync::Arc,
};
use winapi::shared::{
    minwindef::{LPARAM, LRESULT, UINT, WPARAM},
    windef::HWND,
};

// data we pass around the GUI thread
pub(crate) struct WindowData {
    // the number of windows currently active
    pub window_count: Cell<usize>,
    // send directive thread messages to the directive thread
    pub dt_action: Sender<DirectiveThreadMessage>,
    // we should be able to receive server tasks in the event
    pub task_recv: Receiver<Option<ServerTask>>,
    // used to send dummy tasks
    pub task_send: Sender<Option<ServerTask>>,
    // the waiting task we notify once the loop is complete
    pub waiter: RefCell<Option<ServerTask>>,
}

// data we pass around to each window
pub(crate) struct WindowSpecific {
    pub subclass: unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT,
    pub window_data: NonNull<WindowData>,
}
