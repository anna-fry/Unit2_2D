use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;
#[derive(Clone)]
pub struct Sprite {
    image: Rc<Texture>,
    pub frame: Rect, // Maybe better to use a type that can't have a negative origin
    // Or use =animation:Animation= instead of a frame field
    pub position: Vec2i,
    pub drawable: bool,
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, frame: Rect, position: Vec2i, drawable: bool) -> Self {
        Self {
            image: Rc::clone(image),
            frame,
            position,
            drawable,
        }
    }
    pub fn get_dimensions(&self) -> Vec2i {
        Vec2i(self.frame.w as i32, self.frame.h as i32)
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
        if s.drawable {
            self.bitblt(&s.image, s.frame, s.position);
        }
    }
}
