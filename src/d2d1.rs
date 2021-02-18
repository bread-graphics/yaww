// MIT/Apache2 License

#![cfg(feature = "direct2d")]

use crate::ptr::Com;
use std::ptr::NonNull;
use winapi::um::d2d1_1::{ID2D1Device, ID2D1Factory1};

// Note: direct2d objects are actually thread-safe if we enable the option to do so, so we can let the users
// have them.

/// The Direct2D object factory.
pub struct D2D1Factory {
    inner: Com<ID2D1Factory1>,
}
