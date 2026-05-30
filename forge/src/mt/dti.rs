use core::ffi::{CStr, c_char, c_void};

use macros::{HasVtable, pure_virtual};

use crate::mt::{crc::MtCRC, object::Object};

/// Runtime type information for an MT Framework class ("Data Type Information").
///
/// Every class in MT Framework's reflection system is described by a single,
/// globally-unique `MtDti` instance owned by the game. A DTI records the class
/// name, a CRC32-derived [`id`](Self::id), the instance size and allocator used
/// to construct objects, and the factory virtual functions that create them. It
/// also forms the framework's type hierarchy: each DTI links to its
/// [`parent`](Self::parent), its first [`child`](Self::child), and the
/// [`next`](Self::next) sibling, which together describe the class tree used for
/// type queries such as [`is_a`](Self::is_a).
///
/// You do not create `MtDti` values; you borrow the game's existing ones, most
/// commonly by looking one up by name with [`find`](Self::find). A reference
/// obtained that way is `'static` because the DTI lives for the lifetime of the
/// process.
///
/// # Layout
///
/// `#[repr(C)]` so the fields match the game's in-memory layout exactly; this
/// type is only ever used through references to existing instances.
#[repr(C)]
#[derive(HasVtable)]
pub struct MtDti {
    _vft: *const c_void,
    name: *const c_char,
    next: *const MtDti,
    child: *const MtDti,
    parent: *const MtDti,
    link: *const MtDti,
    meta: u32,
    id: u32,
}

/// Provides cached access to the [`MtDti`] for a known class.
///
/// Implemented for types that correspond to a specific MT Framework class,
/// allowing the class's DTI to be retrieved (and typically cached after the
/// first lookup) without naming it by string at every call site. Returns
/// `None` if the class is not present in the running game.
pub trait CacheDti {
    /// Returns the [`MtDti`] describing this type's class, if it exists.
    fn dti() -> Option<&'static MtDti>;
}

impl MtDti {
    /// Computes the DTI id for a class name.
    ///
    /// The id is the CRC32 of `name` (seeded with `0xFFFFFFFF`) masked to its
    /// low 31 bits, matching the scheme the game uses to identify classes. This
    /// is the same value stored in [`id`](Self::id) and is what
    /// [`find`](Self::find) hashes a name into before searching.
    pub fn make_id(name: &str) -> u32 {
        MtCRC::from_str(name, 0xFFFFFFFF) & 0x7FFFFFFF
    }

    /// Looks up the DTI for a class by name in the running game.
    ///
    /// The name is hashed with [`make_id`](Self::make_id) and resolved through
    /// the game's DTI registry. Returns `None` if no class with that name is
    /// registered.
    pub fn find(name: &str) -> Option<&'static MtDti> {
        type FindDtiFunc = unsafe extern "C" fn(u32) -> *const MtDti;
        let addr = crate::mem::text_addr() + 0x7AEF68;
        let func: FindDtiFunc = unsafe { core::mem::transmute(addr as *const c_void) };

        let dti = unsafe { func(Self::make_id(name)) };
        if dti == core::ptr::null() {
            None
        } else {
            Some(unsafe { &*dti })
        }
    }

    /// Constructs a new instance of this class, allocating it via the game's
    /// factory.
    ///
    /// Returns a `'static` mutable reference to the freshly constructed object,
    /// reinterpreted as `T`, or `None` if allocation/construction failed. The
    /// caller is responsible for ensuring `T` actually matches this DTI's class
    /// and for eventually destroying the object.
    pub fn new<T: Object>(&self) -> Option<&'static mut T> {
        unsafe {
            let ptr = self.new_instance_impl();
            if ptr != core::ptr::null_mut() {
                Some(&mut *(ptr as *mut T))
            } else {
                None
            }
        }
    }

    /// Constructs this class in place into already-allocated storage.
    ///
    /// Runs the class's constructor on the memory backing `obj` rather than
    /// allocating new storage. Returns `true` on success. `obj` must be valid
    /// storage for an instance of this DTI's class (e.g. at least
    /// [`size`](Self::size) bytes, suitably aligned).
    pub fn instantiate<T: Object>(&self, obj: &mut T) -> bool {
        let ptr = self.instantiate_impl(core::ptr::from_mut(obj) as *mut c_void);
        ptr != core::ptr::null_mut()
    }

    /// Returns the class name as a string slice.
    ///
    /// # Panics
    ///
    /// Panics if the underlying name is not valid UTF-8.
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }

    /// Returns this class's parent (base class) in the type hierarchy, or
    /// `None` if it is a root.
    pub fn parent(&self) -> Option<&MtDti> {
        if self.parent.is_null() {
            None
        } else {
            Some(unsafe { &*self.parent })
        }
    }

    /// Returns this class's first child (most-derived subclass) in the type
    /// hierarchy, or `None` if it has no subclasses.
    ///
    /// Remaining children are reached by walking [`next`](Self::next) from the
    /// returned DTI.
    pub fn child(&self) -> Option<&MtDti> {
        if self.child.is_null() {
            None
        } else {
            Some(unsafe { &*self.child })
        }
    }

    /// Returns the next sibling class sharing the same parent, or `None` if
    /// this is the last sibling.
    pub fn next(&self) -> Option<&MtDti> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }

    /// Returns the next DTI in the game's global registration list, or `None`
    /// at the end of the list.
    ///
    /// Unlike [`next`](Self::next), this link is independent of the class
    /// hierarchy and can be used to enumerate every registered class.
    pub fn link(&self) -> Option<&MtDti> {
        if self.link.is_null() {
            None
        } else {
            Some(unsafe { &*self.link })
        }
    }

    /// Returns the size, in bytes, of an instance of this class.
    ///
    /// Decoded from the packed `meta` field; the stored value is in 4-byte
    /// units and is scaled back up here.
    pub fn size(&self) -> usize {
        ((self.meta & 0x7FFFFF) << 2) as usize
    }

    /// Returns the index of the allocator the game uses to construct instances
    /// of this class.
    ///
    /// Decoded from the packed `meta` field.
    pub fn allocator_index(&self) -> usize {
        ((self.meta >> 23) & 0x3F) as usize
    }

    /// Returns the class attribute flags.
    ///
    /// Decoded from the top bits of the packed `meta` field.
    pub fn attr(&self) -> u32 {
        (self.meta >> 29) & 0x7
    }

    /// Returns this class's unique id (see [`make_id`](Self::make_id)).
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns `true` if this class is, or derives from, the class with id
    /// `other_id`.
    ///
    /// Walks the parent chain comparing ids, so it answers "is-a" relationships
    /// across the whole hierarchy, not just the immediate class.
    pub fn is_a(&self, other_id: u32) -> bool {
        let mut current = Some(self);
        while let Some(dti) = current {
            if dti.id == other_id {
                return true;
            }

            let parent = dti.parent();
            if dti.id == parent.map_or(0, |p| p.id) {
                break;
            }

            current = parent;
        }
        false
    }

    /// Returns `true` if this class is, or derives from, `other`.
    ///
    /// Convenience wrapper around [`is_a`](Self::is_a) taking another DTI.
    pub fn is_a_dti(&self, other: &MtDti) -> bool {
        self.is_a(other.id)
    }

    /// Returns `true` if this class is, or derives from, the class named
    /// `other_name`.
    ///
    /// Convenience wrapper around [`is_a`](Self::is_a) that hashes the name via
    /// [`make_id`](Self::make_id).
    pub fn is_a_str(&self, other_name: &str) -> bool {
        self.is_a(Self::make_id(other_name))
    }

    // Virtual Functions
    #[pure_virtual(0)]
    fn dtor(&mut self);

    #[pure_virtual(1)]
    fn dtor2(&mut self);

    #[pure_virtual(2)]
    fn new_instance_impl(&self) -> *mut c_void;

    #[pure_virtual(3)]
    fn instantiate_impl(&self, obj: *mut c_void) -> *mut c_void;

    #[pure_virtual(4)]
    fn instantiate_array_impl(&self, objs: *mut c_void, count: i64) -> *mut c_void;
}
