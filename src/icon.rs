// MIT/Apache2 License

use breadthread::key_type;
use winapi::shared::windef::HICON__;

pub(crate) const ICON_IDENTIFIER: usize = 0x993;

key_type! {
    /// A handle to a window icon.
    pub struct Icon(HICON__): [IconType, ICON_IDENTIFIER];
}
