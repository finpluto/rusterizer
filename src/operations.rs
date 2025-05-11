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

impl Iterator for LinePoints<Vec2> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rounds == 0 {
            return None;
        }
        let cursor = self.cursor;
        self.cursor += self.step;
        self.rounds -= 1;
        Some(IVec2::new(cursor.x.round() as i32, cursor.y.round() as i32))
    }
}

impl Interpolate<IVec2> for IVec2 {
    fn interpolate(&self, rhs: &IVec2, result_size: usize) -> Self::Output {
        LinePoints {
            cursor: self.as_vec2(),
            step: (rhs - self).as_vec2() / std::cmp::max(result_size - 1, 1) as f32,
            rounds: result_size,
        }
    }

    type Output = LinePoints<Vec2>;
}

impl Iterator for LinePoints<Vec3> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rounds == 0 {
            return None;
        }
        let cursor = self.cursor;
        self.cursor += self.step;
        self.rounds -= 1;
        Some(Pixel {
            point: IVec2::new(cursor.x.round() as i32, cursor.y.round() as i32),
            z_recip: cursor.z,
        })
    }
}

impl Interpolate<Pixel> for Pixel {
    type Output = LinePoints<Vec3>;

    fn interpolate(&self, rhs: &Pixel, result_size: usize) -> Self::Output {
        LinePoints {
            cursor: self.as_vec3(),
            step: (rhs.as_vec3() - self.as_vec3()) / std::cmp::max(result_size - 1, 1) as f32,
            rounds: result_size,
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
