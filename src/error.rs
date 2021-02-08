// MIT/Apache2 License

use std::{ffi::CString, fmt, iter, ptr};
use winapi::{
    shared::{
        minwindef::DWORD,
        ntdef::{CHAR, LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT},
    },
    um::{errhandlingapi::GetLastError, winbase},
};

/// Error type combinator.
#[derive(Debug, Clone)]
pub enum Error {
    ChannelFailed,
    PtrAlreadyExists,
    TypeMismatch,
    Win32Error {
        code: DWORD,
        description: CString,
        calling_function: Option<&'static str>,
    },
    HandlerChangedDuringEvent,
    ErrorFailed,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChannelFailed => f.write_str("Unable to send messages through channel"),
            Self::PtrAlreadyExists => {
                f.write_str("The given pointer is already given by the provider")
            }
            Self::TypeMismatch => {
                f.write_str("Attempted to use an improper pointer key for a value")
            }
            Self::HandlerChangedDuringEvent => {
                f.write_str("Attempted to change event handler during an event")
            }
            Self::Win32Error {
                code,
                description,
                calling_function: None,
            } => write!(f, "Win32 Error ({}): {:?}", code, description),
            Self::Win32Error {
                code,
                description,
                calling_function: Some(cf),
            } => write!(
                f,
                "Win32 Error while calling {} ({}): {:?}",
                cf, code, description
            ),
            Self::ErrorFailed => f.write_str("Failed to get error"),
        }
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    #[inline]
    pub(crate) fn win32_error(function: Option<&'static str>) -> Self {
        // first, get the error code
        let code = unsafe { GetLastError() };

        // then, allocate some memory so we can format the message
        // TODO: make this new_uninit once that gets stabilized
        const MAX_BUFFER_LEN: usize = 100;
        let mut description: Vec<CHAR> = iter::repeat(0).take(MAX_BUFFER_LEN).collect();

        let len = unsafe {
            winbase::FormatMessageA(
                winbase::FORMAT_MESSAGE_FROM_SYSTEM | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null(),
                code,
                MAKELANGID(LANG_NEUTRAL as _, SUBLANG_DEFAULT as _) as _,
                description.as_mut_ptr(),
                MAX_BUFFER_LEN as _,
                ptr::null_mut(),
            )
        };
        if len == 0 {
            Error::ErrorFailed
        } else {
            let description: Box<[u8]> = description
                .into_iter()
                .map(|e| e as u8)
                .take(len as _)
                .collect();
            match CString::new(description) {
                Ok(description) => Error::Win32Error {
                    code,
                    description,
                    calling_function: function,
                },
                Err(_) => Error::ErrorFailed,
            }
        }
    }
}
