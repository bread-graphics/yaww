// MIT/Apache2 License

use crate::{directive::Directive, server::SendsDirective, task::Task};
use breadthread::key_type;
use winapi::ctypes::c_void;

pub(crate) const GDI_IDENTIFIER: usize = 0x995;

key_type! {
    /// A GDI object, generally used for window drawing.
    pub struct GdiObject(c_void) : [GdiObjectType, GDI_IDENTIFIER];
}

/// An object that can be transformed into a `GdiObject` and back.
pub trait AsGdiObject {
    /// Convert into a `GdiObject`.
    fn into_gdi_object(self) -> GdiObject;

    /// Convert from a `GdiObject`.
    fn from_gdi_object(obj: GdiObject) -> Self;

    /// Delete this object using the GDI mechanisms.
    #[inline]
    fn delete<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<()>>
    where
        Self: Sized,
    {
        gt.send(Directive::DeleteObject {
            obj: self.into_gdi_object(),
        })
    }
}

impl AsGdiObject for GdiObject {
    #[inline]
    fn into_gdi_object(self) -> GdiObject { self }

    #[inline]
    fn from_gdi_object(obj: GdiObject) -> GdiObject { obj } 
}

pub trait GdiFunctions {
    /// Load a stock object from the system.
    fn get_stock_object(&self, so: StockObject) -> crate::Result<Task<Option<GdiObject>>>;
}

impl<S: SendsDirective> GdiFunctions for S {
    #[inline]
    fn get_stock_object(&self, so: StockObject) -> crate::Result<Task<Option<GdiObject>>> {
        self.send(Directive::GetStockObject(so))
    }
}

/// A certain stock object.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StockObject {
    NullBrush,
}

impl GdiObject {
    #[inline]
    pub fn delete_gdi<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<()>> {
        gt.send(Directive::DeleteObject { obj: self })
    }
}
