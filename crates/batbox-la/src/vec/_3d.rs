use super::*;

/// 3 dimensional vector.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct vec3<T>(pub T, pub T, pub T);

impl<T: std::fmt::Display> std::fmt::Display for vec3<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> From<[T; 3]> for vec3<T> {
    fn from(v: [T; 3]) -> vec3<T> {
        let [x, y, z] = v;
        vec3(x, y, z)
    }
}

/// Data structure used to provide access to coordinates with the dot notation, e.g. `v.x`
pub struct XYZ<T> {
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
    #[allow(missing_docs)]
    pub z: T,
}

impl<T> Deref for XYZ<T> {
    type Target = [T; 3];
    fn deref(&self) -> &[T; 3] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for XYZ<T> {
    fn deref_mut(&mut self) -> &mut [T; 3] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Deref for vec3<T> {
    type Target = XYZ<T>;
    fn deref(&self) -> &XYZ<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for vec3<T> {
    fn deref_mut(&mut self) -> &mut XYZ<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> vec3<T> {
    /// Get the first two components as a [vec2]
    pub fn xy(self) -> vec2<T> {
        vec2(self.0, self.1)
    }

    /// Extend with another component and get a [vec4]
    pub fn extend(self, w: T) -> vec4<T> {
        vec4(self.0, self.1, self.2, w)
    }

    /// Map every component (coordinate)
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> vec3<U> {
        vec3(f(self.0), f(self.1), f(self.2))
    }
}

impl<T: Clone> vec3<T> {
    /// Construct a vector with all components set to specified value
    pub fn splat(value: T) -> Self {
        Self(value.clone(), value.clone(), value)
    }
}

impl<T: UNum> vec3<T> {
    /// A zero 3-d vector
    pub const ZERO: Self = vec3(T::ZERO, T::ZERO, T::ZERO);

    /// A unit X
    pub const UNIT_X: Self = Self(T::ONE, T::ZERO, T::ZERO);

    /// A unit Y
    pub const UNIT_Y: Self = Self(T::ZERO, T::ONE, T::ZERO);

    /// A unit Z
    pub const UNIT_Z: Self = Self(T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Num> vec3<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec3::dot(vec3(1, 2, 3), vec3(3, 4, 5)), 26);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// Calculate cross product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec3::cross(vec3(1, 2, 3), vec3(3, 4, 5)), vec3(-2, 4, -2));
    /// ```
    pub fn cross(a: Self, b: Self) -> Self {
        vec3(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

impl<T: Float> vec3<T> {
    /// Normalize a vector.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v: vec3<f64> = vec3(1.0, 2.0, 3.0);
    /// assert!((v.normalize().len() - 1.0).abs() < 1e-5);
    /// ```
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Normalizes a vector unless its length is approximately 0.
    /// Can be used to avoid division by 0.
    ///
    /// Uses [Approx::approx_eq] to determine equality to zero
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.normalize_or_zero(), v.normalize());
    /// let v = vec3(1e-10, 1e-10, 1e-10);
    /// assert_eq!(v.normalize_or_zero(), vec3::ZERO);
    /// ```
    pub fn normalize_or_zero(self) -> Self {
        let len = self.len();
        if len.approx_eq(&T::ZERO) {
            vec3::ZERO
        } else {
            self / len
        }
    }

    /// Calculate length of a vector.
    pub fn len(self) -> T {
        T::sqrt(self.len_sqr())
    }

    /// Calculate squared length of this vector
    pub fn len_sqr(self) -> T {
        vec3::dot(self, self)
    }

    /// Convert a homogenous 3d vector into 2d
    ///
    /// Same as self.xy() / self.z
    pub fn into_2d(self) -> vec2<T> {
        self.xy() / self.z
    }

    /// Clamp vector's length. Note that the range must be inclusive.
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.clamp_len(..=1.0), v.normalize());
    /// ```
    pub fn clamp_len(self, len_range: impl FixedRangeBounds<T>) -> Self {
        let len = self.len();
        let target_len = len.clamp_range(len_range);
        if len == target_len {
            self
        } else {
            self * target_len / len
        }
    }

    /// Clamp vector in range. Note the range must be inclusive.
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.clamp_coordinates(.., 0.0..=1.0, 5.0..), vec3(1.0, 1.0, 5.0));
    /// ```
    pub fn clamp_coordinates(
        self,
        x_range: impl FixedRangeBounds<T>,
        y_range: impl FixedRangeBounds<T>,
        z_range: impl FixedRangeBounds<T>,
    ) -> Self {
        vec3(
            self.x.clamp_range(x_range),
            self.y.clamp_range(y_range),
            self.z.clamp_range(z_range),
        )
    }

    /// Apply transformation matrix
    pub fn transform(self, transform: mat4<T>) -> Self {
        (transform * self.extend(T::ONE)).into_3d()
    }
}
