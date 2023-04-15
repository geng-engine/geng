use super::*;

impl<T: Float> mat4<T> {
    /// Construct prespective projection matrix.
    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let ymax = near * (fov / (T::ONE + T::ONE)).tan();
        let xmax = ymax * aspect;
        Self::frustum(-xmax, xmax, -ymax, ymax, near, far)
    }

    /// Construct frustum projection matrix.
    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let double_near = near + near;
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        mat4::new([
            [
                double_near / width,
                T::ZERO,
                (right + left) / width,
                T::ZERO,
            ],
            [
                T::ZERO,
                double_near / height,
                (top + bottom) / height,
                T::ZERO,
            ],
            [
                T::ZERO,
                T::ZERO,
                (-far - near) / depth,
                (-double_near * far) / depth,
            ],
            [T::ZERO, T::ZERO, -T::ONE, T::ZERO],
        ])
    }
}
