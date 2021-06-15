// MIT/Apache2 License

use std::{
    future::Future,
    mem::{self, ManuallyDrop},
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    thread::{self, Thread},
};

/// A simple future blocking mechanism that blocks the current thread for the future. The reason why we put this
/// here instead of importing it from somewhere else is because "somewhere else", as far as I know, always pulls
/// in a few extra dependencies for what's obstensibly a single function.
#[inline]
pub fn block_on(mut future: impl Future<Output = ()>) {
    // first, pin the future to the stack
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // then, create a waker and a context to go with that waker
    // the waker takes a pointer to the thread to unpark
    // this is safe because the waker does not outlive the thread
    let th = thread::current();
    let waker = RawWaker::new(&th as *const Thread as *const _, &RAW_WAKER_VTABLE);
    let waker = unsafe { Waker::from_raw(waker) };
    let mut ctx = Context::from_waker(&waker);

    // TODO: figure out if we need to defeat spurious wakeups
    loop {
        match future.as_mut().poll(&mut ctx) {
            Poll::Ready(()) => break,
            Poll::Pending => thread::park(),
        }
    }

    // ensure that the waker does not outlive the thread
    mem::drop(ctx);
    mem::drop(th);
}

const RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(bo_clone, bo_wake, bo_wake_by_ref, bo_drop);

unsafe fn bo_clone(this: *const ()) -> RawWaker {
    RawWaker::new(this, &RAW_WAKER_VTABLE)
}

unsafe fn bo_wake(this: *const ()) {
    (*(this as *const Thread)).unpark();
}

unsafe fn bo_wake_by_ref(this: *const ()) {
    (*(this as *const Thread)).unpark();
}

unsafe fn bo_drop(this: *const ()) {}
