use super::*;

/// This struct represents an angle in 2d space,
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Angle<T: Float = f32> {
    radians: T,
}

impl<T: Float> Angle<T> {
    /// 0 angle is pointing to positive x axis
    pub const ZERO: Self = Self { radians: T::ZERO };

    /// Computes the arccosine of a number as an angle.
    ///
    /// Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
    pub fn acos(cos: T) -> Self {
        Self {
            radians: cos.acos(),
        }
    }

    /// Computes the arcsine of a number as an angle.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    pub fn asin(sin: T) -> Self {
        Self {
            radians: sin.asin(),
        }
    }

    /// Computes the arctangent of a number as an angle.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2];
    pub fn atan(tan: T) -> Self {
        Self {
            radians: tan.atan(),
        }
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) as an angle.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    pub fn atan2(y: T, x: T) -> Self {
        Self {
            radians: T::atan2(y, x),
        }
    }

    /// Compute the sine
    pub fn sin(&self) -> T {
        self.radians.sin()
    }

    /// Compute the cosine
    pub fn cos(&self) -> T {
        self.radians.cos()
    }

    /// Simultaneously computes the sine and cosine of the angle.
    /// Returns `(sin(self), cos(self))`.
    pub fn sin_cos(&self) -> (T, T) {
        self.radians.sin_cos()
    }

    /// Computes the tangent of the angle.
    pub fn tan(self) -> T {
        self.radians.tan()
    }

    /// Create angle from value in radians
    pub fn from_radians(radians: T) -> Self {
        Self { radians }
    }

    /// Create angle from value in degrees
    pub fn from_degrees(degrees: T) -> Self {
        Self {
            radians: degrees_to_radians(degrees),
        }
    }

    /// See angle value as radians
    pub fn as_radians(&self) -> T {
        self.radians
    }

    /// See angle value as degrees
    pub fn as_degrees(&self) -> T {
        radians_to_degrees(self.radians)
    }

    /// Normalize the angle to be in range `0..2*pi`.
    pub fn normalized_2pi(&self) -> Self {
        let tau = T::PI + T::PI;
        let mut norm = (self.radians / tau).fract();
        if norm < T::ZERO {
            norm += T::ONE;
        }
        Self {
            radians: norm * tau,
        }
    }

    /// Calculate absolute value
    pub fn abs(&self) -> Self {
        Self {
            radians: self.radians.abs(),
        }
    }

    /// Normalize the angle to be in range `-pi..pi`.
    pub fn normalized_pi(&self) -> Self {
        let pi = T::PI;
        let mut angle = self.normalized_2pi().radians;
        if angle > pi {
            angle -= pi + pi;
        }
        Self { radians: angle }
    }

    /// Calculates the angle between `from` and `self` in range `-pi..pi`.
    pub fn angle_from(&self, from: Self) -> Self {
        from.angle_to(*self)
    }

    /// Calculates the angle between `self` and `target` in range `-pi..pi`.
    pub fn angle_to(&self, target: Self) -> Self {
        let pi = T::PI;
        let mut delta = target.normalized_2pi().radians - self.normalized_2pi().radians;
        if delta.abs() > pi {
            delta -= (pi + pi) * delta.signum();
        }
        Self { radians: delta }
    }

    /// Returns a direction vector of unit length.
    pub fn unit_vec(&self) -> vec2<T> {
        let (sin, cos) = self.radians.sin_cos();
        vec2(cos, sin)
    }
}

fn degrees_to_radians<T: Float>(degrees: T) -> T {
    degrees / T::from_f32(180.0) * T::PI
}

fn radians_to_degrees<T: Float>(radians: T) -> T {
    radians / T::PI * T::from_f32(180.0)
}

impl<T: Float> Mul<T> for Angle<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            radians: self.radians * rhs,
        }
    }
}

impl<T: Float> Div<T> for Angle<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            radians: self.radians / rhs,
        }
    }
}

impl<T: Float> Add for Angle<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians + rhs.radians,
        }
    }
}

impl<T: Float> AddAssign for Angle<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl<T: Float> Sub for Angle<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians - rhs.radians,
        }
    }
}

impl<T: Float> SubAssign for Angle<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl<T: Float> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            radians: -self.radians,
        }
    }
}

#[test]
fn test_angle_conversion() {
    const EPSILON: f32 = 1e-3;
    let tests = [
        (0.0, 0.0),
        (90.0, f32::PI / 2.0),
        (180.0, f32::PI),
        (270.0, f32::PI * 3.0 / 2.0),
        (360.0, f32::PI * 2.0),
    ];
    for (degrees, radians) in tests {
        let d = Angle::from_degrees(degrees).as_radians();
        let r = Angle::from_radians(radians).as_radians();
        let delta = r - d;
        assert!(
            delta.abs() < EPSILON,
            "{degrees} degrees expected to be converted to {radians} radians, found {d}"
        )
    }
}

#[test]
fn test_angle_normalize_2pi() {
    const EPSILON: f32 = 1e-3;
    let tests = [0.0, f32::PI, f32::PI / 2.0, f32::PI * 3.0 / 2.0];
    for test in tests {
        for offset in [0, 1, -1, 2, -2] {
            let angle = test + f32::PI * 2.0 * offset as f32;
            let norm = Angle::from_radians(angle).normalized_2pi().as_radians();
            let delta = test - norm;
            assert!(
                delta.abs() < EPSILON,
                "Normalized {angle} expected to be {test}, found {norm}"
            );
        }
    }
}

#[test]
fn test_angle_delta() {
    const EPSILON: f32 = 1e-3;
    let tests = [
        (0.0, f32::PI / 2.0, f32::PI / 2.0),
        (0.0, f32::PI * 3.0 / 2.0, -f32::PI / 2.0),
    ];
    for (from, to, test) in tests {
        for offset_from in [0, 1, -1, 2, -2] {
            for offset_to in [0, 1, -1, 2, -2] {
                for offset in [0.0, 1.0, -1.0, 2.0, -2.0] {
                    let from = from + f32::PI * 2.0 * offset_from as f32 + offset;
                    let to = to + f32::PI * 2.0 * offset_to as f32 + offset;
                    let angle = Angle::from_radians(from)
                        .angle_to(Angle::from_radians(to))
                        .as_radians();
                    let delta = test - angle;
                    assert!(
                        delta.abs() < EPSILON,
                        "Angle from {from} to {to} expected to be {test}, found {angle}"
                    );
                }
            }
        }
    }
}
