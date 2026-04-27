use core::ffi::c_void;

/// Declares a type as having a C++ virtual function table.
/// Structs that implement this trait should only ever exist as pointers/references
/// from the game itself. They should never be instantiated directly.
///
/// This trait can be implemented with a `derive`
/// ```ignore
/// #[derive(forge::HasVtable)]
/// pub struct MyStruct;
/// ```
pub trait HasVtable {
    fn vtable_ptr(&self) -> *const *const c_void;
}
