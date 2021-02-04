// MIT/Apache2 License

use std::{cell, ops};

/// A reference cell that logs its usage.
#[repr(transparent)]
pub(crate) struct RefCell<T>(cell::RefCell<T>);

impl<T> RefCell<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self(cell::RefCell::new(val))
    }

    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        #[cfg(debug_assertions)]
        log::trace!("Borrowed refcell immutably");

        Ref(self.0.borrow())
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        #[cfg(debug_assertions)]
        log::trace!("Borrowed refcell mutably");

        RefMut(self.0.borrow_mut())
    }
}

#[repr(transparent)]
pub(crate) struct Ref<'a, T>(cell::Ref<'a, T>);

impl<'a, T> ops::Deref for Ref<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

#[cfg(debug_assertions)]
impl<'a, T> Drop for Ref<'a, T> {
    #[inline]
    fn drop(&mut self) {
        log::trace!("Dropped refcell immutable borrow");
    }
}

#[repr(transparent)]
pub(crate) struct RefMut<'a, T>(cell::RefMut<'a, T>);

impl<'a, T> ops::Deref for RefMut<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T> ops::DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

#[cfg(debug_assertions)]
impl<'a, T> Drop for RefMut<'a, T> {
    #[inline]
    fn drop(&mut self) {
        log::trace!("Dropped refcell mutable borrow");
    }
}
