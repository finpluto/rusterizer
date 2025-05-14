use std::f32::consts::PI;

use glam::{IVec2, Vec2, Vec3};

use crate::{
    camera::Camera,
    geometry::primitives::{Pixel, Vertex},
    painter::PointPainter,
};

pub trait VertexShader {
    fn vertex_shader(&self, v: &Vertex) -> Pixel;
}

pub trait PixelShader {
    fn pixel_shader(&mut self, p: Pixel);
}

const LIGHT_POS: Vec3 = Vec3::new(0f32, -0.5, -0.7);
const LIGHT_POWER: Vec3 = Vec3::splat(14f32);
const INDIRECT_LIGHT_POWER_PER_AREA: Vec3 = Vec3::splat(0.5);

pub struct PixelShaderImpl<'pp, PP> {
    width: u32,
    height: u32,
    point_painter: &'pp mut PP,
    z_buf: Vec<f32>,
}

impl<'pp, PP: PointPainter> PixelShaderImpl<'pp, PP> {
    pub fn from_point_painter(pp: &'pp mut PP, height: u32, width: u32) -> Self {
        Self {
            width,
            height,
            point_painter: pp,
            z_buf: vec![0f32; (height * width) as usize],
        }
    }

    fn get_z_value_idx(&self, point: IVec2) -> usize {
        (self.width * (point.y as u32) + (point.x as u32)) as usize
    }
}

impl<PP: PointPainter> PixelShader for PixelShaderImpl<'_, PP> {
    fn pixel_shader(&mut self, pixel: Pixel) {
        let z_idx = self.get_z_value_idx(pixel.point);

        // FIXME: weird out of bound issue
        if z_idx >= (self.width * self.height) as usize {
            return;
        }
        let z_recip = self.z_buf[z_idx];
        if pixel.z_recip > z_recip {
            self.z_buf[z_idx] = pixel.z_recip;
            self.point_painter.draw_point(
                pixel.point.x as u32,
                pixel.point.y as u32,
                &pixel.illumination,
            );
        }
    }
}

pub struct VertexShaderImpl<'c> {
    camera: &'c Camera,
}

impl<'c> VertexShaderImpl<'c> {
    pub fn wrap_camera(camera: &'c Camera) -> Self {
        VertexShaderImpl { camera }
    }
}

impl VertexShader for VertexShaderImpl<'_> {
    fn vertex_shader(&self, vertex: &Vertex) -> Pixel {
        let v = self.camera.rotation * (vertex.point - self.camera.position);

        let focal = self.camera.focal as f32;
        let width = self.camera.width as f32;
        let height = self.camera.height as f32;

        let projected_point =
            (focal / v.z) * Vec2::new(v.x, v.y) + Vec2::new(width / 2f32, height / 2f32);

        // illumination calculation
        let r = LIGHT_POS - vertex.point;
        let n = vertex.normal;
        let d = (r.normalize().dot(n)).max(0f32) / (4f32 * PI * r.dot(r)) * LIGHT_POWER;

        let illumination = vertex.reflectance * (d + INDIRECT_LIGHT_POWER_PER_AREA);

        Pixel::new(
            projected_point.as_ivec2(),
            (self.camera.position.z - v.z).abs().recip(),
            illumination,
        )
    }
}
