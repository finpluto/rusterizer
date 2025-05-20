use glam::{Mat3, Vec3};

use crate::shaders::VertexShaderImpl;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub focal: u32,
    pub position: Vec3,
    pub rotation: Mat3,
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

    pub fn set_yaw(&mut self, yaw: f32) {
        self.rotation = Mat3::from_rotation_y(yaw);
    }

    pub fn set_z_translate(&mut self, amount: f32) {
        self.position.z += amount;
    }

    pub fn as_vertex_shader(&self) -> VertexShaderImpl {
        VertexShaderImpl::wrap_camera(self)
    }
}
