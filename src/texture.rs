use crate::types::Rect;
use image::{self, RgbaImage};
use std::path::Path;

pub struct Texture {
    image: Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
}

enum AlphaChannel {
    First,
    Last,
}
impl Texture {
    pub fn with_file(path: &Path) -> Self {
        Self::new(image::open(path).expect("Couldn't load image").into_rgba8())
    }
    pub fn new(image: RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let mut image = image.into_vec();
        premultiply(&mut image, 4, AlphaChannel::Last);
        Self {
            width: width as usize,
            height: height as usize,
            depth: 4,
            image,
        }
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn pitch(&self) -> usize {
        self.width * self.depth
    }
    pub fn buffer(&self) -> &[u8] {
        &self.image
    }
    pub fn valid_frame(&self, frame: Rect) -> bool {
        0 <= frame.x
            && (frame.x + frame.w as i32) <= (self.width as i32)
            && 0 <= frame.y
            && (frame.y + frame.h as i32) <= (self.height as i32)
    }
}

fn premultiply(img: &mut [u8], depth: usize, alpha: AlphaChannel) {
    match alpha {
        AlphaChannel::First => {
            for px in img.chunks_exact_mut(depth) {
                let a = px[0] as f32 / 255.0;
                for component in px[1..].iter_mut() {
                    *component = (*component as f32 * a).round() as u8;
                }
                // swap around to rgba8888
                let a = px[0];
                px[0] = px[1];
                px[1] = px[2];
                px[2] = px[3];
                px[3] = a;
            }
        }
        AlphaChannel::Last => {
            for px in img.chunks_exact_mut(depth) {
                let a = *px.last().unwrap() as f32 / 255.0;
                for component in px[0..(depth - 1)].iter_mut() {
                    *component = (*component as f32 * a) as u8;
                }
                // already rgba8888
            }
        }
    }
}
