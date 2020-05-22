use js_sys::Float32Array;
use std::ops::{Add, Mul};
#[derive(Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

impl Vec2 {
    pub fn flatten(vectors: &[Vec2]) -> Float32Array {
        let f32array = Float32Array::new_with_length((vectors.len() * 2) as u32);

        for (idx, v) in vectors.into_iter().enumerate() {
            f32array.set_index((idx * 2) as u32, v.0);
            f32array.set_index((idx * 2 + 1) as u32, v.1);
        }

        f32array
    }

    pub fn mix(&self, rhs: &Vec2, scale: f32) -> Vec2 {
        *self * scale + *rhs * (1.0 - scale)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}
#[derive(Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn flatten(vectors: &[Vec3]) -> Float32Array {
        let f32array = Float32Array::new_with_length((vectors.len() * 3) as u32);

        for (idx, v) in vectors.into_iter().enumerate() {
            f32array.set_index((idx * 3) as u32, v.0);
            f32array.set_index((idx * 3 + 1) as u32, v.1);
            f32array.set_index((idx * 3 + 2) as u32, v.2);
        }

        f32array
    }

    pub fn mix(&self, rhs: &Vec3, scale: f32) -> Vec3 {
        *self * scale + *rhs * (1.0 - scale)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
