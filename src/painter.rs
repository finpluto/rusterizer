use glam::{IVec2, Vec3};

use crate::{
    geometry::primitives::{ColorPicker, Pixel, Vertices},
    operations::Interpolate,
    pixels::PixelBuffer,
};

pub trait LinePainter {
    fn draw_line(&mut self, start: &IVec2, end: &IVec2, color: &Vec3);
}

impl LinePainter for PixelBuffer<'_> {
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
pub trait PolygonPainter: LinePainter {
    fn draw_polygon(&mut self, vertices: impl AsRef<[IVec2]>);
}

impl PolygonPainter for PixelBuffer<'_> {
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
    fn fill_polygon(&mut self, polygon: impl Vertices<Vertex = Pixel> + ColorPicker);
}

impl PolygonFiller for PixelBuffer<'_> {
    fn fill_polygon(&mut self, polygon: impl Vertices<Vertex = Pixel> + ColorPicker) {
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
            .map(|y| Pixel::new(IVec2::new(i32::MAX, y), 0f32))
            .collect();
        let mut right_pixels: Vec<Pixel> = (y_min..=y_max)
            .map(|y| Pixel::new(IVec2::new(i32::MIN, y), 0f32))
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
                left_pixels[i].point.x = p.point.x;
                left_pixels[i].z_recip = p.z_recip;
            }
            if right_pixels[i].point.x < p.point.x {
                right_pixels[i].point.x = p.point.x;
                right_pixels[i].z_recip = p.z_recip;
            }
        }

        for (start, end) in left_pixels.into_iter().zip(right_pixels.into_iter()) {
            //for x in start.point.x..=end.point.x {
            //    if x < 0 || start.point.y < 0 {
            //        continue;
            //    }
            //    self.draw_point(x as u32, start.y as u32, &polygon.color());
            //}
            let pixel_num = end.point.x - start.point.x + 1;
            for pixel in start.interpolate(&end, pixel_num as usize) {
                if pixel.point.x < 0 || pixel.point.y < 0 {
                    continue;
                }
                self.pixel_shader(pixel, &polygon.color());
            }
        }
    }
}
