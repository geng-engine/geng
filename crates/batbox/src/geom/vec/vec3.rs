use super::*;

/// 3 dimensional vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec3<T> {
    /// `x` coordinate of the vector
    pub x: T,
    /// `y` coordinate of the vector
    pub y: T,
    /// `z` coordinate of the vector
    pub z: T,
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Construct a 3-d vector with given components.
///
/// # Example
/// ```
/// use batbox::prelude::*;
/// let v = vec3(1, 2, 3);
/// ```
pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Vec3<T> {
        let [x, y, z] = v;
        vec3(x, y, z)
    }
}

impl<T> Deref for Vec3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &[T; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec3<T> {
    fn deref_mut(&mut self) -> &mut [T; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> Vec3<T> {
    pub fn xy(self) -> Vec2<T> {
        vec2(self.x, self.y)
    }
    pub fn extend(self, w: T) -> Vec4<T> {
        vec4(self.x, self.y, self.z, w)
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec3<U> {
        vec3(f(self.x), f(self.y), f(self.z))
    }
}

impl<T: UNum> Vec3<T> {
    /// A zero 3-d vector
    pub const ZERO: Self = vec3(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Copy + Num> Vec3<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// assert_eq!(Vec3::dot(vec3(1, 2, 3), vec3(3, 4, 5)), 26);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// Calculate cross product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// assert_eq!(Vec3::cross(vec3(1, 2, 3), vec3(3, 4, 5)), vec3(-2, 4, -2));
    /// ```
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

impl<T: Float> Vec3<T> {
    /// Normalize a vector.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let v: Vec3<f64> = vec3(1.0, 2.0, 3.0);
    /// assert!((v.normalize().len() - 1.0).abs() < 1e-5);
    /// ```
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Normalizes a vector unless its length its approximately 0.
    /// Can be used to avoid division by 0.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let v = vec3(1.0, 2.0, 3.0);
    /// assert_eq!(v.normalize_or_zero(), v.normalize());
    /// let v = vec3(1e-10, 1e-10, 1e-10);
    /// assert_eq!(v.normalize_or_zero(), Vec3::ZERO);
    /// ```
    pub fn normalize_or_zero(self) -> Self {
        let len = self.len();
        if len.approx_eq(&T::ZERO) {
            Vec3::ZERO
        } else {
            self / len
        }
    }

    /// Calculate length of a vector.
    pub fn len(self) -> T {
        T::sqrt(self.len_sqr())
    }

    pub fn len_sqr(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn into_2d(self) -> Vec2<T> {
        vec2(self.x / self.z, self.y / self.z)
    }

    /// Clamp vector's length. Note that the range must be inclusive.
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
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
    /// use batbox::prelude::*;
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
}
