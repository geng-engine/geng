use super::*;

/// A [quaternion](https://en.wikipedia.org/wiki/Quaternion)
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Quat<T> {
    /// First imaginary component
    pub i: T,
    /// Second imaginary component
    pub j: T,
    /// Third imaginary component
    pub k: T,
    /// Real component
    pub w: T,
}

impl<T> Quat<T> {
    /// Map every component
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Quat<U> {
        Quat {
            i: f(self.i),
            j: f(self.j),
            k: f(self.k),
            w: f(self.w),
        }
    }
}

impl<T: Float> Quat<T> {
    /// Identity element - 1
    pub fn identity() -> Self {
        Self {
            i: T::ZERO,
            j: T::ZERO,
            k: T::ZERO,
            w: T::ONE,
        }
    }

    /// Construct a quaternion representing rotation around given axis by given angle
    pub fn from_axis_angle(axis: vec3<T>, angle: T) -> Self {
        let angle = angle / (T::ONE + T::ONE);
        let sin = angle.sin();
        let cos = angle.cos();
        let v = axis * sin;
        Self {
            i: v.x,
            j: v.y,
            k: v.z,
            w: cos,
        }
    }

    /// Calculate length of this quaternion
    pub fn len(self) -> T {
        self.len_sqr().sqrt()
    }

    /// Calculate squared length of this quaternion
    pub fn len_sqr(self) -> T {
        self.i * self.i + self.j * self.j + self.k * self.k + self.w * self.w
    }

    /// Normalize this quaternion
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Lerp - calculate `v0 * (1 - t) + v1 * t`
    pub fn lerp(v0: Self, v1: Self, t: T) -> Self {
        v0 * (T::ONE - t) + v1 * t
    }
}

impl<T: Float> Mul for Quat<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            i: self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            j: self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
            k: self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w,
            w: self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
        }
    }
}

impl<T: Float> MulAssign for Quat<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num> Add for Quat<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
            w: self.w + rhs.w,
        }
    }
}

impl<T: Float> Mul<T> for Quat<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T: Float> MulAssign<T> for Quat<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Float> Div<T> for Quat<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            i: self.i / rhs,
            j: self.j / rhs,
            k: self.k / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T: Float> DivAssign<T> for Quat<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Quat<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            i: -self.i,
            j: -self.j,
            k: -self.k,
            w: -self.w,
        }
    }
}

impl<T: Float> From<Quat<T>> for mat4<T> {
    fn from(quat: Quat<T>) -> Self {
        let i = quat.i;
        let j = quat.j;
        let k = quat.k;
        let w = quat.w;

        let two = T::ONE + T::ONE;

        let ww = w * w;
        let ii = i * i;
        let jj = j * j;
        let kk = k * k;
        let ij = i * j * two;
        let wk = w * k * two;
        let wj = w * j * two;
        let ik = i * k * two;
        let jk = j * k * two;
        let wi = w * i * two;

        Self::new([
            [ww + ii - jj - kk, ij - wk, wj + ik, T::ZERO],
            [wk + ij, ww - ii + jj - kk, jk - wi, T::ZERO],
            [ik - wj, wi + jk, ww - ii - jj + kk, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

#[test]
fn test_quat() {
    let mat = mat4::from(Quat::from_axis_angle(
        vec3(0.0, 1.0, 0.0),
        std::f64::consts::PI / 2.0,
    ));
    let v = mat * vec4(1.0, 0.0, 0.0, 1.0);
    assert!(v.x.approx_eq(&0.0));
    assert!(v.y.approx_eq(&0.0));
    assert!(v.z.approx_eq(&-1.0));
    assert!(v.w.approx_eq(&1.0));
}
