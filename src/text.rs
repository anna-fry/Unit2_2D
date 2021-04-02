use crate::types::{Rect, Vec2i};
use fontdue::{Font, Metrics};
use std::collections::HashMap;

const NUM_FONTS: usize = 1;

pub struct Fonts {
    pub rasterized: HashMap<u64, (Metrics, Vec<u8>)>,
    pub font_list: [Font; NUM_FONTS],
}
impl Fonts {
    pub fn new(font_list: [Font; NUM_FONTS]) -> Self {
        Self {
            rasterized: HashMap::new(),
            font_list,
        }
    }
}
