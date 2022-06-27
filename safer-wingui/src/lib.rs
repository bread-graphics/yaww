// MIT/Apache2 License

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod automatically_generated;
mod abort;
pub(crate) use abort::{abort, abort_on_panic};
mod cow;
pub use cow::*;
mod devmode;
pub use devmode::{Devmodea, Devmodew};
mod error;
pub use error::{Result, Error};
pub(crate) use error::last_win32_error;
mod pointer_info;
pub use pointer_info::PointerTypeInfo;
mod special_param;
pub use special_param::*;
mod wide_strlen;
pub(crate) use wide_strlen::wide_strlen;
