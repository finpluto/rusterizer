use glam::Vec3;

use crate::{
    camera::Camera,
    geometry::primitives::Triangle,
    painter::PolygonFiller,
    pixels::PixelBuffer,
    scene::cornell::{ROOM, SHORT_BLOCK, TALL_BLOCK, scale_triangle},
    shaders::PixelShaderImpl,
};

pub struct World {
    camera: Camera,
    triangles: Vec<Triangle>,
    light_offset: Vec3,
}

impl World {
    pub fn new(height: u32, width: u32) -> Self {
        let triangles = [ROOM, SHORT_BLOCK, TALL_BLOCK]
            .into_iter()
            .flatten()
            .map(scale_triangle)
            .collect();

        Self {
            camera: Camera::new(height, width),
            triangles,
            light_offset: Vec3::ZERO,
        }
    }

    pub fn draw(&self, mut writer: PixelBuffer) {
        writer.memset(0);
        let mut ps = PixelShaderImpl::from_point_painter(
            &mut writer,
            self.camera.height,
            self.camera.width,
            self.light_offset,
        );
        for triangle2d in self
            .triangles
            .iter()
            .map(|t| t.project_to_canvas(&self.camera))
        {
            ps.fill_polygon(triangle2d);
        }
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.camera.set_yaw(yaw);
    }

    pub fn get_canvas_size(&self) -> (u32, u32) {
        (self.camera.height, self.camera.width)
    }

    pub fn update_light_offset(&mut self, x: f32, y: f32, z: f32) {
        self.light_offset = Vec3::new(x, y, z);
    }
}
