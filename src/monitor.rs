// MIT/Apache2 License

use crate::{directive::Directive, server::SendsDirective, task::Task};
use breadthread::key_type;
use winapi::shared::{ntdef::LONG, windef::HMONITOR__};

key_type! {
    /// A handle to a Win32 monitor.
    pub struct Monitor(HMONITOR__) : [MonitorType, 0x994];
}

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
