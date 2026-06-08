use core::{ffi::c_void, mem::transmute};

use macros::Object;
use sys::cpp::HasVtable;

use crate::mt::{dti::MtDti, property_list::MtPropertyList};

/// The root class of MT Framework's object system (`MtObject`).
///
/// Nearly every game class derives from `MtObject`; it provides the reflection
/// machinery shared by all of them - a link to the class's [`MtDti`] and the
/// ability to enumerate its reflected properties as an [`MtPropertyList`]. The
/// shared virtual-function behaviour (destructors, construction, property
/// creation, DTI access) lives in the [`Object`] trait, which this type
/// implements via `#[derive(Object)]`.
///
/// Instances are owned by the game; you work with them through references,
/// usually after obtaining one from the game or constructing a concrete
/// subclass through its [`MtDti`].
#[derive(Object)]
pub struct MtObject;

impl MtObject {
    /// Returns the [`MtDti`] describing this object's concrete class.
    pub fn dti(&self) -> &'static MtDti {
        self.get_dti()
    }

    /// Collects this object's reflected properties into an [`MtPropertyList`].
    ///
    /// Invokes the object's property-creation virtual function to populate a
    /// fresh list, which can then be iterated to inspect each property.
    pub fn get_properties(&self) -> MtPropertyList {
        let mut props = MtPropertyList::new();
        self.create_property(&mut props);
        props
    }

    /// Reinterprets this object as a shared reference to the subclass `T`.
    ///
    /// This is an unchecked downcast; the caller must ensure the object is
    /// actually a `T` (e.g. via [`MtDti::is_a`](crate::mt::dti::MtDti::is_a)).
    pub fn to<T: Object>(&self) -> &T {
        unsafe { &*(self as *const Self as *const T) }
    }

    /// Reinterprets this object as a mutable reference to the subclass `T`.
    ///
    /// This is an unchecked downcast; the caller must ensure the object is
    /// actually a `T` (e.g. via [`MtDti::is_a`](crate::mt::dti::MtDti::is_a)).
    pub fn to_mut<T: Object>(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut Self as *mut T) }
    }
}

/// Behaviour shared by every MT Framework object, backed by the class's vtable.
///
/// This trait exposes the common virtual functions found at the start of every
/// `MtObject` vtable. It is implemented for concrete object types via
/// `#[derive(Object)]`; each method dispatches through the corresponding vtable
/// slot, so calling one runs the game's own implementation for that object.
pub trait Object: HasVtable {
    /// Runs the destructor on `self`
    ///
    /// # Safety
    /// `self` is in an undefined state after this function is called.
    /// It is not safe to continue using the object.
    fn dtor(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(0)) };
        unsafe { func(self) }
    }

    /// Runs the destructor on `self` and deallocates the object
    ///
    /// # Safety
    /// `self` is in an undefined state after this function is called.
    /// It is not safe to continue using the object.
    fn destroy(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(1)) };
        unsafe { func(self) }
    }

    /// Invokes the object's UI-creation virtual function.
    ///
    /// Note: Always unimplemented.
    fn create_ui(&self) {
        let func: unsafe extern "C" fn(&Self) = unsafe { transmute(self.get_virtual_function(2)) };
        unsafe { func(self) }
    }

    /// Returns whether the object is enabled
    fn is_enable_instance(&self) -> bool {
        let func: unsafe extern "C" fn(&Self) -> bool = unsafe { transmute(self.get_virtual_function(3)) };
        unsafe { func(self) }
    }

    /// Populates `props` with the object's reflected properties.
    ///
    /// Prefer the safe [`MtObject::get_properties`] wrapper, which builds and
    /// returns the list for you.
    fn create_property(&self, props: &mut MtPropertyList) {
        let func: unsafe extern "C" fn(&Self, *mut MtPropertyList) = unsafe { transmute(self.get_virtual_function(4)) };
        unsafe { func(self, props as *mut MtPropertyList) }
    }

    /// Returns a reference to the object's [`MtDti`].
    ///
    /// Equivalent to the [`MtObject::dti`] wrapper.
    fn get_dti(&self) -> &'static MtDti {
        let func: unsafe extern "C" fn(&Self) -> *const c_void = unsafe { transmute(self.get_virtual_function(5)) };
        let dti = unsafe { func(self) };
        unsafe { &*(dti as *const MtDti) }
    }

    /// Converts this object to a reference to an [`MtObject`].
    fn to_mt_object(&self) -> &MtObject {
        let ptr = self as *const Self as *const MtObject;
        unsafe { &*ptr }
    }

    /// Converts this object to a mutable reference to an [`MtObject`].
    fn to_mut_mt_object(&mut self) -> &mut MtObject {
        let ptr = self as *mut Self as *mut MtObject;
        unsafe { &mut *ptr }
    }

    /// Reads a value of type `T` from `self` at the given byte offset.
    ///
    /// # Safety
    /// The caller must ensure that `self` is valid for reads of type `T` at the given offset.
    /// `offset` must be a valid byte offset within `self`.
    unsafe fn read<T>(&self, offset: usize) -> T {
        let ptr = self as *const Self as *const u8;
        unsafe { (ptr.add(offset) as *const T).read_unaligned() }
    }

    /// Writes a value of type `T` to `self` at the given byte offset.
    ///
    /// # Safety
    /// The caller must ensure that `self` is valid for writes of type `T` at the given offset.
    /// `offset` must be a valid byte offset within `self`.
    unsafe fn write<T: Copy>(&mut self, offset: usize, value: &T) {
        let ptr = self as *mut Self as *mut u8;
        unsafe { *(ptr.add(offset) as *mut T) = *value };
    }

    /// Returns a reference to a value of type `T` within `self` at the given byte offset.
    ///
    /// # Safety
    /// The caller must ensure that `self` is valid for reads of type `T` at the given offset.
    /// `offset` must be a valid byte offset within `self`.
    unsafe fn get_ref<T>(&self, offset: usize) -> &T {
        let ptr = self as *const Self as *const u8;
        unsafe { &*(ptr.add(offset) as *const T) }
    }

    /// Returns a mutable reference to a value of type `T` within `self` at the given byte offset.
    ///
    /// # Safety
    /// The caller must ensure that `self` is valid for writes of type `T` at the given offset.
    /// `offset` must be a valid byte offset within `self`.
    unsafe fn get_mut_ref<T>(&mut self, offset: usize) -> &mut T {
        let ptr = self as *mut Self as *mut u8;
        unsafe { &mut *(ptr.add(offset) as *mut T) }
    }
}
