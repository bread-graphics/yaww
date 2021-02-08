// MIT/Apache2 License

/*

The problem with Win32 winuser APIs is that they are not thread-safe. If we call winuser
functions outside of the thread the window was created on, the results are unpredictable.
In order to uphold Rust's safety guarantees: we have two options:

 1). Pass on this thread unsafety to the user. This is an alright solution that I believe
     most other winapi-wrapping Rust libraries use (assuming they don't just assume the
     pointers are thread safe and thus create undefined behavior). This solution has two
     problems:
  a). You can't send windows to other threads. This is a minor convenience that could
      propogate to larger GUI frameworks having non-thread-safe primitives. See druid
      for an example of this.

      I intend to use this crate in larger GUI frameworks. breadx is completely thread
      safe, so it would be annoying to have primitives on Linux be thread safe but
      not on Windows.
  b). This means any future operating on yaww pritimitives must be !Send and !Sync. I'm
      fine with !Sync futures (I'm pretty sure most breadglx futures are !Sync because
      of how often they have to deal with raw pointers and !Sync data), but having !Send
      futures means that it can't be spawned on global executors. You'd have to use
      local executors for it, and since I want to support the async ecosystem that would
      quickly evolve into a trainwreck.
 2). Create another thread dedicated to holding the thread unsafe primitives, and use
     an interface that connects to that thread. This interface needs to send data into
     the thread (directives) and receive data in response to those directives
     (responses). In addition, there would need to be some kind of event handler for
     messages produced by the message loop.

We take option 2, since I want to support async.

*/

mod message;
mod process;
mod provider;
mod thread;

pub use message::{Directive, Event, Response, SpecialResize, Point, RasterOperation};
pub(crate) use process::process_directive;
pub use provider::Key;
pub(crate) use provider::{KeyType, Provider};
pub(crate) use thread::handle_directive_processing;

use event_listener::Event as LEvent;
use flume::{Receiver, Sender};
use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "async")]
use futures_lite::future::{self, Future};

/// Interface to the common GUI thread.
///
/// # Notes
///
/// This object is cheaply clonable, so don't worry about putting it into an `Arc`.
#[derive(Debug, Clone)]
pub struct GuiThread {
    // channel for sending and receiving
    send: Sender<Directive>,
    recv: Receiver<crate::Result<Response>>,
    // event used to tell us to stop
    stop_event: Arc<LEvent>,
}

impl GuiThread {
    /// Spawn a new GuiThread.
    #[inline]
    pub fn new() -> Self {
        let (directive_send, directive_recv) = flume::unbounded();
        let (response_send, response_recv) = flume::unbounded();
        let stop_event = Arc::new(LEvent::new());
        thread::create(
            directive_recv,
            response_send,
            directive_send.clone(),
            stop_event.clone(),
        );
        Self {
            send: directive_send,
            recv: response_recv,
            stop_event,
        }
    }

    /// Send a directive to the other thread, and block until a response is reached.
    #[inline]
    pub fn send_directive(&self, directive: Directive) -> crate::Result<Response> {
        // send the directive to the gui thread
        let is_empty = directive.is_empty();
        self.send
            .send(directive)
            .map_err(|_| crate::Error::ChannelFailed)?;
        if is_empty {
            return Ok(Response::Empty);
        }

        // receive the directive or the error
        self.recv
            .recv()
            .map_err(|_| crate::Error::ChannelFailed)
            .and_then(|r| r)
    }

    /// Send a directive to the other thread, and pend until a response is reached.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn send_directive_async(&self, directive: Directive) -> crate::Result<Response> {
        // send the directive to the gui thread
        let is_empty = directive.is_empty();
        self.send
            .send_async(directive)
            .await
            .map_err(|_| crate::Error::ChannelFailed)?;
        if is_empty {
            return Ok(Response::Empty);
        }

        // receive the directive or the error
        self.recv
            .recv_async()
            .await
            .map_err(|_| crate::Error::ChannelFailed)
            .and_then(|r| r)
    }

    /// Set the event loop for this gui thread.
    #[inline]
    pub fn set_event_handler(
        &self,
        event_handler: impl FnMut(Event) -> crate::Result + Send + 'static,
    ) -> crate::Result {
        self.send_directive(Directive::SetEventHandler(Box::new(event_handler)))?;
        Ok(())
    }

    /// Set an event handler, async redox. Note that this uses an event handler that returns a future,
    /// rather than a standard function.
    ///
    /// If you are using an executor, it is recommended to use it to spawn a hot future within the function.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_event_handler_async<Fut: Future<Output = crate::Result>>(
        &self,
        mut event_handler: impl FnMut(Event) -> Fut + Send + 'static,
    ) -> crate::Result {
        let f = move |event| future::block_on(event_handler(event));
        self.send_directive_async(Directive::SetEventHandler(Box::new(f)))
            .await?;
        Ok(())
    }

    /// Wait for the loop to end.
    #[inline]
    pub fn wait(self) -> crate::Result {
        // first, let the GUI thread know we're waiting
        self.send
            .send(Directive::BeginWait)
            .map_err(|_| crate::Error::ChannelFailed)?;

        // then, wait for the loop to end
        self.stop_event.listen().wait();

        // once the loop has ended, drain the messages and report errors
        self.recv.try_iter().try_for_each::<_, crate::Result>(|r| {
            r?;
            Ok(())
        })
    }

    /// Wait for the loop to end, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn wait_async(self) -> crate::Result {
        self.send
            .send_async(Directive::BeginWait)
            .await
            .map_err(|_| crate::Error::ChannelFailed)?;
        self.stop_event.listen().await;

        // once the loop has ended, drain the messages and report errors
        self.recv.try_iter().try_for_each::<_, crate::Result>(|r| {
            r?;
            Ok(())
        })
    }
}
