// MIT/Apache2 License

use core::{fmt::{self, Write}, num::NonZeroU16};

/// An error that may occur during operation of the Windows API.
pub struct Error {
    repr: Repr,
}

enum Repr {
    FormattedMessage {
        error_code: NonZeroU16,
        msg: error_buffer::ErrorBuffer,
        function_call: &'static str,
    },
    StaticMsg(&'static str),
}

pub type Result<T = ()> = core::result::Result<T, Error>;

pub(crate) fn last_win32_error() -> Error {
    todo!()
}

#[cfg(feature = "alloc")]
mod error_buffer {
    use alloc::vec::Vec;
    use core::str::from_utf8_unchecked;
    use cstr_core::CStr;

    pub(crate) struct ErrorBuffer {
        heap: Vec<u8>
    }
}

#[cfg(not(feature = "alloc"))]
mod error_buffer {
    use core::str::from_utf8_unchecked;
    use cstr_core::CStr;

    const LIMIT: u16 = 64;
    const LIMIT_US: usize = LIMIT as usize;

    pub(crate) struct ErrorBuffer {
        stack: [u8; LIMIT_US],
        valid: usize,
    }

    impl ErrorBuffer {
        fn msg(&self) -> &[u8] {
            &self.stack[..self.valid]
        }
    }
}