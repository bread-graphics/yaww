// MIT/Apache2 License

use core::mem;

/// Abort the current process.
pub(crate) fn abort() -> ! {
    cfg_if::cfg_if! {
        if #[cfg(feature = "std")] {
            // abort using normal system means
            std::process::abort()
        } else {
            // abort by panicking while panicking
            struct PanicOnDrop;

            impl Drop for PanicOnDrop {
                fn drop(&mut self) {
                    panic!("aborting the process")
                }
            }

            let _panic = PanicOnDrop;
            panic!("aborting the process by panicking while panicking")
        }
    }
}

/// Abort if the function panics.
pub(crate) fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    let bomb = AbortOnDrop;

    // run the function
    let t = f();

    // defuse the bomb
    mem::forget(bomb);

    t
}

/// A structure that aborts the current process when it is dropped.
struct AbortOnDrop;

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        abort();
    }
}