use glam::{IVec2, Vec3};

use crate::geometry::primitives::Pixel;

pub struct PixelBuffer<'b> {
    width: u32,
    height: u32,
    buf: &'b mut [u8],
    z_buf: Vec<f32>,
}

impl<'b> PixelBuffer<'b> {
    pub fn new(height: u32, width: u32, buf: &'b mut [u8]) -> PixelBuffer<'b> {
        Self {
            width,
            height,
            buf,
            z_buf: vec![0f32; (height * width) as usize],
        }
    }

    pub fn draw_point(&mut self, x: u32, y: u32, color: &Vec3) {
        let bytes_offset = (y * self.height + x) * 4;
        let bytes_offset = bytes_offset as usize;
        let rgb_8bits_color = (color * 255f32).clamp(Vec3::ZERO, Vec3::splat(255f32));
        if let Some(pixel_buf_ref) = self.buf.get_mut(bytes_offset..bytes_offset + 4) {
            let bgr_8bits_color = {
                let mut raw_color = rgb_8bits_color.as_u8vec3().to_array();
                raw_color.reverse();
                raw_color
            };
            // BGR
            pixel_buf_ref[0..3].copy_from_slice(&bgr_8bits_color);
            // A
            pixel_buf_ref[3] = 255;
        }
    }

    pub fn pixel_shader(&mut self, pixel: Pixel, color: &Vec3) {
        let z_idx = self.get_z_value_idx(pixel.point);

        // FIXME: weird out of bound issue
        if z_idx >= (self.width * self.height) as usize {
            return;
        }
        let z_recip = self.z_buf[z_idx];
        if pixel.z_recip > z_recip {
            self.z_buf[z_idx] = pixel.z_recip;
            self.draw_point(pixel.point.x as u32, pixel.point.y as u32, color);
        }
    }

    pub fn memset(&mut self, val: u8) {
        self.buf.fill(val);
    }

    fn get_z_value_idx(&self, point: IVec2) -> usize {
        (self.width * (point.y as u32) + (point.x as u32)) as usize
    }
}
