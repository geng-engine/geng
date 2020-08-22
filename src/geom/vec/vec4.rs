use super::*;

/// 4-d vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Trans)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
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
/// use batbox::*;
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
    pub fn xy(self) -> Vec2<T> {
        vec2(self.x, self.y)
    }
    pub fn xyz(self) -> Vec3<T> {
        vec3(self.x, self.y, self.z)
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec4<U> {
        vec4(f(self.x), f(self.y), f(self.z), f(self.w))
    }
}

impl<T: Copy + Num> Vec4<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(Vec4::dot(vec4(1, 2, 3, 4), vec4(3, 4, 5, 6)), 50);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}
