// MIT/Apache2 License

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
