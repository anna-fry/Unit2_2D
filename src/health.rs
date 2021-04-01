use crate::texture::Texture;
use crate::types::{Rect, Vec2i};
use std::rc::Rc;

pub struct HealthStatus {
    pub image: Rc<Texture>,
    pub lives: usize,
    pub frame: Rect,
    pub start: Vec2i,
    pub spacing: i32,
}

pub trait DrawHealthExt {
    fn draw_health(&mut self, s: &HealthStatus);
}

use crate::screen::Screen;
impl<'fb> DrawHealthExt for Screen<'fb> {
    fn draw_health(&mut self, h: &HealthStatus) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        for n in 0..h.lives {
            self.bitblt(
                &h.image,
                h.frame,
                Vec2i(h.start.0 + (n as i32 * h.spacing), h.start.1),
                false,
            );
        }
    }
}
