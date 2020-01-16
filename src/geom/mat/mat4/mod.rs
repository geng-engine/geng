use crate::*;

mod extra;
mod ops;
mod projection;
mod transform;

/// 4x4 matrix
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Mat4<T>([[T; 4]; 4]);

impl<T> Mat4<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Mat4<U> {
        fn map_arr<T, U, F: Fn(T) -> U>(arr: [T; 4], f: F) -> [U; 4] {
            let [a, b, c, d] = arr;
            [f(a), f(b), f(c), f(d)]
        }
        Self(map_arr(self.0, |row| map_arr(row, &f)))
    }
}

impl<T: Copy> Mat4<T> {
    /// Construct a matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let matrix = Mat4::new([
    ///     [1, 2, 3, 4],
    ///     [3, 4, 5, 6],
    ///     [5, 6, 7, 8],
    ///     [0, 5, 2, 9],
    /// ]);
    /// ```
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Self { 0: values }.transpose()
    }

    pub fn row(&self, row_index: usize) -> Vec4<T> {
        vec4(
            self[(row_index, 0)],
            self[(row_index, 1)],
            self[(row_index, 2)],
            self[(row_index, 3)],
        )
    }
}

impl<T> Index<(usize, usize)> for Mat4<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.0[col][row]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat4<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.0[col][row]
    }
}

impl<T> Mat4<T> {
    pub fn as_flat_array(&self) -> &[T; 16] {
        unsafe { mem::transmute(self) }
    }
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 16] {
        unsafe { mem::transmute(self) }
    }
}

impl<T: Num + Copy> Mat4<T> {
    /// Construct zero matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let matrix = Mat4::<i32>::zero();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[(i, j)], 0);
    ///     }
    /// }
    /// ```
    pub fn zero() -> Self {
        Mat4([[T::ZERO; 4]; 4])
    }

    /// Construct identity matrix.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let matrix = Mat4::<i32>::identity();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[(i, j)], if i == j { 1 } else { 0 });
    ///     }
    /// }
    /// ```
    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result[(i, i)] = T::ONE;
        }
        result
    }
}

impl<T: Float> ApproxEq for Mat4<T> {
    fn approx_distance_to(&self, other: &Self) -> f32 {
        let mut dist = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                dist = partial_max(dist, (other[(i, j)] - self[(i, j)]).abs().as_f32());
            }
        }
        dist
    }
}
