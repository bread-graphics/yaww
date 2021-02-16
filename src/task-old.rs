// MIT/Apache2 License

use crate::directive::Directive;
use std::{
    any::Any,
    fmt,
    future::Future,
    mem,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread::{self, Thread},
};

enum DirectiveOrOutput<T> {
    Directive(Directive),
    Output(T),
}

enum ThreadOrWaker {
    Thread(Thread),
    Waker(Waker),
}

/// Having a task id makes it easier to identify them for debugging purposes.
#[derive(Copy, Clone)]
struct TaskId {
    #[cfg(debug_assertions)]
    id: usize,
}

impl fmt::Debug for TaskId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        {
            write!(f, "Task #{}", self.id)
        }

        #[cfg(not(debug_assertions))]
        {
            f.write_str("Task")
        }
    }
}

impl TaskId {
    #[inline]
    fn new() -> TaskId {
        #[cfg(debug_assertions)]
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        TaskId {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::AcqRel),
        }
    }
}

/// The inner container defining the task.
struct Inner<T: Any + Send + Sync + 'static> {
    id: TaskId,

    // fine using spinlocks because in most cases they shouldn't be contended
    d_or_o: Mutex<Option<DirectiveOrOutput<T>>>,
    t_or_w: Mutex<Option<ThreadOrWaker>>,
}

impl<T: Any + Send + Sync + 'static> Inner<T> {
    #[inline]
    fn is_complete(&self) -> Option<T> {
        log::trace!(
            "{:?}: Grabbing directive_or_output lock for is_complete()",
            self.id
        );
        let mut l = self.d_or_o.lock().unwrap();
        let r = match &mut *l {
            None => None,
            Some(DirectiveOrOutput::Directive(_)) => None,
            Some(DirectiveOrOutput::Output(_)) => Some(match l.take() {
                Some(DirectiveOrOutput::Output(o)) => o,
                _ => unreachable!(),
            }),
        };
        log::trace!(
            "{:?}: Dropping directive_or_output lock for is_complete()",
            self.id
        );
        r
    }

    #[inline]
    fn wake(&self) {
        log::trace!("{:?}: Grabbing thread_or_waker lock for wake()", self.id);
        let mut l = self.t_or_w.lock().unwrap();
        let res = l.take();
        log::trace!("{:?}: Dropping thread_or_waker lock for wake()", self.id);
        mem::drop(l);

        match res {
            Some(ThreadOrWaker::Thread(t)) => t.unpark(),
            Some(ThreadOrWaker::Waker(w)) => w.wake(),
            None => (),
        }
    }
}

trait InnerGeneric {
    fn directive(&self) -> Directive;
    fn set_output(&self, output: Box<dyn Any + Send + 'static>);
}

impl<T: Any + Send + Sync + 'static> InnerGeneric for Inner<T> {
    #[inline]
    fn directive(&self) -> Directive {
        log::trace!(
            "{:?}: Grabbing directive_or_output lock for directive()",
            self.id
        );
        let d = match self.d_or_o.lock().unwrap().take() {
            Some(DirectiveOrOutput::Directive(d)) => d,
            _ => panic!("Already pulled directive from task"),
        };
        log::trace!(
            "{:?}: Dropping directive_or_output lock for directive()",
            self.id
        );
        d
    }

    #[inline]
    fn set_output(&self, output: Box<dyn Any + Send + 'static>) {
        let output: T = match output.downcast::<T>() {
            Ok(output) => *output,
            Err(_) => panic!("Unable to derive the actual type of output"),
        };

        log::trace!(
            "{:?}: Grabbing directive_or_output lock for set_output()",
            self.id
        );
        *self.d_or_o.lock().unwrap() = Some(DirectiveOrOutput::Output(output));
        log::trace!(
            "{:?}: Dropping directive_or_output lock for set_output()",
            self.id
        );
        self.wake();
    }
}

/// A task represents an ongoing operation being done by the GUI server.
pub struct Task<T: Any + Send + Sync + 'static> {
    inner: Arc<Inner<T>>,
}

/// The server-facing side of the task.
pub(crate) struct ServerTask {
    // this pointer is actually an Arc<Inner<T>>, but type-erased
    inner: Arc<dyn InnerGeneric + Send + Sync + 'static>,
}

impl<T: Any + Send + Sync + 'static> Task<T> {
    /// Wait for the task to resolve to a real result by parking the current thread and waiting.
    #[inline]
    pub fn wait(self) -> T {
        // park until completion
        loop {
            if let Some(val) = self.inner.is_complete() {
                break val;
            }

            log::trace!(
                "{:?}: Grabbing thread_or_waker lock for wait()",
                self.inner.id
            );
            *self.inner.t_or_w.lock().unwrap() = Some(ThreadOrWaker::Thread(thread::current()));
            log::trace!(
                "{:?}: Dropping thread_or_waker lock for wait()",
                self.inner.id
            );
            thread::park();
        }
    }
}

impl<T: Any + Send + Sync + 'static> Future for Task<T> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        // set the waker
        if let Some(val) = self.inner.is_complete() {
            Poll::Ready(val)
        } else {
            *self.inner.t_or_w.lock().unwrap() = Some(ThreadOrWaker::Waker(cx.waker().clone()));
            Poll::Pending
        }
    }
}

impl ServerTask {
    /// Pull the directive from this task.
    #[inline]
    pub fn directive(&self) -> Directive {
        self.inner.directive()
    }

    /// Complete the task.
    #[inline]
    pub fn complete<T: Any + Send + Sync + 'static>(self, val: T) {
        self.inner.set_output(Box::new(val));
    }
}

pub(crate) fn create_task<T: Any + Send + Sync + 'static>(
    directive: Directive,
) -> (Task<T>, ServerTask) {
    let inner = Arc::new(Inner {
        id: TaskId::new(),
        d_or_o: Mutex::new(Some(DirectiveOrOutput::Directive(directive))),
        t_or_w: Mutex::new(None),
    });

    let srvtask = ServerTask {
        inner: inner.clone(),
    };
    let task = Task { inner };
    (task, srvtask)
}
