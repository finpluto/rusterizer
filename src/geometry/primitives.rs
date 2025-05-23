use std::cell::OnceCell;

use glam::{IVec2, Vec3};

use crate::{camera::Camera, shaders::VertexShader};

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
        let vs = camera.as_vertex_shader();
        let v0 = Vertex::new(self.v0, self.get_normal(), self.color);
        let v1 = Vertex::new(self.v1, self.get_normal(), self.color);
        let v2 = Vertex::new(self.v2, self.get_normal(), self.color);
        Triangle2D {
            v0: vs.vertex_shader(&v0),
            v1: vs.vertex_shader(&v1),
            v2: vs.vertex_shader(&v2),
        }
    }
}

pub struct Triangle2D {
    pub v0: Pixel,
    pub v1: Pixel,
    pub v2: Pixel,
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

#[derive(Clone, Copy)]
pub struct Pixel {
    pub point: IVec2,
    pub z_recip: f32,
    pub illumination: Vec3,
}

impl Pixel {
    pub fn new(point: IVec2, z_recip: f32, illumination: Vec3) -> Self {
        Self {
            point,
            z_recip,
            illumination,
        }
    }

    pub fn xyz_as_vec3(&self) -> Vec3 {
        let point = self.point.as_vec2();
        Vec3::new(point.x, point.y, self.z_recip)
    }
}

pub struct Vertex {
    pub point: Vec3,
    pub normal: Vec3,
    pub reflectance: Vec3,
}

impl Vertex {
    pub fn new(point: Vec3, normal: Vec3, reflectance: Vec3) -> Self {
        Self {
            point,
            normal,
            reflectance,
        }
    }
}
