// MIT/Apache2 License

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

//mod automatically_generated;
mod abort;
pub(crate) use abort::abort_on_panic;
mod cow;
pub use cow::*;
mod special_param;
pub use special_param::*;
mod wide_strlen;
pub(crate) use wide_strlen::wide_strlen;
