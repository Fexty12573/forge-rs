use alloc::{boxed::Box, ffi::CString, vec::Vec};

use sys::singleton::*;

use crate::mt::object::{MtObject, Object};

/// Access point for the game's registered singletons.
///
/// Allows looking up singletons by class name or DTI `id`, and retrieving them as
/// [`MtObject`] references or as concrete subclasses. Also supports fetching all
/// registered singletons at once as a boxed slice of [`MtObject`] references.
pub struct SingletonManager;

impl SingletonManager {
    /// Looks up a singleton by class name, returning it as an [`MtObject`].
    ///
    /// Returns `None` if no singleton with that name is registered. See
    /// [`get_by_name_typed`](Self::get_by_name_typed) to receive a concrete
    /// subclass instead.
    pub fn get_by_name(name: &str) -> Option<&'static mut MtObject> {
        Self::get_by_name_typed(name)
    }

    /// Looks up a singleton by its DTI `id`, returning it as an [`MtObject`].
    ///
    /// Returns `None` if no singleton with that id is registered. See
    /// [`get_by_id_typed`](Self::get_by_id_typed) to receive a concrete
    /// subclass instead.
    pub fn get_by_id(id: u32) -> Option<&'static mut MtObject> {
        Self::get_by_id_typed(id)
    }

    /// Looks up a singleton by class name and returns it as the subclass `T`.
    ///
    /// The downcast to `T` is unchecked; the caller must ensure the singleton
    /// is actually a `T`. Returns `None` if `name` contains an interior NUL
    /// byte or if no singleton with that name is registered.
    pub fn get_by_name_typed<T: Object>(name: &str) -> Option<&'static mut T> {
        let c_name = CString::new(name).ok()?;
        let ptr = unsafe { forge_singleton_getInstanceByName(c_name.as_bytes().as_ptr()) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *(ptr as *mut T) })
        }
    }

    /// Looks up a singleton by its DTI `id` and returns it as the subclass `T`.
    ///
    /// The downcast to `T` is unchecked; the caller must ensure the singleton
    /// is actually a `T`. Returns `None` if no singleton with that id is
    /// registered.
    pub fn get_by_id_typed<T: Object>(id: u32) -> Option<&'static mut T> {
        let ptr = unsafe { forge_singleton_getInstanceById(id) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *(ptr as *mut T) })
        }
    }

    /// Returns every registered singleton as a boxed slice of [`MtObject`]
    /// references.
    ///
    /// The count is queried first, then the instances are fetched into a buffer
    /// of that size. If the game reports a different count on the second call
    /// (e.g. singletons were registered or removed in between) the mismatch is
    /// logged and the actual number returned is used.
    pub fn get_all() -> Box<[&'static mut MtObject]> {
        let count = unsafe { forge_singleton_getAllInstances(core::ptr::null_mut(), 0) };
        let mut instances = Vec::with_capacity(count as usize);

        let actual = unsafe {
            let n = forge_singleton_getAllInstances(instances.as_mut_ptr(), count);
            instances.set_len(n as usize);
            n
        };

        if actual != count {
            log::error!("Mismatching singleton count. Expected {count} got {actual}");
        }

        instances
            .iter()
            .map(|&inst| unsafe { &mut *(inst as *mut MtObject) })
            .collect()
    }
}
