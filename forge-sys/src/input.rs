#[repr(i32)]
pub enum Button {
    A = 0,
    B = 1,
    X = 2,
    Y = 3,
    StickL = 4,
    StickR = 5,
    L = 6,
    R = 7,
    ZL = 8,
    ZR = 9,
    Plus = 10,
    Minus = 11,
    Left = 12,
    Up = 13,
    Right = 14,
    Down = 15,
}

#[repr(i32)]
pub enum Key {
    A = 4,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N1 = 30,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Enter = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
}

unsafe extern "C" {
    pub fn forge_input_isDown(button: Button) -> bool;
    pub fn forge_input_isPressed(button: Button) -> bool;
    pub fn forge_input_isReleased(button: Button) -> bool;

    pub fn forge_input_getStickL(x: *mut f32, y: *mut f32);
    pub fn forge_input_getStickR(x: *mut f32, y: *mut f32);

    pub fn forge_input_isConnected() -> bool;

    pub fn forge_input_getTouch(x: *mut f32, y: *mut f32) -> bool;

    pub fn forge_input_isKeyDown(key: Key) -> bool;
    pub fn forge_input_isKeyPressed(key: Key) -> bool;
    pub fn forge_input_isKeyReleased(key: Key) -> bool;

    pub fn forge_input_isShiftDown() -> bool;
    pub fn forge_input_isCtrlDown() -> bool;
    pub fn forge_input_isAltDown() -> bool;
}
