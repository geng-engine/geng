use super::*;

impl<T: Num + Copy> mat3<T> {
    /// Construct a uniform scale matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat3::scale_uniform(2);
    /// assert_eq!(matrix * vec3(1, 2, 1), vec3(2, 4, 1));
    /// ```
    pub fn scale_uniform(factor: T) -> Self {
        Self::scale(vec2(factor, factor))
    }

    /// Construct matrix that performs uniform scaling around a specified point
    pub fn scale_uniform_around(p: vec2<T>, scale: T) -> Self {
        mat3::translate(p) * mat3::scale_uniform(scale) * mat3::translate(-p)
    }

    /// Construct matrix that performs scaling around a specified point
    pub fn scale_around(p: vec2<T>, scale: vec2<T>) -> Self {
        mat3::translate(p) * mat3::scale(scale) * mat3::translate(-p)
    }

    /// Construct a scale matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat3::scale(vec2(1, 2));
    /// assert_eq!(matrix * vec3(1, 2, 1), vec3(1, 4, 1));
    /// ```
    pub fn scale(factor: vec2<T>) -> Self {
        let mut result = Self::zero();
        result[(0, 0)] = factor.x;
        result[(1, 1)] = factor.y;
        result[(2, 2)] = T::ONE;
        result
    }

    /// Construct a translation matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat3::translate(vec2(3, 2));
    /// assert_eq!(matrix * vec3(1, 2, 1), vec3(4, 4, 1));
    /// ```
    pub fn translate(dv: vec2<T>) -> Self {
        let mut result = Self::identity();
        result[(0, 2)] = dv.x;
        result[(1, 2)] = dv.y;
        result
    }
}

impl<T: Float> mat3<T> {
    /// Construct rotational matrix
    pub fn rotate(angle: Angle<T>) -> Self {
        let mut result = Self::identity();
        let (sin, cos) = angle.sin_cos();
        result[(0, 0)] = cos;
        result[(0, 1)] = -sin;
        result[(1, 0)] = sin;
        result[(1, 1)] = cos;
        result
    }

    /// Construct matrix that performs rotation around a specified point
    pub fn rotate_around(p: vec2<T>, angle: Angle<T>) -> Self {
        Self::translate(p) * Self::rotate(angle) * Self::translate(-p)
    }
}
