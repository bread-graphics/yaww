// MIT/Apache2 License

use breadthread::key_type;
use winapi::shared::windef::HMENU__;

key_type! {
    /// A handle to a window's menu.
    pub struct Menu(HMENU__) : [MenuType, 0x992];
}
