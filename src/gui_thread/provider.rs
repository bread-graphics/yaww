// MIT/Apache2 License

// Here, we take two strategies:
//
// 1). If debug assertions are enabled, keys are not only the pointer but also the type of the key. The
//     provider will error if the translation fails.
//     not only translates the key to the pointer, but also the type.
// 2). If debug assertions are not enabled, keys

use std::{num::NonZeroUsize, ptr::NonNull};

#[cfg(debug_assertions)]
use std::collections::HashMap;

#[cfg_attr(not(debug_assertions), repr(transparent))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    // a lot of types here are just pointers so we can use this as a pointer
    // of course, it doesn't really make sense for this to be zero, so we can
    // optimize on space
    // (note: for things that aren't just pointers, we can just say that the
    //  key can't be zero)
    key: NonZeroUsize,
    // the type associated with the key
    #[cfg(debug_assertions)]
    ty: KeyType,
}

impl Key {
    pub(crate) unsafe fn from_pointer(pointer: NonNull<()>, ty: KeyType) -> Key {
        #[cfg(not(debug_assertions))]
        let _ = ty;

        Key {
            key: unsafe { NonZeroUsize::new_unchecked(pointer.as_ptr() as usize) },
            #[cfg(debug_assertions)]
            ty,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum KeyType {
    None,
    Window,
    Brush,
    Icon,
    Cursor,
    Menu,
    Dc,
}

/// Provides matches between data keys and data.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Provider {
    #[cfg(debug_assertions)]
    type_map: HashMap<NonZeroUsize, KeyType>,
    #[cfg(not(debug_assertions))]
    _private: (),
}

impl Provider {
    #[inline]
    pub fn new() -> Self {
        Provider {
            #[cfg(debug_assertions)]
            type_map: HashMap::new(),
            #[cfg(not(debug_assertions))]
            _private: (),
        }
    }

    #[inline]
    pub fn create_key(&mut self, pointer: NonNull<()>, ty: KeyType) -> crate::Result<Key> {
        #[cfg(debug_assertions)]
        {
            let key = unsafe { NonZeroUsize::new_unchecked(pointer.as_ptr() as usize) };
            if let Some(old_value) = self.type_map.insert(key, ty) {
                self.type_map.insert(key, old_value);
                Err(crate::Error::PtrAlreadyExists)
            } else {
                Ok(Key { key, ty })
            }
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = ty;
            Ok(Key {
                key: unsafe { NonZeroUsize::new_unchecked(pointer.as_ptr() as usize) },
            })
        }
    }

    #[inline]
    pub fn remove_key(&mut self, pointer: NonNull<()>) {
        #[cfg(debug_assertions)]
        {
            let key = unsafe { NonZeroUsize::new_unchecked(pointer.as_ptr() as usize) };
            self.type_map.remove(&key);
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = pointer;
        }
    }

    #[inline]
    pub fn translate<K: Into<Key>>(&mut self, key: K) -> crate::Result<NonNull<()>> {
        #[cfg(debug_assertions)]
        {
            // see if the key has the right type
            let Key { key, ty } = key.into();
            if self.type_map.get(&key) == Some(&ty) {
                // SAFETY: we know that key is not zero because it came from a NonZeroUsize
                Ok(unsafe { NonNull::new_unchecked(key.get() as *mut ()) })
            } else {
                Err(crate::Error::TypeMismatch)
            }
        }

        #[cfg(not(debug_assertions))]
        {
            // just translate the number directly
            Ok(unsafe { NonNull::new_unchecked(key.key.get() as *mut ()) })
        }
    }
}
