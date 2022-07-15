//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

#[cfg(feature = "alloc")]
mod internal {
    #[doc(no_inline)]
    pub use alloc::borrow::Cow;
}

#[cfg(not(feature = "alloc"))]
mod internal {
    use core::{borrow::Borrow, ops::Deref};

    /// A replacement for `Cow` for platforms without allocators.
    pub enum Cow<'a, T: ?Sized> {
        Borrowed(&'a T),
    }

    impl<'a, T: ?Sized> Clone for Cow<'a, T> {
        fn clone(&self) -> Self {
            match self {
                Self::Borrowed(r) => Self::Borrowed(r),
            }
        }
    }

    impl<'a, T: ?Sized> From<&'a T> for Cow<'a, T> {
        fn from(r: &'a T) -> Self {
            Self::Borrowed(r)
        }
    }

    impl<'a, T: ?Sized> AsRef<T> for Cow<'a, T> {
        fn as_ref(&self) -> &T {
            match self {
                Self::Borrowed(r) => r,
            }
        }
    }

    impl<'a, T: ?Sized> Borrow<T> for Cow<'a, T> {
        fn borrow(&self) -> &T {
            self.as_ref()
        }
    }

    impl<'a, T: ?Sized> Deref for Cow<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }
}

pub use internal::Cow;
