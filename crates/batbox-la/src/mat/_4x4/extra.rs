use super::*;

impl<T: Copy> mat4<T> {
    /// Get transposed matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat4::translate(vec3(1, 2, 3));
    /// let matrix_transposed = matrix.transpose();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[(i, j)], matrix_transposed[(j, i)]);
    ///     }
    /// }
    /// ```
    pub fn transpose(self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = self[(j, i)];
            }
        }
        result
    }
}

impl<T: Num> mat4<T> {
    /// Construct a transformation matrix with given orts
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let e1 = vec3(1.0, 2.0, 3.0);
    /// let e2 = vec3(3.0, 4.0, 5.0);
    /// let e3 = vec3(5.0, 6.0, -1.0);
    /// let m = mat4::from_orts(e1, e2, e3);
    /// assert_eq!(vec3(1.0, 0.0, 0.0).transform(m), e1);
    /// assert_eq!(vec3(0.0, 1.0, 0.0).transform(m), e2);
    /// assert_eq!(vec3(0.0, 0.0, 1.0).transform(m), e3);
    /// let v = vec3(2.0, 3.0, 4.0);
    /// assert_eq!(v.transform(m), e1 * v.x + e2 * v.y + e3 * v.z);
    /// ```
    pub fn from_orts(x: vec3<T>, y: vec3<T>, z: vec3<T>) -> Self {
        mat4::new([
            [x.x, y.x, z.x, T::ZERO],
            [x.y, y.y, z.y, T::ZERO],
            [x.z, y.z, z.z, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

impl<T: Float> mat4<T> {
    /// Get inverse matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat4::<f64>::rotate_x(0.123);
    /// let matrix_inv = matrix.inverse();
    /// let mult = matrix * matrix_inv;
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert!((mult[(i, j)] - if i == j { 1.0 } else { 0.0 }).abs() < 1e-5);
    ///     }
    /// }
    /// ```
    pub fn inverse(self) -> Self {
        let b00 = self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)];
        let b01 = self[(0, 0)] * self[(2, 1)] - self[(2, 0)] * self[(0, 1)];
        let b02 = self[(0, 0)] * self[(3, 1)] - self[(3, 0)] * self[(0, 1)];
        let b03 = self[(1, 0)] * self[(2, 1)] - self[(2, 0)] * self[(1, 1)];
        let b04 = self[(1, 0)] * self[(3, 1)] - self[(3, 0)] * self[(1, 1)];
        let b05 = self[(2, 0)] * self[(3, 1)] - self[(3, 0)] * self[(2, 1)];
        let b06 = self[(0, 2)] * self[(1, 3)] - self[(1, 2)] * self[(0, 3)];
        let b07 = self[(0, 2)] * self[(2, 3)] - self[(2, 2)] * self[(0, 3)];
        let b08 = self[(0, 2)] * self[(3, 3)] - self[(3, 2)] * self[(0, 3)];
        let b09 = self[(1, 2)] * self[(2, 3)] - self[(2, 2)] * self[(1, 3)];
        let b10 = self[(1, 2)] * self[(3, 3)] - self[(3, 2)] * self[(1, 3)];
        let b11 = self[(2, 2)] * self[(3, 3)] - self[(3, 2)] * self[(2, 3)];

        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det == T::ZERO {
            Self::identity()
        } else {
            mat4::new([
                [
                    self[(1, 1)] * b11 - self[(2, 1)] * b10 + self[(3, 1)] * b09,
                    self[(2, 1)] * b08 - self[(0, 1)] * b11 - self[(3, 1)] * b07,
                    self[(0, 1)] * b10 - self[(1, 1)] * b08 + self[(3, 1)] * b06,
                    self[(1, 1)] * b07 - self[(0, 1)] * b09 - self[(2, 1)] * b06,
                ],
                [
                    self[(2, 0)] * b10 - self[(1, 0)] * b11 - self[(3, 0)] * b09,
                    self[(0, 0)] * b11 - self[(2, 0)] * b08 + self[(3, 0)] * b07,
                    self[(1, 0)] * b08 - self[(0, 0)] * b10 - self[(3, 0)] * b06,
                    self[(0, 0)] * b09 - self[(1, 0)] * b07 + self[(2, 0)] * b06,
                ],
                [
                    self[(1, 3)] * b05 - self[(2, 3)] * b04 + self[(3, 3)] * b03,
                    self[(2, 3)] * b02 - self[(0, 3)] * b05 - self[(3, 3)] * b01,
                    self[(0, 3)] * b04 - self[(1, 3)] * b02 + self[(3, 3)] * b00,
                    self[(1, 3)] * b01 - self[(0, 3)] * b03 - self[(2, 3)] * b00,
                ],
                [
                    self[(2, 2)] * b04 - self[(1, 2)] * b05 - self[(3, 2)] * b03,
                    self[(0, 2)] * b05 - self[(2, 2)] * b02 + self[(3, 2)] * b01,
                    self[(1, 2)] * b02 - self[(0, 2)] * b04 - self[(3, 2)] * b00,
                    self[(0, 2)] * b03 - self[(1, 2)] * b01 + self[(2, 2)] * b00,
                ],
            ]) / det
        }
    }
}

#[test]
fn test_mat_inverse_random() {
    fn check(m: mat4<f64>) {
        let m_inv = m.inverse();
        let mul = m * m_inv;
        assert!(mul.approx_eq(&mat4::identity()));
    }
    // Random generated test cases
    check(mat4::new([
        [8.7, 3.6, 6.5, 6.5],
        [7.4, 5.8, 8.6, 2.6],
        [1.8, 8.3, 6.6, 4.9],
        [2.1, 3.4, 3.5, 8.8],
    ]));
    check(mat4::new([
        [9.6, 0.6, 5.4, 3.2],
        [0.5, 0.1, 5.4, 6.6],
        [2.0, 3.8, 0.0, 1.4],
        [6.6, 2.9, 4.3, 9.3],
    ]));
    check(mat4::new([
        [6.1, 1.7, 2.7, 3.6],
        [1.8, 2.5, 7.8, 7.1],
        [2.6, 9.5, 1.5, 8.0],
        [6.5, 6.5, 5.9, 7.2],
    ]));
    check(mat4::new([
        [8.4, 1.0, 6.4, 0.0],
        [1.6, 1.1, 1.5, 4.0],
        [5.5, 1.2, 0.6, 8.3],
        [9.1, 9.7, 8.7, 0.8],
    ]));
    check(mat4::new([
        [0.6, 5.7, 0.2, 7.1],
        [3.5, 1.7, 6.4, 1.6],
        [2.0, 3.4, 4.1, 5.0],
        [7.3, 5.9, 8.9, 3.0],
    ]));
    check(mat4::new([
        [1.9, 1.4, 4.0, 3.7],
        [7.8, 8.2, 9.1, 1.3],
        [3.2, 4.4, 3.9, 2.5],
        [7.1, 7.3, 3.5, 5.0],
    ]));
    check(mat4::new([
        [6.0, 7.0, 4.0, 0.6],
        [0.9, 8.7, 6.2, 6.0],
        [0.2, 4.6, 3.7, 0.2],
        [8.4, 6.2, 7.6, 2.8],
    ]));
    check(mat4::new([
        [5.1, 9.6, 4.6, 4.5],
        [1.5, 9.2, 2.3, 9.4],
        [5.6, 7.6, 0.4, 2.9],
        [0.6, 0.5, 4.3, 4.6],
    ]));
    check(mat4::new([
        [3.0, 5.7, 8.6, 2.1],
        [6.2, 7.2, 0.1, 6.7],
        [5.3, 5.9, 5.8, 2.1],
        [9.4, 9.9, 9.8, 5.8],
    ]));
    check(mat4::new([
        [4.3, 8.4, 2.0, 6.2],
        [2.3, 9.0, 4.6, 5.8],
        [5.5, 1.2, 8.8, 1.6],
        [4.5, 9.3, 4.8, 1.5],
    ]));
}
