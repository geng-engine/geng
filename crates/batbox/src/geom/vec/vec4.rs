use super::*;

/// 4 dimensional vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec4<T> {
    /// `x` coordinate of the vector
    pub x: T,
    /// `y` coordinate of the vector
    pub y: T,
    /// `z` coordinate of the vector
    pub z: T,
    /// `w` coordinate of the vector
    pub w: T,
}

impl<T: Display> Display for Vec4<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

/// Construct a 4-d vector with given components.
///
/// # Example
/// ```
/// use batbox::prelude::*;
/// let v = vec4(1, 2, 3, 4);
/// ```
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Vec4<T> {
        let [x, y, z, w] = arr;
        vec4(x, y, z, w)
    }
}

impl<T> Deref for Vec4<T> {
    type Target = [T; 4];
    fn deref(&self) -> &[T; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec4<T> {
    fn deref_mut(&mut self) -> &mut [T; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> Vec4<T> {
    /// Construct a new [Vec4]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Get first two components as a [Vec2]
    pub fn xy(self) -> Vec2<T> {
        vec2(self.x, self.y)
    }

    /// Get first three components as a [Vec3]
    pub fn xyz(self) -> Vec3<T> {
        vec3(self.x, self.y, self.z)
    }

    /// Map every value (coordinate).
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec4<U> {
        vec4(f(self.x), f(self.y), f(self.z), f(self.w))
    }
}

impl<T: UNum> Vec4<T> {
    /// A zero 4-d vector
    pub const ZERO: Self = vec4(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Copy + Num> Vec4<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// assert_eq!(Vec4::dot(vec4(1, 2, 3, 4), vec4(3, 4, 5, 6)), 50);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

impl<T: Float> Vec4<T> {
    /// Convert a homogenous 4d vector into 3d
    ///
    /// Same as self.xyz() / self.w
    pub fn into_3d(self) -> Vec3<T> {
        self.xyz() / self.w
    }
}
