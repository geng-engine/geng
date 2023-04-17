use super::*;

impl<T: Copy> mat3<T> {
    /// Get transposed matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat3::translate(vec2(1, 2));
    /// let matrix_transposed = matrix.transpose();
    /// for i in 0..3 {
    ///     for j in 0..3 {
    ///         assert_eq!(matrix[(i, j)], matrix_transposed[(j, i)]);
    ///     }
    /// }
    /// ```
    pub fn transpose(self) -> Self {
        let mut result = self;
        for i in 0..3 {
            for j in 0..3 {
                result[(i, j)] = self[(j, i)];
            }
        }
        result
    }
}

impl<T: Num> mat3<T> {
    /// Construct a transformation matrix with given orts
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let e1 = vec2(1.0, 2.0);
    /// let e2 = vec2(3.0, 4.0);
    /// let m = mat3::from_orts(e1, e2);
    /// assert_eq!(vec2(1.0, 0.0).transform(m), e1);
    /// assert_eq!(vec2(0.0, 1.0).transform(m), e2);
    /// let v = vec2(2.0, 3.0);
    /// assert_eq!(v.transform(m), e1 * v.x + e2 * v.y);
    /// ```
    pub fn from_orts(x: vec2<T>, y: vec2<T>) -> Self {
        mat3::new([
            [x.x, y.x, T::ZERO],
            [x.y, y.y, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }

    /// Extend this into a 3d transformation matrix, leaving z coordinate as is
    pub fn extend3d(self) -> mat4<T> {
        let Self([[a00, a01, a02], [a10, a11, a12], [a20, a21, a22]]) = self;
        mat4([
            [a00, a01, T::ZERO, a02],
            [a10, a11, T::ZERO, a12],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [a20, a21, T::ZERO, a22],
        ])
    }
}

impl<T: Float> mat3<T> {
    /// Get inverse matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat3::<f64>::rotate(0.123);
    /// let matrix_inv = matrix.inverse();
    /// let mult = matrix * matrix_inv;
    /// for i in 0..3 {
    ///     for j in 0..3 {
    ///         assert!((mult[(i, j)] - if i == j { 1.0 } else { 0.0 }).abs() < 1e-5);
    ///     }
    /// }
    /// ```
    pub fn inverse(self) -> Self {
        let b01 = self[(2, 2)] * self[(1, 1)] - self[(2, 1)] * self[(1, 2)];
        let b11 = -self[(2, 2)] * self[(0, 1)] + self[(2, 1)] * self[(0, 2)];
        let b21 = self[(1, 2)] * self[(0, 1)] - self[(1, 1)] * self[(0, 2)];

        let det = self[(0, 0)] * b01 + self[(1, 0)] * b11 + self[(2, 0)] * b21;

        if det == T::ZERO {
            Self::identity()
        } else {
            mat3::new([
                [b01, b11, b21],
                [
                    -self[(2, 2)] * self[(1, 0)] + self[(2, 0)] * self[(1, 2)],
                    self[(2, 2)] * self[(0, 0)] - self[(2, 0)] * self[(0, 2)],
                    -self[(1, 2)] * self[(0, 0)] + self[(1, 0)] * self[(0, 2)],
                ],
                [
                    self[(2, 1)] * self[(1, 0)] - self[(2, 0)] * self[(1, 1)],
                    -self[(2, 1)] * self[(0, 0)] + self[(2, 0)] * self[(0, 1)],
                    self[(1, 1)] * self[(0, 0)] - self[(1, 0)] * self[(0, 1)],
                ],
            ]) / det
        }
    }
}

#[test]
fn test_mat_inverse_random() {
    fn check(m: mat3<f64>) {
        let m_inv = m.inverse();
        let mul = m * m_inv;
        assert!(mul.approx_eq(&mat3::identity()));
    }
    // Random generated test cases
    check(mat3::new([
        [8.7, 3.6, 6.5],
        [7.4, 5.8, 8.6],
        [1.8, 8.3, 6.6],
    ]));
    check(mat3::new([
        [9.6, 0.6, 5.4],
        [0.5, 0.1, 5.4],
        [2.0, 3.8, 0.0],
    ]));
    check(mat3::new([
        [6.1, 1.7, 2.7],
        [1.8, 2.5, 7.8],
        [2.6, 9.5, 1.5],
    ]));
    check(mat3::new([
        [8.4, 1.0, 6.4],
        [1.6, 1.1, 1.5],
        [5.5, 1.2, 0.6],
    ]));
    check(mat3::new([
        [0.6, 5.7, 0.2],
        [3.5, 1.7, 6.4],
        [2.0, 3.4, 4.1],
    ]));
    check(mat3::new([
        [1.9, 1.4, 4.0],
        [7.8, 8.2, 9.1],
        [3.2, 4.4, 3.9],
    ]));
    check(mat3::new([
        [6.0, 7.0, 4.0],
        [0.9, 8.7, 6.2],
        [0.2, 4.6, 3.7],
    ]));
    check(mat3::new([
        [5.1, 9.6, 4.6],
        [1.5, 9.2, 2.3],
        [5.6, 7.6, 0.4],
    ]));
    check(mat3::new([
        [3.0, 5.7, 8.6],
        [6.2, 7.2, 0.1],
        [5.3, 5.9, 5.8],
    ]));
    check(mat3::new([
        [4.3, 8.4, 2.0],
        [2.3, 9.0, 4.6],
        [5.5, 1.2, 8.8],
    ]));
}
