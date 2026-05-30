use core::{ffi::c_void, marker::PhantomData};

use macros::Object;

use crate::{mem, mt::property::MtProperty};

/// The list of [`MtProperty`] entries exposed by an [`MtObject`].
///
/// MT Framework reports an object's reflected properties by populating one of
/// these lists (see [`MtObject::get_properties`]). The list itself only holds
/// the head of the chain; the properties live in the game and are linked
/// together through their [`next`](MtProperty::next) pointers. Iterate it (via
/// [`iter`](Self::iter) or `for p in &list`) to walk those properties in order.
///
/// # Layout
///
/// `#[repr(C)]` so the fields match the game's in-memory layout exactly, since
/// the game writes directly into a list created with [`new`](Self::new).
///
/// [`MtObject`]: crate::mt::object::MtObject
/// [`MtObject::get_properties`]: crate::mt::object::MtObject::get_properties
#[repr(C)]
#[derive(Object)]
pub struct MtPropertyList {
    _vft: *const c_void,
    first: *const MtProperty,
}

/// Iterator over an [`MtPropertyList`], yielding each [`MtProperty`] from the
/// head of the chain onward.
///
/// Created by [`MtPropertyList::iter`] or by iterating a `&MtPropertyList`.
pub struct MtPropertyListIter<'a> {
    current: *const MtProperty,
    _phantom: PhantomData<&'a MtProperty>,
}

impl MtPropertyList {
    /// Creates a new, empty property list.
    ///
    /// The list is initialised with the game's `MtPropertyList` vtable so it can
    /// be handed to the game to be filled in (see
    /// [`MtObject::get_properties`](crate::mt::object::MtObject::get_properties));
    /// on its own it contains no properties.
    pub fn new() -> Self {
        Self {
            _vft: (mem::text_addr() + 0x177E9A0) as *const c_void,
            first: core::ptr::null(),
        }
    }

    /// Returns the first property in the list, or `None` if the list is empty.
    pub fn first(&self) -> Option<&MtProperty> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.first })
        }
    }

    /// Returns `true` if the list contains no properties.
    pub fn is_empty(&self) -> bool {
        self.first == core::ptr::null()
    }

    /// Returns the number of properties in the list.
    ///
    /// This walks the whole chain, so it is `O(n)`.
    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut len = 0;
        let mut p = self.first;
        while p != core::ptr::null() {
            len += 1;
            p = unsafe { (*p).next };
        }

        len
    }

    /// Returns an iterator over the properties in the list, from first to last.
    pub fn iter(&self) -> MtPropertyListIter<'_> {
        MtPropertyListIter {
            current: self.first,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for MtPropertyListIter<'a> {
    type Item = &'a MtProperty;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        let current = unsafe { &*self.current };
        self.current = current.next;
        Some(current)
    }
}

impl<'a> IntoIterator for &'a MtPropertyList {
    type Item = &'a MtProperty;
    type IntoIter = MtPropertyListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
