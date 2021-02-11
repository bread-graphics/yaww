// MIT/Apache2 License

use crate::Key;
use std::num::NonZeroUsize;
use winapi::um::winuser;

pub type Brush = Key;

pub const DEFAULT_BRUSH: Brush = unsafe {
    Brush::from_raw(NonZeroUsize::new_unchecked(
        winuser::COLOR_WINDOW as usize + 1,
    ))
};
