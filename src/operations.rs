use std::{
    iter::Map,
    ops::AddAssign,
};

use glam::{IVec2, Vec2, Vec3};

use crate::geometry::primitives::Pixel;

pub trait Interpolate<RHS> {
    type Output: Iterator<Item = RHS>;
    fn interpolate(&self, rhs: &RHS, result_size: usize) -> Self::Output;
}

pub struct LinePoints<T> {
    cursor: T,
    step: T,
    rounds: usize,
}

impl<AddAssignable: AddAssign + Copy> Iterator for LinePoints<AddAssignable> {
    type Item = AddAssignable;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rounds == 0 {
            return None;
        }
        let cursor = self.cursor;
        self.cursor += self.step;
        self.rounds -= 1;
        Some(cursor)
    }
}

impl Interpolate<Vec2> for Vec2 {
    type Output = LinePoints<Vec2>;

    fn interpolate(&self, rhs: &Vec2, result_size: usize) -> Self::Output {
        LinePoints {
            cursor: *self,
            step: (rhs - self) / std::cmp::max(result_size - 1, 1) as f32,
            rounds: result_size,
        }
    }
}

impl Interpolate<Vec3> for Vec3 {
    type Output = LinePoints<Vec3>;

    fn interpolate(&self, rhs: &Vec3, result_size: usize) -> Self::Output {
        LinePoints {
            cursor: *self,
            step: (rhs - self) / std::cmp::max(result_size - 1, 1) as f32,
            rounds: result_size,
        }
    }
}

type InterIVec2 = Map<LinePoints<Vec2>, fn(Vec2) -> IVec2>;

impl Interpolate<IVec2> for IVec2 {
    type Output = InterIVec2;

    fn interpolate(&self, rhs: &IVec2, result_size: usize) -> Self::Output {
        self.as_vec2()
            .interpolate(&rhs.as_vec2(), result_size)
            .map(cast_vec2_to_ivec2)
    }
}

fn cast_vec2_to_ivec2(v: Vec2) -> IVec2 {
    IVec2::new(v.x.round() as i32, v.y.round() as i32)
}

pub struct InterPixels {
    pos_and_z_iter: LinePoints<Vec3>,
    illu_iter: LinePoints<Vec3>,
}

impl Iterator for InterPixels {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.pos_and_z_iter.next(), self.illu_iter.next()) {
            (Some(pos_and_z), Some(illumination)) => Some(Pixel::new(
                IVec2::new(pos_and_z.x.round() as i32, pos_and_z.y.round() as i32),
                pos_and_z.z,
                illumination,
            )),
            _ => None,
        }
    }
}

impl Interpolate<Pixel> for Pixel {
    type Output = InterPixels;

    fn interpolate(&self, rhs: &Pixel, result_size: usize) -> Self::Output {
        InterPixels {
            pos_and_z_iter: self.xyz_as_vec3().interpolate(&rhs.xyz_as_vec3(), result_size),
            illu_iter: self
                .illumination
                .interpolate(&rhs.illumination, result_size),
        }
    }
}

#[cfg(test)]
mod test {
    use glam::{IVec2, ivec2};

    use super::Interpolate;

    #[test]
    fn interpolate() {
        let points: Vec<IVec2> = ivec2(5, 3).interpolate(&ivec2(1, 2), 4).collect();
        assert_eq!(Vec::<IVec2>::new(), points);
    }
}
