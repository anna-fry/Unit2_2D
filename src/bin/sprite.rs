use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Sprite {
    image: Rc<Texture>,
    pub frame: Rect, // Maybe better to use a type that can't have a negative origin
    // Or use =animation:Animation= instead of a frame field
    pub position: Vec2i,
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, frame: Rect, position: Vec2i) -> Self {
        Self {
            image: Rc::clone(image),
            frame,
            position,
        }
    }
}