// MIT/Apache2 License

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
    }
}

devmode! { Devmodea, DEVMODEA, }
devmode! { Devmodew, DEVMODEW, }