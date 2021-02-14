// MIT/Apache2 License

use crate::{directive::Directive, server::GuiThread, task::Task, Key};

pub type GdiObject = Key;

impl GdiObject {
    #[inline]
    pub fn delete(self, gt: &GuiThread) -> crate::Result<Task<()>> {
        gt.send_directive(Directive::DeleteObject { obj: self })
    }
}
