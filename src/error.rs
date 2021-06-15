// MIT/Apache2 License

use std::{error, ffi::CString, fmt, ptr};
use winapi::{
    shared::minwindef::DWORD,
    um::{errhandlingapi, winbase},
};

#[derive(Debug)]
pub enum Error {
    Dynamic(Box<dyn error::Error + Send>),
    ServerClosed,
    Win32 {
        code: DWORD,
        message: CString,
        function: Option<&'static str>,
    },
    FailedToGetError,
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dynamic(e) => fmt::Display::fmt(e, f),
            Error::ServerClosed => {
                f.write_str("Attempted to send request to the GUI thread after it closed")
            }
            Error::Win32 {
                code,
                message,
                function: None,
            } => write!(f, "Win32 error with code {}: {:?}", code, message),
            Error::Win32 {
                code,
                message,
                function: Some(function),
            } => write!(
                f,
                "Win32 error in function {} with code {}: {:?}",
                code, function, message
            ),
            Error::FailedToGetError => f.write_str("Failed to get the error message"),
        }
    }
}

impl Error {
    #[inline]
    pub(crate) fn win32_error(function: Option<&'static str>) -> Error {
        // get the error code associated with the last function
        let error = unsafe { errhandlingapi::GetLastError() };

        // allocate memory for the error message
        const MESSAGE_BUFFER: usize = 256;
        let mut message = Vec::with_capacity(MESSAGE_BUFFER);

        // get the message
        let len = unsafe {
            winbase::FormatMessageA(
                winbase::FORMAT_MESSAGE_FROM_SYSTEM | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null_mut(),
                error,
                0,
                message.as_mut_ptr() as *mut _,
                MESSAGE_BUFFER as _,
                ptr::null_mut(),
            )
        };
        if len == 0 {
            Error::FailedToGetError
        } else {
            unsafe { message.set_len(len as usize) };
            match CString::new(message) {
                Ok(message) => Error::Win32 {
                    code: error,
                    message,
                    function,
                },
                Err(_) => Error::FailedToGetError,
            }
        }
    }
}

pub type Result<O = ()> = std::result::Result<O, Error>;
