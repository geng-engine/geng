use super::*;

/// 4 dimensional vector.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct vec4<T>(pub T, pub T, pub T, pub T);

impl<T: std::fmt::Display> std::fmt::Display for vec4<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T> From<[T; 4]> for vec4<T> {
    fn from(arr: [T; 4]) -> vec4<T> {
        let [x, y, z, w] = arr;
        vec4(x, y, z, w)
    }
}

/// Data structure used to provide access to coordinates with the dot notation, e.g. `v.x`
#[repr(C)]
pub struct XYZW<T> {
    #[allow(missing_docs)]
    pub x: T,
    #[allow(missing_docs)]
    pub y: T,
    #[allow(missing_docs)]
    pub z: T,
    #[allow(missing_docs)]
    pub w: T,
}

impl<T> Deref for XYZW<T> {
    type Target = [T; 4];
    fn deref(&self) -> &[T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for XYZW<T> {
    fn deref_mut(&mut self) -> &mut [T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Deref for vec4<T> {
    type Target = XYZW<T>;
    fn deref(&self) -> &XYZW<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for vec4<T> {
    fn deref_mut(&mut self) -> &mut XYZW<T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> vec4<T> {
    /// Get first two components as a [vec2]
    pub fn xy(self) -> vec2<T> {
        vec2(self.0, self.1)
    }

    /// Get first three components as a [vec3]
    pub fn xyz(self) -> vec3<T> {
        vec3(self.0, self.1, self.2)
    }

    /// Map every value (coordinate).
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> vec4<U> {
        vec4(f(self.0), f(self.1), f(self.2), f(self.3))
    }
}

impl<T: Clone> vec4<T> {
    /// Construct a vector with all components set to specified value
    pub fn splat(value: T) -> Self {
        Self(value.clone(), value.clone(), value.clone(), value)
    }
}

impl<T: UNum> vec4<T> {
    /// A zero 4-d vector
    pub const ZERO: Self = vec4(T::ZERO, T::ZERO, T::ZERO, T::ZERO);

    /// A unit X
    pub const UNIT_X: Self = Self(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// A unit Y
    pub const UNIT_Y: Self = Self(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// A unit Z
    pub const UNIT_Z: Self = Self(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// A unit W
    pub const UNIT_W: Self = Self(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: Copy + Num> vec4<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(vec4::dot(vec4(1, 2, 3, 4), vec4(3, 4, 5, 6)), 50);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

impl<T: Float> vec4<T> {
    /// Convert a homogenous 4d vector into 3d
    ///
    /// Same as self.xyz() / self.w
    pub fn into_3d(self) -> vec3<T> {
        self.xyz() / self.w
    }
}
