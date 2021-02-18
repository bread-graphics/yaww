// MIT/Apache2 License

#![cfg(feature = "direct2d")]

use crate::server::GuiThread;
use std::{
    any::type_name,
    fmt,
    marker::PhantomData,
    mem, ops,
    ptr::{self, NonNull},
};
use winapi::{um::unknwnbase::IUnknown, Interface};

// Note: I've looked through the docs, and I'm about 99% sure this wouldn't really block anything

/// An owned pointer that releases its object on drop.
pub(crate) struct Com<T> {
    // the pointer proper
    ptr: NonNull<T>,
    // a handle on the GUI thread so we can still do things asynchronously
    server: GuiThread,

    _marker: PhantomData<T>,
}

struct ThreadSafe<T>(pub T);

unsafe impl Send for ThreadSafe {}

impl<T> Com<T> {
    #[inline]
    pub unsafe fn new(ptr: NonNull<T>, gt: &GuiThread) -> Self
    where
        T: Interface,
    {
        Self {
            ptr,
            server: gt.as_inferior(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Upcast this pointer.
    #[inline]
    pub fn upcast<U>(self) -> Com<U>
    where
        T: ops::Deref<Target = U>,
        U: Interface,
    {
        let Com { ptr, server } = self;
        unsafe { Com::new(ptr.cast(), server) }
    }

    /// Cast to an IUnknown.
    #[inline]
    fn as_unknown(&self) -> &IUnknown {
        unsafe { &*(p.0 as *mut IUnknown) }
    }
}

impl<T> fmt::Debug for Com<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {:p}", type_name::<T>(), self.ptr)
    }
}

impl<T> Deref for Com<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }
}

impl<T> Drop for Com<T> {
    #[inline]
    fn drop(&mut self) {
        // offload the task onto the GUI thread
        let p = ThreadSafe(self.ptr.as_ptr());
        // drops the task, which detaches it
        // if the server is already closed there isn't much we can do
        self.server
            .send_directive(Directive::RunFunction(move || unsafe {
                self.as_unknown().Release()
            }))
            .ok();
    }
}
