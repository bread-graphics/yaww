//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use windows_sys::Win32::UI::Controls::POINTER_TYPE_INFO;

pub enum PointerTypeInfo {
    // todo
}

impl PointerTypeInfo {
    pub fn to_win32(&self) -> POINTER_TYPE_INFO {
        todo!()
    }

    pub fn from_win32(pti: POINTER_TYPE_INFO) -> Self {
        todo!()
    }
}
