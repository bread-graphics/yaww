// MIT/Apache2 License

use std::{fmt, ops};

/// Container to make something Debug if it isn't.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct DebugContainer<T>(pub T);

impl<T> DebugContainer<T> {
    /// Create a new DebugContainer.
    #[inline]
    pub fn new(t: T) -> Self {
        Self(t)
    }

    /// Get the inner element.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for DebugContainer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for DebugContainer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Pointer> fmt::Debug for DebugContainer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}
