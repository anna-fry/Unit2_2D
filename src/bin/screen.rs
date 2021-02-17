// We can pull in definitions from elsewhere in the crate!
use crate::texture::Texture;
use crate::types::{Rect, Rgba, Vec2i};
pub struct Screen<'fb> {
    framebuffer: &'fb mut [u8],
    width: usize,
    height: usize,
    depth: usize,
}
impl<'fb> Screen<'fb> {
    pub fn wrap(framebuffer: &'fb mut [u8], width: usize, height: usize, depth: usize) -> Self {
        Self {
            framebuffer,
            width,
            height,
            depth,
        }
    }
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    // This is not going to be the most efficient API.
    // Lots of bounds checks!
    #[inline(always)]
    pub fn draw_at(&mut self, col: Rgba, x: usize, y: usize) {
        // No need to check x or y < 0, they're usizes!
        if self.width <= x || self.height <= y {
            return;
        }
        assert_eq!(self.depth, 4);
        let c = [col.0, col.1, col.2, col.3];
        let idx = y * self.width * self.depth + x * self.depth;
        // TODO should handle alpha blending!
        self.framebuffer[idx..(idx + self.depth)].copy_from_slice(&c);
    }
    // If we know the primitives in advance we're in much better shape:
    pub fn clear(&mut self, col: Rgba) {
        let c = [col.0, col.1, col.2, col.3];
        for px in self.framebuffer.chunks_exact_mut(4) {
            px.copy_from_slice(&c);
        }
    }
    pub fn rect(&mut self, r: Rect, col: Rgba) {
        let c = [col.0, col.1, col.2, col.3];
        let x0 = r.x.max(0).min(self.width as i32) as usize;
        let x1 = (r.x + r.w as i32).max(0).min(self.width as i32) as usize;
        let y0 = r.y.max(0).min(self.height as i32) as usize;
        let y1 = (r.y + r.h as i32).max(0).min(self.height as i32) as usize;
        let depth = self.depth;
        let pitch = self.width * depth;
        for row in self.framebuffer[(y0 * pitch)..(y1 * pitch)].chunks_exact_mut(pitch) {
            for p in row[(x0 * depth)..(x1 * depth)].chunks_exact_mut(depth) {
                // TODO should handle alpha blending
                p.copy_from_slice(&c);
            }
        }
    }
    pub fn line(&mut self, Vec2i(x0, y0): Vec2i, Vec2i(x1, y1): Vec2i, col: Rgba) {
        let c = [col.0, col.1, col.2, col.3];
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let width = self.width as i32;
        let height = self.height as i32;
        let depth = self.depth;
        let pitch = self.width * depth;
        while x != x1 || y != y1 {
            // We couldn't just clamp x0/y0 and x1/y1 into bounds, because then
            // we might change the slope of the line.
            // We could find the intercept of the line with the left/right or top/bottom edges of the rect though, but that's work!
            if 0 <= x && x < width && 0 <= y && y < height {
                // TODO this bounds check could in theory be avoided with
                // the unsafe get_unchecked, but maybe better not...
                // TODO better handle alpha blending too, but not just yet...
                self.framebuffer[(y as usize * pitch + x as usize * depth)
                    ..(y as usize * pitch + (x as usize + 1) * depth)]
                    .copy_from_slice(&c);
            }
            let e2 = 2 * err;
            if dy <= e2 {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
    pub fn bitblt(&mut self, src: &Texture, from: Rect, Vec2i(to_x, to_y): Vec2i) {
        assert!(src.valid_frame(from));
        if (to_x + from.w as i32) < 0
            || (self.width as i32) <= to_x
            || (to_y + from.h as i32) < 0
            || (self.height as i32) <= to_y
        {
            return;
        }
        let depth = self.depth;
        assert_eq!(depth, src.depth());
        let src_pitch = src.pitch();
        let dst_pitch = self.width * depth;
        // All this rigmarole is just to avoid bounds checks on each pixel of the blit.
        // We want to calculate which row/col of the src image to start at and which to end at.
        // This way there's no need to even check for out of bounds draws.
        let y_skip = to_y.max(0) - to_y;
        let x_skip = to_x.max(0) - to_x;
        let y_count = (to_y + from.h as i32).min(self.height as i32) - to_y;
        let x_count = (to_x + from.w as i32).min(self.width as i32) - to_x;
        // The code above is gnarly so these are just for safety:
        debug_assert!(0 <= x_skip);
        debug_assert!(0 <= y_skip);
        debug_assert!(0 <= x_count);
        debug_assert!(0 <= y_count);
        debug_assert!(x_count <= from.w as i32);
        debug_assert!(y_count <= from.h as i32);
        debug_assert!(0 <= to_x + x_skip);
        debug_assert!(0 <= to_y + y_skip);
        debug_assert!(0 <= from.x + x_skip);
        debug_assert!(0 <= from.y + y_skip);
        debug_assert!(to_x + x_count <= self.width as i32);
        debug_assert!(to_y + y_count <= self.height as i32);
        // OK, let's do some copying now
        let src_buf = src.buffer();
        for (row_a, row_b) in src_buf
            [(src_pitch * (from.y + y_skip) as usize)..(src_pitch * (from.y + y_count) as usize)]
            .chunks_exact(src_pitch)
            .zip(
                self.framebuffer[(dst_pitch * (to_y + y_skip) as usize)
                    ..(dst_pitch * (to_y + y_count) as usize)]
                    .chunks_exact_mut(dst_pitch),
            )
        {
            let to_cols = row_b
                [(depth * (to_x + x_skip) as usize)..(depth * (to_x + x_count) as usize)]
                .chunks_exact_mut(depth);
            let from_cols = row_a
                [(depth * (from.x + x_skip) as usize)..(depth * (from.x + x_count) as usize)]
                .chunks_exact(depth);
            // Composite over, assume premultiplied rgba8888
            for (to, from) in to_cols.zip(from_cols) {
                let ta = to[3] as f32 / 255.0;
                let fa = from[3] as f32 / 255.0;
                for i in 0..3 {
                    to[i] = from[i].saturating_add((to[i] as f32 * (1.0 - fa)).round() as u8);
                }
                to[3] = ((fa + ta * (1.0 - fa)) * 255.0).round() as u8;
            }
        }
    }
}