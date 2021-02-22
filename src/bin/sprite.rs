use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct Sprite {
    pub image: Rc<Texture>,
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

pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.bitblt(&s.image, s.frame, s.position);
    }
}
