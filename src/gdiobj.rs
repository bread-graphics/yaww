// MIT/Apache2 License

use crate::{directive::Directive, server::SendsDirective, task::Task, Key};

pub type GdiObject = Key;

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
