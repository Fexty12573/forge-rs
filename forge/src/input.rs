pub use sys::input::{Button, Key};

pub struct Input;

impl Input {
    pub fn is_button_down(button: Button) -> bool {
        unsafe { sys::input::forge_input_isDown(button) }
    }

    pub fn is_button_pressed(button: Button) -> bool {
        unsafe { sys::input::forge_input_isPressed(button) }
    }

    pub fn is_button_released(button: Button) -> bool {
        unsafe { sys::input::forge_input_isReleased(button) }
    }

    pub fn get_stick_l() -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { sys::input::forge_input_getStickL(&mut x, &mut y) };
        (x, y)
    }

    pub fn get_stick_r() -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { sys::input::forge_input_getStickR(&mut x, &mut y) };
        (x, y)
    }

    pub fn is_connected() -> bool {
        unsafe { sys::input::forge_input_isConnected() }
    }

    pub fn get_touch() -> Option<(f32, f32)> {
        let mut x = 0.0;
        let mut y = 0.0;
        if unsafe { sys::input::forge_input_getTouch(&mut x, &mut y) } {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn is_key_down(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyDown(key) }
    }

    pub fn is_key_pressed(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyPressed(key) }
    }

    pub fn is_key_released(key: Key) -> bool {
        unsafe { sys::input::forge_input_isKeyReleased(key) }
    }

    pub fn is_shift_down() -> bool {
        unsafe { sys::input::forge_input_isShiftDown() }
    }

    pub fn is_ctrl_down() -> bool {
        unsafe { sys::input::forge_input_isCtrlDown() }
    }

    pub fn is_alt_down() -> bool {
        unsafe { sys::input::forge_input_isAltDown() }
    }
}
