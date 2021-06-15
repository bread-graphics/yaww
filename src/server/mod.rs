// MIT/Apache2 License

mod thread;

use crate::{
    directive::Directive,
    event::Event,
    future,
    task::{self, ServerTask, Task},
    util::DebugContainer,
};
use flume::{Receiver, Sender};
use std::{any::Any, future::Future};

pub(crate) use thread::DirectiveThreadMessage;

#[derive(Debug, Clone)]
pub struct GuiThread {
    // sends the requests to the server
    task_sender: Sender<Option<ServerTask>>,
}

impl GuiThread {
    /// Create a new GuiThread. This spawns the threads necessary to run yaww.
    #[inline]
    pub fn new() -> Self {
        let (task_sender, task_receiver) = flume::unbounded();

        thread::create(task_sender.clone(), task_receiver);
        Self { task_sender }
    }

    /// Create an inferior copy of the GuiThread.
    #[inline]
    pub(crate) fn inferior_copy(task_sender: Sender<Option<ServerTask>>) -> Self {
        Self { task_sender }
    }

    #[inline]
    pub(crate) fn into_inner(self) -> Sender<Option<ServerTask>> {
        self.task_sender
    }

    #[cfg(feature = "direct2d")]
    #[inline]
    pub fn as_inferior(&self) -> Self {
        Self {
            task_sender: self.task_sender.clone(),
        }
    }

    /// Send a directive to the GUI thread and get a task bask to wait on.
    #[inline]
    pub(crate) fn send_directive<T: Any + Send + 'static>(
        &self,
        directive: Directive,
    ) -> crate::Result<Task<T>> {
        // create the task/servertask pair
        let (tsk, server_tsk) = unsafe { task::create_task::<T>(directive) };
        // send the server task to the server for processing
        // note: should never block, channel should never be over capacity
        self.task_sender
            .try_send(Some(server_tsk))
            .map_err(|_| crate::Error::ServerClosed)?;
        // return the client task to the user
        Ok(tsk)
    }

    /// Set the event handler.
    #[inline]
    pub fn set_event_handler<F: FnMut(&GuiThread, Event) + Send + 'static>(
        &self,
        f: F,
    ) -> crate::Result<Task<()>> {
        self.send_directive(Directive::SetEventHandler(DebugContainer::new(Box::new(f))))
    }

    /// Set the event handler to be an async function.
    ///
    /// Note that the future that the function returns should probably be spawned.
    #[inline]
    pub fn set_async_event_handler<
        Fut: Future<Output = ()>,
        F: FnMut(&GuiThread, Event) -> Fut + Send + 'static,
    >(
        &self,
        mut f: F,
    ) -> crate::Result<Task<()>> {
        self.set_event_handler(move |gt, ev| future::block_on(f(gt, ev)))
    }

    /// Wait for the event loop to complete.
    #[inline]
    pub fn main_loop(mut self) -> crate::Result<Task<()>> {
        // send a directive that tells the gui thread we're waiting
        self.send_directive::<()>(Directive::BeginWait)
    }
}
