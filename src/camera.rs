use glam::{Mat3, Vec2, Vec3};

use crate::geometry::primitives::Pixel;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    focal: u32,
    position: Vec3,
    rotation: Mat3,
}

impl Camera {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            width,
            height,
            focal: width,
            position: Vec3::new(0f32, 0f32, -3.001),
            rotation: Mat3::IDENTITY,
        }
    }

    pub fn vertex_shader(&self, v: &Vec3) -> Pixel {
        let v = self.rotation * (v - self.position);

        let focal = self.focal as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        let projected_point =
            (focal / v.z) * Vec2::new(v.x, v.y) + Vec2::new(width / 2f32, height / 2f32);
        Pixel::new(
            projected_point.as_ivec2(),
            (self.position.z - v.z).abs().recip(),
        )
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.rotation = Mat3::from_rotation_y(yaw);
    }
}
