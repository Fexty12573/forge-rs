pub struct Graphics;

impl Graphics {
    pub fn is_menu_visible() -> bool {
        unsafe { sys::graphics::forge_graphics_isMenuVisible() }
    }
}
