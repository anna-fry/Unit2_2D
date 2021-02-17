#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32,pub i32);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);