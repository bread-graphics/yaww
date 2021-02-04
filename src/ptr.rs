// MIT/Apache2 License

use std::{borrow, marker::PhantomData, mem, ops, ptr::NonNull};
use winapi::ctypes::c_void;

/// An opaque object that can be accessed via the opaque pointers in this module.
pub trait Opaque {
    fn destroy(ptr: *mut c_void);
}

/// An opaque object that can be cloned.
pub trait ClonableOpaque: Opaque {
    fn clone(ptr: *mut c_void) -> OwnedPtr<Self>;
}

/// An opaque object that can be accessed from outside of its native thread.
pub unsafe trait SendableOpaque: Opaque {}

/// An opaque object that can be accessed from multiple threads at a time.
pub unsafe trait SyncOpaque: Opaque {}

/// An owned pointer to an opaque object.
#[repr(transparent)]
pub struct OwnedPtr<T: Opaque + ?Sized> {
    inner: NonNull<c_void>,
    _marker: PhantomData<T>,
}

/// An unowned pointer to an opaque object.
#[repr(transparent)]
pub struct UnownedPtr<'a, T: Opaque + ?Sized> {
    _inner: NonNull<c_void>,
    _marker: PhantomData<&'a T>,
}

impl<T: Opaque + ?Sized> OwnedPtr<T> {
    #[inline]
    pub unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> NonNull<c_void> {
        self.inner
    }

    /// Convert this value to its inner value, without calling the destructor.
    #[inline]
    pub unsafe fn into_inner(self) -> NonNull<c_void> {
        let inner = self.inner;
        mem::forget(self);
        inner
    }

    #[inline]
    pub fn get_ref<'a, 'b>(&'a self) -> UnownedPtr<'b, T>
    where
        'a: 'b,
    {
        // SAFETY: since we statically guarantee that the unowned pointer doesn't outlive
        //         the owned pointer, the data here should always be valid
        unsafe { UnownedPtr::from_raw(self.inner) }
    }
}

impl<T: ClonableOpaque + ?Sized> Clone for OwnedPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        T::clone(self.inner.as_ptr())
    }
}

// SAFETY: The caller has to guarantee that the interior type can be sent.
unsafe impl<T: SendableOpaque + ?Sized> Send for OwnedPtr<T> {}
unsafe impl<T: SyncOpaque + ?Sized> Sync for OwnedPtr<T> {}

impl<T: Opaque + ?Sized> Drop for OwnedPtr<T> {
    #[inline]
    fn drop(&mut self) {
        T::destroy(self.inner.as_ptr());
    }
}

impl<'a, T: Opaque + ?Sized> UnownedPtr<'a, T> {
    #[inline]
    pub unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self {
            _inner: ptr,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Opaque + ?Sized> Copy for UnownedPtr<'a, T> {}

impl<'a, T: Opaque + ?Sized> Clone for UnownedPtr<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: References are only Send if the interior object is Sync.
unsafe impl<'a, T: SyncOpaque + ?Sized> Send for UnownedPtr<'a, T> {}

impl<'a, T: Opaque + ?Sized> borrow::Borrow<OwnedPtr<T>> for UnownedPtr<'a, T> {
    #[inline]
    fn borrow(&self) -> &OwnedPtr<T> {
        &*self
    }
}

impl<'a, T: Opaque + ?Sized> ops::Deref for UnownedPtr<'a, T> {
    type Target = OwnedPtr<T>;

    #[inline]
    fn deref(&self) -> &OwnedPtr<T> {
        // SAFETY: since both UnownedPtr and OwnedPtr are repr(transparent) with a NonNull<c_void>, this
        // doesn't actually do anything bad
        unsafe { &*(self as *const UnownedPtr<'a, T> as *const OwnedPtr<T>) }
    }
}
