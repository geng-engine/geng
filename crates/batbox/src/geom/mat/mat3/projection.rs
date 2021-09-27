use super::*;

impl<T: Float> Mat3<T> {
    pub fn ortho(aabb: AABB<T>) -> Self {
        let AABB {
            x_min: l,
            x_max: r,
            y_min: b,
            y_max: t,
        } = aabb;
        let two = T::ONE + T::ONE;
        Self::new([
            [two / (r - l), T::ZERO, -(r + l) / (r - l)],
            [T::ZERO, two / (t - b), -(t + b) / (t - b)],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }
}
