use js_sys::Float32Array;
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
