use std::cell::OnceCell;

use glam::{IVec2, Vec3};

use crate::camera::Camera;

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Vec3,
    normal: OnceCell<Vec3>,
}

impl Triangle {
    pub const fn new(v0: Vec3, v1: Vec3, v2: Vec3, color: Vec3) -> Self {
        Self {
            v0,
            v1,
            v2,
            color,
            normal: OnceCell::new(),
        }
    }

    // This normal orientation is important,
    // a flipped normal will influence illumination model.
    pub fn get_normal(&self) -> Vec3 {
        *self
            .normal
            .get_or_init(|| ((self.v2 - self.v0).cross(self.v1 - self.v0)).normalize())
    }

    pub fn project_to_canvas(&self, camera: &Camera) -> Triangle2D {
        Triangle2D {
            v0: camera.vertex_shader(&self.v0),
            v1: camera.vertex_shader(&self.v1),
            v2: camera.vertex_shader(&self.v2),
            color: self.color,
        }
    }
}

pub struct Triangle2D {
    pub v0: Pixel,
    pub v1: Pixel,
    pub v2: Pixel,
    pub color: Vec3,
}

pub trait Vertices {
    type Vertex;
    fn vertices(&self) -> impl AsRef<[Self::Vertex]>;
}

impl Vertices for Triangle {
    type Vertex = Vec3;

    fn vertices(&self) -> impl AsRef<[Self::Vertex]> {
        [self.v0, self.v1, self.v2]
    }
}

impl Vertices for Triangle2D {
    type Vertex = Pixel;

    fn vertices(&self) -> impl AsRef<[Self::Vertex]> {
        [self.v0, self.v1, self.v2]
    }
}

pub trait ColorPicker {
    fn color(&self) -> Vec3;
}

impl ColorPicker for Triangle2D {
    fn color(&self) -> Vec3 {
        self.color
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub point: IVec2,
    pub z_recip: f32,
}

impl Pixel {
    pub fn new(point: IVec2, z_recip: f32) -> Self {
        Self { point, z_recip }
    }

    pub fn as_vec3(&self) -> Vec3 {
        let point = self.point.as_vec2();
        Vec3::new(point.x, point.y, self.z_recip)
    }
}
