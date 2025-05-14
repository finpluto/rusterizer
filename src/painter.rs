use glam::{IVec2, Vec3};

use crate::{
    geometry::primitives::{Pixel, Vertices},
    operations::Interpolate,
    pixels::PixelBuffer,
    shaders::PixelShader,
};

pub trait PointPainter {
    fn draw_point(&mut self, x: u32, y: u32, color: &Vec3);
}

impl PointPainter for PixelBuffer<'_> {
    fn draw_point(&mut self, x: u32, y: u32, color: &Vec3) {
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
}

pub trait LinePainter {
    fn draw_line(&mut self, start: &IVec2, end: &IVec2, color: &Vec3);
}

impl<PP: PointPainter> LinePainter for PP {
    fn draw_line(&mut self, start: &IVec2, end: &IVec2, color: &Vec3) {
        let pixels = (start - end).abs().max_element() as usize + 1;
        for p in start.interpolate(end, pixels) {
            if p.x < 0 || p.y < 0 {
                continue;
            }
            self.draw_point(p.x as u32, p.y as u32, color);
        }
    }
}

#[allow(dead_code)]
pub trait PolygonPainter {
    fn draw_polygon(&mut self, vertices: impl AsRef<[IVec2]>);
}

impl<LP: LinePainter> PolygonPainter for LP {
    fn draw_polygon(&mut self, vertices: impl AsRef<[IVec2]>) {
        if vertices.as_ref().is_empty() {
            return;
        }

        let it = vertices
            .as_ref()
            .iter()
            .zip(vertices.as_ref().iter().cycle().skip(1));
        for (start, end) in it {
            self.draw_line(start, end, &Vec3::splat(1f32));
        }
    }
}

pub trait PolygonFiller {
    fn fill_polygon(&mut self, polygon: impl Vertices<Vertex = Pixel>);
}

impl<PS: PixelShader> PolygonFiller for PS {
    fn fill_polygon(&mut self, polygon: impl Vertices<Vertex = Pixel>) {
        let polygon_vertices = polygon.vertices();
        let y_max = polygon_vertices
            .as_ref()
            .iter()
            .max_by_key(|v| v.point.y)
            .unwrap()
            .point
            .y;
        let y_min = polygon_vertices
            .as_ref()
            .iter()
            .min_by_key(|v| v.point.y)
            .unwrap()
            .point
            .y;

        let mut left_pixels: Vec<Pixel> = (y_min..=y_max)
            .map(|y| Pixel::new(IVec2::new(i32::MAX, y), 0f32, Vec3::ZERO))
            .collect();
        let mut right_pixels: Vec<Pixel> = (y_min..=y_max)
            .map(|y| Pixel::new(IVec2::new(i32::MIN, y), 0f32, Vec3::ZERO))
            .collect();

        let viter = polygon_vertices.as_ref().iter();
        let viter_skip1 = polygon_vertices.as_ref().iter().cycle().skip(1);

        let edge_pixel_iter = viter.zip(viter_skip1).flat_map(|(start, end)| {
            let pixels = (start.point - end.point).abs().max_element() as usize + 1;
            start.interpolate(end, pixels)
        });

        for p in edge_pixel_iter {
            let i = p.point.y - y_min;
            let i = i as usize;
            if left_pixels[i].point.x > p.point.x {
                left_pixels[i] = p;
            }
            if right_pixels[i].point.x < p.point.x {
                right_pixels[i] = p;
            }
        }

        for (start, end) in left_pixels.into_iter().zip(right_pixels.into_iter()) {
            let pixel_num = end.point.x - start.point.x + 1;
            for pixel in start.interpolate(&end, pixel_num as usize) {
                if pixel.point.x < 0 || pixel.point.y < 0 {
                    continue;
                }
                self.pixel_shader(pixel);
            }
        }
    }
}
