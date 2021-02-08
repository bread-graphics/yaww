// MIT/Apache2 License

use crate::gui_thread::{Directive, GuiThread, Key};

pub type GdiObject = Key;

impl GdiObject {
    #[inline]
    pub fn delete(self, gt: &GuiThread) -> crate::Result {
        gt.send_directive(Directive::DeleteObject(self))?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn delete_async(self, gt: &GuiThread) -> crate::Result {
        gt.send_directive_async(Directive::DeleteObject(self))
            .await?;
        Ok(())
    }
}
