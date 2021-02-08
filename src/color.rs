// MIT/Apache2 License

use winapi::{shared::windef::COLORREF, um::wingdi::RGB};

/// A color in Win32.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    inner: COLORREF,
}

impl Color {
    #[inline]
    pub const fn from_colorref(colorref: COLORREF) -> Self {
        Self { inner: colorref }
    }

    #[inline]
    pub fn colorref(self) -> COLORREF {
        self.inner
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: RGB(r, g, b),
        }
    }
}
