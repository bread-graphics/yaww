//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use core::{fmt, marker::PhantomData};

struct HexWrapper(usize);

impl fmt::Debug for HexWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

macro_rules! base_impl {
    ($name: ident, $into_name: ident, $inner: ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name {
            value: $inner,
            _thread_unsafe: PhantomData<*const ()>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::empty()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&HexWrapper(self.value as usize))
                    .finish()
            }
        }

        pub trait $into_name {
            /// Serialize this value into the parameter.
            ///
            /// # Safety
            ///
            /// The end consumer must be aware of the true value of the
            /// parameter.
            unsafe fn serialize(self) -> $name;
            /// Deserialize this value from this parameter.
            ///
            /// # Safety
            ///
            /// The parameter must be a valid instance of this type.
            unsafe fn deserialize(val: $name) -> Self;
        }

        impl $name {
            pub const fn into_inner(self) -> $inner {
                self.value
            }

            pub const unsafe fn from_inner(inner: $inner) -> Self {
                Self {
                    value: inner,
                    _thread_unsafe: PhantomData,
                }
            }

            pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
                Self::from_inner(ptr as $inner)
            }

            pub const fn empty() -> Self {
                unsafe { Self::from_inner(0) }
            }
        }

        impl $into_name for $name {
            unsafe fn serialize(self) -> Self {
                self
            }

            unsafe fn deserialize(val: Self) -> Self {
                val
            }
        }

        impl $into_name for usize {
            unsafe fn serialize(self) -> $name {
                <$name>::from_inner(self as $inner)
            }

            unsafe fn deserialize(val: $name) -> Self {
                val.into_inner() as Self
            }
        }

        impl $into_name for isize {
            unsafe fn serialize(self) -> $name {
                <$name>::from_inner(self as $inner)
            }

            unsafe fn deserialize(val: $name) -> Self {
                val.into_inner() as Self
            }
        }

        impl $into_name for () {
            unsafe fn serialize(self) -> $name {
                <$name>::empty()
            }

            unsafe fn deserialize(_val: $name) -> () {}
        }
    };
}

base_impl! { Wparam, IntoWparam, usize }
base_impl! { Lparam, IntoLParam, isize }
base_impl! { Lresult, IntoLresult, isize }
