use crate::{
    camera::Camera,
    geometry::primitives::Triangle,
    painter::PolygonFiller,
    pixels::PixelBuffer,
    scene::cornell::{ROOM, SHORT_BLOCK, TALL_BLOCK, scale_triangle},
};

pub struct World {
    camera: Camera,
    triangles: Vec<Triangle>,
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
        }
    }

    pub fn draw(&self, mut writer: PixelBuffer) {
        writer.memset(0);
        for triangle2d in self
            .triangles
            .iter()
            .map(|t| t.project_to_canvas(&self.camera))
        {
            writer.fill_polygon(triangle2d);
        }
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.camera.set_yaw(yaw);
    }

    pub fn get_canvas_size(&self) -> (u32, u32) {
        (self.camera.height, self.camera.width)
    }
}
