// MIT/Apache2 License

use std::{ptr::NonNull, num::NonZeroUsize};

/// A key that maps to a certain Win32 object.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Key {
    // we pretend this opaque pointer is a key, lol
    key: NonZeroUsize,
}

impl Key {
    #[inline]
    pub(crate) unsafe fn as_ptr(self) -> NonNull<()> {
        // SAFETY: key is guaranteed to be non-zero
        unsafe { NonNull::new_unchecked(self.key.get() as *mut ()) }
    }

    #[inline]
    pub(crate) fn from_ptr_nn(ptr: NonNull<()>) -> Self { 
        // SAFETY: same as above
        Self { key: unsafe { NonZeroUsize::new_unchecked(ptr.as_ptr() as usize) } }
    }

    #[inline]
    pub(crate) fn from_ptr(ptr: *mut ()) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { key: unsafe { NonZeroUsize::new_unchecked(ptr as usize) } })
        }
    }
}
