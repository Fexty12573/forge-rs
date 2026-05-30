use core::ffi::{CStr, c_char, c_void};

use crate::mt::{crc::MtCRC, datatype::MtType, object::MtObject};

/// A single reflected property exposed by an [`MtObject`].
///
/// MT Framework's reflection system describes each object's fields as a list of
/// `MtProperty` entries (see [`MtPropertyList`]). A property carries its name,
/// its [data type](MtType), attribute flags, and pointers to the data (or the
/// getter/setter functions, for "get/set" accessors). Each property also knows
/// the [`owner`](Self::owner) object it belongs to and is chained to its
/// siblings through [`prev`](Self::prev) / [`next`](Self::next).
///
/// You do not construct these; they are produced by the game when an object's
/// properties are enumerated.
///
/// # Layout
///
/// `#[repr(C)]` so the fields match the game's in-memory layout exactly; this
/// type is only ever used through references to existing instances.
///
/// [`MtPropertyList`]: crate::mt::property_list::MtPropertyList
#[repr(C)]
pub struct MtProperty {
    name: *const c_char,
    dtype: MtType,
    attr: u16,
    owner: *mut MtObject,
    get: *mut c_void,
    _unk1: *const c_void,
    count: u32,
    _unk2: *const c_void,
    set: *const c_void,
    _unk3: *const c_void,
    set_count: *const c_void,
    _unk4: *const c_void,
    index: u32,
    /// Previous property in the owner's property list, or null at the head.
    pub prev: *const MtProperty,
    /// Next property in the owner's property list, or null at the tail.
    pub next: *const MtProperty,
}

/// Attribute bit: the property is accessed through getter/setter functions
/// rather than a direct field pointer.
const ATTR_GET_SET: u16 = 0x80;
/// Attribute bit: the property holds an array of values.
const ATTR_ARRAY: u16 = 0x20;
/// Attribute bit: the property represents an event rather than stored data.
const ATTR_EVENT: u16 = 0x08;

impl MtProperty {
    /// Returns the property's name as a string slice.
    ///
    /// # Panics
    ///
    /// Panics if the name pointer is null or the name is not valid UTF-8. Use
    /// [`has_name`](Self::has_name) to guard against the null case.
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name) }
            .to_str()
            .expect("Failed to convert string to UTF-8")
    }

    /// Returns `true` if this property has a (non-null) name.
    pub fn has_name(&self) -> bool {
        !self.name.is_null()
    }

    /// Returns the property's [data type](MtType).
    pub fn dtype(&self) -> MtType {
        self.dtype
    }

    /// Returns the raw attribute flags, also exposed through
    /// [`is_get_set`](Self::is_get_set), [`is_array`](Self::is_array) and
    /// [`is_event`](Self::is_event).
    pub fn attr(&self) -> u16 {
        self.attr
    }

    /// Returns the CRC32 of the property's name, the value MT Framework uses to
    /// identify it.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`name`](Self::name).
    pub fn crc(&self) -> u32 {
        MtCRC::from_str(self.name(), !0)
    }

    /// Returns a mutable reference to the object that owns this property.
    pub fn owner(&self) -> &mut MtObject {
        unsafe { &mut *self.owner }
    }

    /// Returns the raw pointer to the object that owns this property.
    pub fn owner_ptr(&self) -> *mut MtObject {
        self.owner
    }

    /// Returns `true` if the property is accessed through getter/setter
    /// functions rather than a direct field pointer.
    pub fn is_get_set(&self) -> bool {
        (self.attr & ATTR_GET_SET) != 0
    }

    /// Returns `true` if the property holds an array of values.
    pub fn is_array(&self) -> bool {
        (self.attr & ATTR_ARRAY) != 0
    }

    /// Returns `true` if the property represents an event rather than stored
    /// data.
    ///
    /// True when either the event attribute bit is set or the [data
    /// type](Self::dtype) is one of the `Event` variants.
    pub fn is_event(&self) -> bool {
        (self.attr & ATTR_EVENT) != 0 || matches!(self.dtype, MtType::Event | MtType::Event32 | MtType::Event64)
    }

    /// Returns a typed pointer to the property's backing data, indexed by the
    /// property's element index.
    ///
    /// Only valid for direct-field properties; `T` must match the property's
    /// actual value type. See [`is_get_set`](Self::is_get_set) for accessor
    /// properties, which have no direct pointer.
    pub fn ptr<T>(&self) -> *mut T {
        unsafe { (self.get as *mut T).add(self.index as usize) }
    }

    /// Returns a mutable reference to the property's backing data as `T`.
    ///
    /// Convenience wrapper around [`ptr`](Self::ptr); the same validity
    /// requirements apply.
    pub fn as_ref<T>(&self) -> &mut T {
        unsafe { &mut *(self.get as *mut T).add(self.index as usize) }
    }

    /// Returns the raw getter pointer (a field address, or a getter function
    /// for accessor properties).
    pub fn get(&self) -> *const c_void {
        self.get
    }

    /// Returns the raw setter function pointer, if any.
    pub fn set(&self) -> *const c_void {
        self.set
    }

    /// Returns the raw "get count" pointer for array properties.
    pub fn getcount(&self) -> *const c_void {
        self.count as *const c_void
    }

    /// Returns the raw "set count" pointer for array properties.
    pub fn setcount(&self) -> *const c_void {
        self.set_count
    }

    /// Returns the element count for array properties.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Returns the property's byte offset within its owner object.
    ///
    /// Computed as the distance from the owner's address to the field pointer;
    /// saturates to `0` for accessor properties whose `get` is not a field
    /// address.
    pub fn offset(&self) -> usize {
        (self.get as usize).saturating_sub(self.owner as usize)
    }

    /// Returns the next property in the owner's list, or `None` at the tail.
    pub fn next(&self) -> Option<&MtProperty> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }

    /// Returns the previous property in the owner's list, or `None` at the head.
    pub fn prev(&self) -> Option<&MtProperty> {
        if self.prev.is_null() {
            None
        } else {
            Some(unsafe { &*self.prev })
        }
    }
}
