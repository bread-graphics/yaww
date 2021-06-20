// MIT/Apache2 License

use crate::{dc::Dc, directive::Directive, server::SendsDirective, task::Task, Key};
use std::ptr::NonNull;
use winapi::ctypes::c_void;

pub type Glrc = Key;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GlProc {
    inner: NonNull<c_void>,
}

// SAFETY: we know glproc's only point to a single function and are thus thread-safe
unsafe impl Send for GlProc {}
unsafe impl Sync for GlProc {}

impl GlProc {
    #[inline]
    pub(crate) unsafe fn new(inner: NonNull<c_void>) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn as_ptr(self) -> *const c_void {
        self.inner.as_ptr() as *const c_void
    }
}

impl Dc {
    #[inline]
    pub fn create_wgl_context<S: SendsDirective>(
        self,
        gt: &S,
    ) -> crate::Result<Task<crate::Result<Glrc>>> {
        gt.send(Directive::CreateWglContext(self))
    }
}

pub trait WglFunctions {
    fn make_wgl_current(
        &self,
        dc: Option<Dc>,
        rc: Option<Glrc>,
    ) -> crate::Result<Task<crate::Result>>;
}

impl<S: SendsDirective> WglFunctions for S {
    #[inline]
    fn make_wgl_current(
        &self,
        dc: Option<Dc>,
        rc: Option<Glrc>,
    ) -> crate::Result<Task<crate::Result>> {
        self.send(Directive::MakeWglCurrent { dc, rc })
    }
}

impl Glrc {
    #[inline]
    pub fn delete_wgl<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<crate::Result>> {
        gt.send(Directive::DestroyWglContext(self))
    }
}
