//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

/// Get the length of a null-terminated string.
pub(crate) unsafe fn strlen(mut str: *const u8) -> usize {
    let start = str;
    while *str != 0 {
        str = str.offset(1);
    }
    str.offset_from(start) as usize
}

/// Get the length of a 16-bit string (bytes until zero).
pub(crate) unsafe fn wide_strlen(mut wide: *const u16) -> usize {
    // impl inspired by musl's wcslen():
    // https://github.com/bminor/musl/blob/cfdfd5ea3ce14c6abf7fb22a531f3d99518b5a1b/src/string/wcslen.c
    let start = wide;
    while *wide != 0 {
        wide = wide.offset(1);
    }
    wide.offset_from(start) as usize
}
