// MIT/Apache2 License

use core::marker::PhantomData;

macro_rules! base_impl {
    ($name: ident, $into_name: ident, $inner: ident) => {
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        pub struct $name {
            value: $inner,
            _thread_unsafe: PhantomData<*const ()>,
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
            pub fn into_inner(self) -> $inner {
                self.value
            }

            pub unsafe fn from_inner(inner: $inner) -> Self {
                Self {
                    value: inner,
                    _thread_unsafe: PhantomData,
                }
            }

            pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
                Self::from_inner(ptr as $inner)
            }

            pub fn empty() -> Self {
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
base_impl! { Lparam, IntoWParam, isize }
