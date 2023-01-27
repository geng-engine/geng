use super::*;

impl<T: Float> Mat3<T> {
    /// Get 2d part of the orthographic projection matrix
    pub fn ortho(aabb: Aabb2<T>) -> Self {
        let Aabb2 {
            min: Vec2 { x: l, y: b },
            max: Vec2 { x: r, y: t },
        } = aabb;
        let two = T::ONE + T::ONE;
        Self::new([
            [two / (r - l), T::ZERO, -(r + l) / (r - l)],
            [T::ZERO, two / (t - b), -(t + b) / (t - b)],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }
}
