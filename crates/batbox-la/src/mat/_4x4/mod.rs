use super::*;

mod extra;
mod ops;
mod projection;
mod transform;

/// 4x4 matrix
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct mat4<T>(pub(crate) [[T; 4]; 4]);

impl<T> mat4<T> {
    /// Map every element
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> mat4<U> {
        mat4(self.0.map(|row| row.map(&f)))
    }
}

impl<T: Copy> mat4<T> {
    /// Construct a matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat4::new([
    ///     [1, 2, 3, 4],
    ///     [3, 4, 5, 6],
    ///     [5, 6, 7, 8],
    ///     [0, 5, 2, 9],
    /// ]);
    /// ```
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Self(values).transpose()
    }

    /// Get a row as a [vec4]
    pub fn row(&self, row_index: usize) -> vec4<T> {
        vec4(
            self[(row_index, 0)],
            self[(row_index, 1)],
            self[(row_index, 2)],
            self[(row_index, 3)],
        )
    }

    /// Get a column as a [vec4]
    pub fn col(&self, col_index: usize) -> vec4<T> {
        vec4(
            self[(0, col_index)],
            self[(1, col_index)],
            self[(2, col_index)],
            self[(3, col_index)],
        )
    }
}

impl<T> Index<(usize, usize)> for mat4<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.0[col][row]
    }
}

impl<T> IndexMut<(usize, usize)> for mat4<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.0[col][row]
    }
}

impl<T> mat4<T> {
    /// Get self as a flat array
    pub fn as_flat_array(&self) -> &[T; 16] {
        unsafe { std::mem::transmute(self) }
    }
    /// Get self as a mutable flat array
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 16] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Num + Copy> mat4<T> {
    /// Construct zero matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat4::<i32>::zero();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[(i, j)], 0);
    ///     }
    /// }
    /// ```
    pub fn zero() -> Self {
        mat4([[T::ZERO; 4]; 4])
    }

    /// Construct identity matrix.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let matrix = mat4::<i32>::identity();
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

impl<T: Float> Approx for mat4<T> {
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
