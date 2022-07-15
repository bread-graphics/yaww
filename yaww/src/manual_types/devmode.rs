//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use windows_sys::Win32::Graphics::Gdi::{DEVMODEA, DEVMODEW};

macro_rules! devmode {
    (
        $name: ident,
        $win32: ident,
    ) => {
        pub struct $name {
            // TODO
        }

        impl $name {
            pub(crate) fn to_win32(&self) -> $win32 {
                todo!()
            }

            pub(crate) unsafe fn from_win32(win32: $win32) -> Self {
                todo!()
            }
        }
    };
}

devmode! { Devmodea, DEVMODEA, }
devmode! { Devmodew, DEVMODEW, }
