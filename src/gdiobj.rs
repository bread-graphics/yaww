// MIT/Apache2 License

use crate::{directive::Directive, server::SendsDirective, task::Task, Key};

pub type GdiObject = Key;

impl GdiObject {
    #[inline]
    pub fn delete_gdi<S: SendsDirective>(self, gt: &S) -> crate::Result<Task<()>> {
        gt.send_directive(Directive::DeleteObject { obj: self })
    }
}
