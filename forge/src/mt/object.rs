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
    pub fn dti(&self) -> &MtDti {
        unsafe { &*(self.get_dti() as *const MtDti) }
    }

    /// Collects this object's reflected properties into an [`MtPropertyList`].
    ///
    /// Invokes the object's property-creation virtual function to populate a
    /// fresh list, which can then be iterated to inspect each property.
    pub fn get_properties(&self) -> MtPropertyList {
        let mut props = MtPropertyList::new();
        self.create_property(&mut props as *mut MtPropertyList as *mut c_void);
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
    fn dtor(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(0)) };
        unsafe { func(self) }
    }

    /// Runs the destructor on `self` and deallocates the object
    fn destroy(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(1)) };
        unsafe { func(self) }
    }

    /// Invokes the object's UI-creation virtual function (vtable slot 2).
    fn create_ui(&self) {
        let func: unsafe extern "C" fn(&Self) = unsafe { transmute(self.get_virtual_function(2)) };
        unsafe { func(self) }
    }

    /// Returns whether the object is currently an enabled instance (vtable slot
    /// 3).
    fn is_enable_instance(&self) -> bool {
        let func: unsafe extern "C" fn(&Self) -> bool = unsafe { transmute(self.get_virtual_function(3)) };
        unsafe { func(self) }
    }

    /// Populates `props` (an [`MtPropertyList`]) with the object's reflected
    /// properties (vtable slot 4).
    ///
    /// Prefer the safe [`MtObject::get_properties`] wrapper, which builds and
    /// returns the list for you.
    fn create_property(&self, props: *mut c_void) {
        let func: unsafe extern "C" fn(&Self, *mut c_void) = unsafe { transmute(self.get_virtual_function(4)) };
        unsafe { func(self, props) }
    }

    /// Returns a raw pointer to the object's [`MtDti`] (vtable slot 5).
    ///
    /// Prefer the safe [`MtObject::dti`] wrapper, which returns a typed
    /// reference.
    fn get_dti(&self) -> *const c_void {
        let func: unsafe extern "C" fn(&Self) -> *const c_void = unsafe { transmute(self.get_virtual_function(5)) };
        unsafe { func(self) }
    }
}
