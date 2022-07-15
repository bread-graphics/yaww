//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[allow(
    unused_unsafe,
    dead_code,
    non_snake_case,
    unused_mut
)]
#[rustfmt::skip]
mod automatically_generated;
pub use automatically_generated::*;

mod abort;
pub(crate) use abort::{abort, abort_on_panic};
mod cow;
pub use cow::*;
mod error;
pub(crate) use error::last_win32_error;
pub use error::{Error, Result};
mod manual_types;
pub use manual_types::*;
mod strlen;
pub(crate) use strlen::{strlen, wide_strlen};
