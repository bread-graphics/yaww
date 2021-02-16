// MIT/Apache2 License

use crate::directive::Directive;
use flume::{r#async::RecvFut, Receiver, Sender};
use std::{
    any::Any,
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll, Waker},
};

pub struct Task<T: 'static>(TaskInner<T>);

enum TaskInner<T: 'static> {
    Recv(Receiver<T>),
    RFuture(RecvFut<'static, T>),
    Transitional,
}

pub(crate) struct ServerTask {
    inner: Box<dyn ServerTaskInnerGeneric + Send + Sync + 'static>,
}

struct ServerTaskInner<T: 'static> {
    send: Sender<T>,
    directive: Option<Directive>,
}

trait ServerTaskInnerGeneric {
    fn complete(&self, item: Box<dyn Any + Send + Sync + 'static>);
    fn directive(&mut self) -> Directive;
}

impl<T: Any + Send + Sync + 'static> ServerTaskInnerGeneric for ServerTaskInner<T> {
    #[inline]
    fn complete(&self, item: Box<dyn Any + Send + Sync + 'static>) {
        let item: T = match Box::<dyn Any + Send + 'static>::downcast::<T>(item) {
            Ok(item) => *item,
            Err(_) => panic!("Item type mismatch"),
        };

        self.send.try_send(item).ok();
    }

    #[inline]
    fn directive(&mut self) -> Directive {
        self.directive.take().unwrap()
    }
}

impl<T: Any + Send + Sync + 'static> Task<T> {
    #[inline]
    pub fn wait(self) -> T {
        match self.0 {
            TaskInner::Recv(r) => r.recv().unwrap(),
            _ => unreachable!(),
        }
    }
}

impl<T: Any + Send + Sync + 'static> Future for Task<T> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let this = self.get_mut();
        loop {
            match this.0 {
                TaskInner::Recv(ref mut r) => {
                    let m = mem::replace(&mut this.0, TaskInner::Transitional);
                    match m {
                        TaskInner::Recv(r) => *this = Task(TaskInner::RFuture(r.into_recv_async())),
                        _ => unreachable!(),
                    }
                }
                TaskInner::RFuture(ref mut r) => {
                    let mut p = Pin::new(r);
                    return match p.as_mut().poll(cx) {
                        Poll::Ready(p) => Poll::Ready(p.unwrap()),
                        Poll::Pending => Poll::Pending,
                    };
                }
                _ => unreachable!(),
            }
        }
    }
}

impl ServerTask {
    #[inline]
    pub fn complete<T: Any + Send + Sync + 'static>(self, item: T) {
        self.inner.complete(Box::new(item));
    }

    #[inline]
    pub fn directive(&mut self) -> Directive {
        self.inner.directive()
    }
}

pub(crate) fn create_task<T: Any + Send + Sync + 'static>(
    directive: Directive,
) -> (Task<T>, ServerTask) {
    let (send, recv) = flume::bounded(1);
    let task = Task(TaskInner::Recv(recv));
    let srvtask = ServerTask {
        inner: Box::new(ServerTaskInner {
            send,
            directive: Some(directive),
        }),
    };
    (task, srvtask)
}
