use crate::*;

/// 3-d vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Trans, Schematic)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
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
/// use batbox::*;
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
    pub fn extend(self, w: T) -> Vec4<T> {
        vec4(self.x, self.y, self.z, w)
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec3<U> {
        vec3(f(self.x), f(self.y), f(self.z))
    }
}

impl<T: Copy + Num> Vec3<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(Vec3::dot(vec3(1, 2, 3), vec3(3, 4, 5)), 26);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// Calculate cross product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
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
    /// use batbox::*;
    /// let v: Vec3<f64> = vec3(1.0, 2.0, 3.0);
    /// assert!((v.normalize().len() - 1.0).abs() < 1e-5);
    /// ```
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Calculate length of a vector.
    pub fn len(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn clamp(self, max_len: T) -> Self {
        let len = self.len();
        if len > max_len {
            self * max_len / len
        } else {
            self
        }
    }
}
