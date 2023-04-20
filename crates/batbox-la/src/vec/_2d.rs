use super::*;

/// 2 dimensional vector.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct vec2<T>(pub T, pub T);

impl<T: std::fmt::Display> std::fmt::Display for vec2<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {})", self.0, self.1)
    }
}

impl<T> From<[T; 2]> for vec2<T> {
    fn from(v: [T; 2]) -> vec2<T> {
        let [x, y] = v;
        vec2(x, y)
    }
}

/// Data structure used to provide access to coordinates with the dot notation, e.g. `v.x`
pub struct XY<T> {
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
}

impl<T> Deref for XY<T> {
    type Target = [T; 2];
    fn deref(&self) -> &[T; 2] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for XY<T> {
    fn deref_mut(&mut self) -> &mut [T; 2] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Deref for vec2<T> {
    type Target = XY<T>;
    fn deref(&self) -> &XY<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for vec2<T> {
    fn deref_mut(&mut self) -> &mut XY<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> vec2<T> {
    /// Extend into a 3-d vector.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec2(1, 2).extend(3), vec3(1, 2, 3));
    /// ```
    pub fn extend(self, z: T) -> vec3<T> {
        vec3(self.0, self.1, z)
    }

    /// Map every component (coordinate)
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> vec2<U> {
        vec2(f(self.0), f(self.1))
    }
}

impl<T: Clone> vec2<T> {
    /// Construct a vector with all components set to specified value
    pub fn splat(value: T) -> Self {
        Self(value.clone(), value)
    }
}

impl<T: UNum> vec2<T> {
    /// A zero 2-d vector
    pub const ZERO: Self = vec2(T::ZERO, T::ZERO);

    /// A unit X
    pub const UNIT_X: Self = Self(T::ONE, T::ZERO);

    /// A unit Y
    pub const UNIT_Y: Self = Self(T::ZERO, T::ONE);
}

impl<T: Num> vec2<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec2::dot(vec2(1, 2), vec2(3, 4)), 11);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }

    /// Calculate skew product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec2::skew(vec2(1, 2), vec2(3, 4)), -2);
    /// ```
    pub fn skew(a: Self, b: Self) -> T {
        a.x * b.y - a.y * b.x
    }
}

impl<T: Neg<Output = T>> vec2<T> {
    /// Rotate a vector by 90 degrees counter clockwise.
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(3.0, 4.0);
    /// assert_eq!(v.rotate_90(), vec2(-4.0, 3.0));
    /// ```
    pub fn rotate_90(self) -> Self {
        let vec2(x, y) = self;
        vec2(-y, x)
    }
}

impl<T: Float> vec2<T> {
    /// Normalize a vector.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v: vec2<f64> = vec2(1.0, 2.0);
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
    /// # use batbox_la::*;
    /// let v = vec2(1.0, 2.0);
    /// assert_eq!(v.normalize_or_zero(), v.normalize());
    /// let v = vec2(1e-10, 1e-10);
    /// assert_eq!(v.normalize_or_zero(), vec2::ZERO);
    /// ```
    pub fn normalize_or_zero(self) -> Self {
        let len = self.len();
        if len.approx_eq(&T::ZERO) {
            vec2::ZERO
        } else {
            self / len
        }
    }

    /// Calculate length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(3.0, 4.0);
    /// assert_eq!(v.len(), 5.0);
    /// ```
    pub fn len(self) -> T {
        T::sqrt(self.len_sqr())
    }

    /// Calculate squared length of this vector
    pub fn len_sqr(self) -> T {
        vec2::dot(self, self)
    }

    /// Rotate a vector by a given angle.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(1.0, 2.0);
    /// assert!((v.rotate(std::f32::consts::FRAC_PI_2) - vec2(-2.0, 1.0)).len() < 1e-5);
    /// ```
    pub fn rotate(self, angle: T) -> Self {
        let (sin, cos) = T::sin_cos(angle);
        Self(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// Clamp vector's length. Note that the range must be inclusive.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(1.0, 2.0);
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
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(1.0, 2.0);
    /// assert_eq!(v.clamp_coordinates(.., 0.0..=1.0), vec2(1.0, 1.0));
    /// ```
    pub fn clamp_coordinates(
        self,
        x_range: impl FixedRangeBounds<T>,
        y_range: impl FixedRangeBounds<T>,
    ) -> Self {
        vec2(self.x.clamp_range(x_range), self.y.clamp_range(y_range))
    }

    /// Clamp vector by `aabb` corners.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(0.5, 2.0);
    /// let min = vec2(0.0, 0.0);
    /// let max = vec2(1.0, 1.0);
    /// let aabb = Aabb2::from_corners(min, max);
    /// assert_eq!(v.clamp_aabb(aabb), vec2(0.5, 1.0));
    /// ```
    pub fn clamp_aabb(self, aabb: Aabb2<T>) -> Self {
        let start = aabb.bottom_left();
        let end = aabb.top_right();
        self.clamp_coordinates(start.x..=end.x, start.y..=end.y)
    }

    /// Get an angle between the positive direction of the x-axis.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let v = vec2(0.0, 1.0);
    /// assert_eq!(v.arg(), std::f32::consts::FRAC_PI_2);
    /// ```
    pub fn arg(self) -> T {
        T::atan2(self.y, self.x)
    }

    /// Apply transformation matrix
    pub fn transform(self, transform: mat3<T>) -> Self {
        (transform * self.extend(T::ONE)).into_2d()
    }

    /// Calculate aspect ratio (x / y)
    pub fn aspect(self) -> T {
        self.x / self.y
    }
}

#[test]
fn test_clamp_zero_len() {
    vec2::ZERO.clamp_len(..=R64::ONE);
}
