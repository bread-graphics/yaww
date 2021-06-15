// MIT/Apache2 License

use crate::{directive::Directive, key::Key, server::GuiThread, task::Task};
use winapi::shared::ntdef::LONG;

pub type Monitor = Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonitorInfo {
    pub monitor: Monitor,
    pub x: LONG,
    pub y: LONG,
    pub width: LONG,
    pub height: LONG,
}

impl GuiThread {
    /// Get a list of every monitor currently available.
    #[inline]
    pub fn monitors(&self) -> crate::Result<Task<crate::Result<Vec<MonitorInfo>>>> {
        self.send_directive(Directive::GetMonitors)
    }

    /// Get the default monitor.
    #[inline]
    pub fn default_monitor(&self) -> crate::Result<Task<crate::Result<Monitor>>> {
        self.send_directive(Directive::GetDefaultMonitor)
    }
}
