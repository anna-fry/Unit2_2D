#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}
impl Rect {
    pub fn rect_touching(r1: Rect, r2: Rect) -> bool {
        // r1 left is left of r2 right
        r1.x <= r2.x+r2.w as i32 &&
            // r2 left is left of r1 right
            r2.x <= r1.x+r1.w as i32 &&
            // those two conditions handle the x axis overlap;
            // the next two do the same for the y axis:
            r1.y <= r2.y+r2.h as i32 &&
            r2.y <= r1.y+r1.h as i32
    }

    pub fn rect_displacement(r1: Rect, r2: Rect) -> Option<(i32, i32)> {
        let x_overlap = (r1.x + r1.w as i32).min(r2.x + r2.w as i32) - r1.x.max(r2.x);
        let y_overlap = (r1.y + r1.h as i32).min(r2.y + r2.h as i32) - r1.y.max(r2.y);
        if x_overlap >= 0 && y_overlap >= 0 {
            Some((x_overlap, y_overlap))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Effect{
    Speedup(usize),
    Hurt(usize),
    Nothing
}