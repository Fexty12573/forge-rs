pub use sys::input::{Button, Key};

/// Provides access to controller, keyboard, and touchscreen input.
pub struct Input;

impl Input {
    /// Returns `true` if `button` is currently held down.
    pub fn is_button_down(button: Button) -> bool {
        unsafe { sys::input::forge_input_isDown(button) }
    }

    /// Returns `true` if `button` was pressed this frame.
    pub fn is_button_pressed(button: Button) -> bool {
        unsafe { sys::input::forge_input_isPressed(button) }
    }

    /// Returns `true` if `button` was released this frame.
    pub fn is_button_released(button: Button) -> bool {
        unsafe { sys::input::forge_input_isReleased(button) }
    }

    /// Returns the left analog stick position as `(x, y)` in the range `[-1.0, 1.0]`.
    pub fn get_stick_l() -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { sys::input::forge_input_getStickL(&mut x, &mut y) };
        (x, y)
    }

    /// Returns the right analog stick position as `(x, y)` in the range `[-1.0, 1.0]`.
    pub fn get_stick_r() -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { sys::input::forge_input_getStickR(&mut x, &mut y) };
        (x, y)
    }

    /// Returns `true` if a controller is connected.
    pub fn is_connected() -> bool {
        unsafe { sys::input::forge_input_isConnected() }
    }

    /// Returns the current touch position as `Some((x, y))`, or `None` if the screen is not touched.
    pub fn get_touch() -> Option<(f32, f32)> {
        let mut x = 0.0;
        let mut y = 0.0;
        if unsafe { sys::input::forge_input_getTouch(&mut x, &mut y) } {
            Some((x, y))
        } else {
            None
        }
    }

    /// Returns `true` if `key` is currently held down.
    pub fn is_key_down(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyDown(key) }
    }

    /// Returns `true` if `key` was pressed this frame.
    pub fn is_key_pressed(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyPressed(key) }
    }

    /// Returns `true` if `key` was released this frame.
    pub fn is_key_released(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyReleased(key) }
    }

    /// Returns `true` if either Shift key is held down.
    pub fn is_shift_down() -> bool {
        unsafe { sys::input::forge_input_isShiftDown() }
    }

    /// Returns `true` if either Ctrl key is held down.
    pub fn is_ctrl_down() -> bool {
        unsafe { sys::input::forge_input_isCtrlDown() }
    }

    /// Returns `true` if either Alt key is held down.
    pub fn is_alt_down() -> bool {
        unsafe { sys::input::forge_input_isAltDown() }
    }
}
