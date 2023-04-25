use super::*;

/// Floating point number, including NaN/Inf
pub trait Float: Num {
    /// Archimedes’ constant (π)
    const PI: Self;

    /// Computes the arccosine of a number.
    ///
    /// Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
    fn acos(self) -> Self;

    /// Computes the arcsine of a number.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    fn asin(self) -> Self;

    /// Computes the arctangent of a number.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2];
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.  
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(y: Self, x: Self) -> Self;

    /// Returns the smallest integer greater than or equal to a number.
    fn ceil(self) -> Self;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self;

    /// Calculates Euclidean division, the matching method for rem_euclid.
    ///
    /// This computes the integer n such that self = n * rhs + self.rem_euclid(rhs).
    /// In other words, the result is self / rhs rounded to the integer n such that self >= n * rhs.
    fn div_euclid(self, other: Self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self;

    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> Self;

    /// Returns the fractional part of `self`.
    fn fract(self) -> Self;

    /// Returns `true` if this number is neither infinite nor NaN.
    fn is_finite(self) -> bool;

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Self) -> Self;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    /// Raises a number to a floating point power.
    fn powf(self, n: Self) -> Self;

    /// Raises a number to an integer power.
    fn powi(self, n: i32) -> Self;

    /// Takes the reciprocal (inverse) of a number, 1/x
    fn recip(self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == rhs.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `rhs.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)`
    /// approximately.
    fn rem_euclid(self, other: Self) -> Self;

    /// Returns the nearest integer to `self`.
    /// Round half-way cases away from `0.0`.
    fn round(self) -> Self;

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    fn signum(self) -> Self;

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`.
    /// Returns `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self);

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    fn sqrt(self) -> Self;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self;

    /// Convert an [f32] into [Self]
    fn from_f32(x: f32) -> Self;

    /// Convert self into an [f32]
    fn as_f32(self) -> f32;
}

macro_rules! impl_float {
    ($t:ident) => {
        impl UNum for $t {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        }
        impl Float for $t {
            const PI: Self = std::$t::consts::PI;
            fn acos(self) -> Self {
                $t::acos(self)
            }
            fn asin(self) -> Self {
                $t::asin(self)
            }
            fn atan(self) -> Self {
                $t::atan(self)
            }
            fn atan2(y: Self, x: Self) -> Self {
                $t::atan2(y, x)
            }
            fn ceil(self) -> Self {
                $t::ceil(self)
            }
            fn cos(self) -> Self {
                $t::cos(self)
            }
            fn div_euclid(self, other: Self) -> Self {
                $t::div_euclid(self, other)
            }
            fn exp(self) -> Self {
                $t::exp(self)
            }
            fn floor(self) -> Self {
                $t::floor(self)
            }
            fn fract(self) -> Self {
                $t::fract(self)
            }
            fn is_finite(self) -> bool {
                $t::is_finite(self)
            }
            fn ln(self) -> Self {
                $t::ln(self)
            }
            fn log(self, base: Self) -> Self {
                $t::log(self, base)
            }
            fn log10(self) -> Self {
                $t::log10(self)
            }
            fn log2(self) -> Self {
                $t::log2(self)
            }
            fn powf(self, n: Self) -> Self {
                $t::powf(self, n)
            }
            fn powi(self, n: i32) -> Self {
                $t::powi(self, n)
            }
            fn recip(self) -> Self {
                $t::recip(self)
            }
            fn rem_euclid(self, other: Self) -> Self {
                $t::rem_euclid(self, other)
            }
            fn round(self) -> Self {
                $t::round(self)
            }
            fn signum(self) -> Self {
                $t::signum(self)
            }
            fn sin(self) -> Self {
                $t::sin(self)
            }
            fn sin_cos(self) -> (Self, Self) {
                $t::sin_cos(self)
            }
            fn sqrt(self) -> Self {
                $t::sqrt(self)
            }
            fn tan(self) -> Self {
                $t::tan(self)
            }
            fn from_f32(x: f32) -> Self {
                x as Self
            }
            fn as_f32(self) -> f32 {
                self as f32
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
