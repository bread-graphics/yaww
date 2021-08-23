// MIT/Apache2 License

use crate::icon::ICON_IDENTIFIER;
use breadthread::key_type;
use winapi::shared::windef::HICON__;

key_type! {
    /// A cursor.
    pub struct Cursor(HICON__) : [CursorType, ICON_IDENTIFIER];
}
