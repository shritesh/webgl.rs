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
