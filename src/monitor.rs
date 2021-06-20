// MIT/Apache2 License

use crate::{directive::Directive, key::Key, server::SendsDirective, task::Task};
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

pub trait MonitorFunctions {
    /// Get a list of every monitor currently available.
    fn monitors(&self) -> crate::Result<Task<crate::Result<Vec<MonitorInfo>>>>;
    /// Get the default monitor.
    fn default_monitor(&self) -> crate::Result<Task<crate::Result<Monitor>>>;
}

impl<S: SendsDirective> MonitorFunctions for S {
    #[inline]
    fn monitors(&self) -> crate::Result<Task<crate::Result<Vec<MonitorInfo>>>> {
        self.send(Directive::GetMonitors)
    }

    #[inline]
    fn default_monitor(&self) -> crate::Result<Task<crate::Result<Monitor>>> {
        self.send(Directive::GetDefaultMonitor)
    }
}
