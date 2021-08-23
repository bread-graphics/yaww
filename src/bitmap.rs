// MIT/Apache2 License

use super::gdiobj::{AsGdiObject, GdiObject, GDI_IDENTIFIER};
use breadthread::key_type;
use winapi::shared::windef::HBITMAP__;

key_type! {
    /// A bitmap, similar to an image.
    pub struct Bitmap(HBITMAP__) : [BitmapType, GDI_IDENTIFIER];
}

impl AsGdiObject for Bitmap {
    #[inline]
    fn into_gdi_object(self) -> GdiObject {
        GdiObject::from_raw(self.into_raw())
    }

    #[inline]
    fn from_gdi_object(obj: GdiObject) -> Self {
        Self::from_raw(obj.into_raw())
    }
}
