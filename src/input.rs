#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key
{
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    SPACE,
    ESCAPE,
    OTHER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton
{
    LEFT,
    RIGHT,
    MIDDLE,
    X1,
    X2,
    OTHER,
}

pub(crate) enum InputEvent
{
    Quit,
    KeyDown(Key),
    KeyUp(Key),
    MouseMove { x: i32, y: i32, dx: i32, dy: i32 },
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp { button: MouseButton, x: i32, y: i32 },
    MouseWheel { x: i32, y: i32 },
    TextInput(String),
}
