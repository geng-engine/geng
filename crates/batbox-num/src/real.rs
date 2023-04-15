use super::*;

/// Real number (differs from [Float] as doesn't support NaN)
pub trait Real: Num + Copy {
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

impl<T: Real> Float for T {
    const PI: Self = <Self as Real>::PI;
    fn acos(self) -> Self {
        Real::acos(self)
    }
    fn asin(self) -> Self {
        Real::asin(self)
    }
    fn atan(self) -> Self {
        Real::atan(self)
    }
    fn atan2(y: Self, x: Self) -> Self {
        Real::atan2(y, x)
    }
    fn ceil(self) -> Self {
        Real::ceil(self)
    }
    fn cos(self) -> Self {
        Real::cos(self)
    }
    fn div_euclid(self, other: Self) -> Self {
        Real::div_euclid(self, other)
    }
    fn exp(self) -> Self {
        Real::exp(self)
    }
    fn floor(self) -> Self {
        Real::floor(self)
    }
    fn fract(self) -> Self {
        Real::fract(self)
    }
    fn ln(self) -> Self {
        Real::ln(self)
    }
    fn log(self, base: Self) -> Self {
        Real::log(self, base)
    }
    fn log10(self) -> Self {
        Real::log10(self)
    }
    fn log2(self) -> Self {
        Real::log2(self)
    }
    fn powf(self, n: Self) -> Self {
        Real::powf(self, n)
    }
    fn powi(self, n: i32) -> Self {
        Real::powi(self, n)
    }
    fn recip(self) -> Self {
        Real::recip(self)
    }
    fn rem_euclid(self, other: Self) -> Self {
        Real::rem_euclid(self, other)
    }
    fn round(self) -> Self {
        Real::round(self)
    }
    fn signum(self) -> Self {
        Real::signum(self)
    }
    fn sin(self) -> Self {
        Real::sin(self)
    }
    fn sin_cos(self) -> (Self, Self) {
        Real::sin_cos(self)
    }
    fn sqrt(self) -> Self {
        Real::sqrt(self)
    }
    fn tan(self) -> Self {
        Real::tan(self)
    }
    fn from_f32(x: f32) -> Self {
        Real::from_f32(x)
    }
    fn as_f32(self) -> f32 {
        Real::as_f32(self)
    }
    fn is_finite(self) -> bool {
        true
    }
}

/// Wrapper around T that checks for valid values (panics on NaN/Inf)
#[derive(Copy, Clone, PartialEq, serde::Serialize)]
#[repr(transparent)]
pub struct RealImpl<T: Float>(T);

macro_rules! impl_for {
    ($t:ty) => {
        impl<'de> serde::Deserialize<'de> for RealImpl<$t> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <$t as serde::Deserialize>::deserialize(deserializer)?
                    .try_into()
                    .map_err(serde::de::Error::custom)
            }
        }
        impl TryFrom<$t> for RealImpl<$t> {
            type Error = &'static str;
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if value.is_finite() {
                    Ok(Self::new_unchecked(value))
                } else {
                    Err("Value must be finite")
                }
            }
        }
    };
}

impl_for!(f32);
impl_for!(f64);

impl<T: Float> std::fmt::Debug for RealImpl<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        <T as std::fmt::Debug>::fmt(&self.0, fmt)
    }
}

impl<T: Float> std::fmt::Display for RealImpl<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        <T as std::fmt::Display>::fmt(&self.0, fmt)
    }
}

impl<T: Float> RealImpl<T> {
    /// Create a new value, panics if not finite
    pub fn new(value: T) -> Self {
        assert!(value.is_finite());
        Self(value)
    }

    /// Create a new value without checking
    pub fn new_unchecked(value: T) -> Self {
        Self(value)
    }

    /// Get raw value
    pub fn raw(self) -> T {
        self.0
    }
}

impl<T: Float> Eq for RealImpl<T> {}

impl<T: Float> PartialOrd for RealImpl<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Float> Ord for RealImpl<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

macro_rules! impl_op {
    ($($op:ident: $fn:ident,)*) => {
        $(
            impl<T: Float> std::ops::$op for RealImpl<T> {
                type Output = Self;
                fn $fn(self, rhs: Self) -> Self {
                    Self::new(self.0 .$fn(rhs.0))
                }
            }
        )*
    };
}
macro_rules! impl_op_assign {
    ($($op:ident: $fn:ident,)*) => {
        $(
            impl<T: Float> std::ops::$op for RealImpl<T> {
                fn $fn(&mut self, rhs: Self) {
                    self.0 .$fn(rhs.0);
                }
            }
        )*
    };
}

impl_op! {
    Add: add,
    Sub: sub,
    Mul: mul,
    Div: div,
}

impl_op_assign! {
    AddAssign: add_assign,
    SubAssign: sub_assign,
    MulAssign: mul_assign,
    DivAssign: div_assign,
}

impl<T: Float> std::ops::Neg for RealImpl<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

impl<T: Float> UNum for RealImpl<T> {
    const ZERO: Self = Self(T::ZERO);
    const ONE: Self = Self(T::ONE);
}

/// Uniform [RealImpl] sampler
pub struct UniformReal<T: rand::distributions::uniform::SampleUniform>(T::Sampler);

impl<T: Float + rand::distributions::uniform::SampleUniform>
    rand::distributions::uniform::UniformSampler for UniformReal<T>
{
    type X = RealImpl<T>;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        Self(T::Sampler::new(low.borrow().0, high.borrow().0))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        Self(T::Sampler::new_inclusive(low.borrow().0, high.borrow().0))
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        RealImpl(self.0.sample(rng))
    }

    fn sample_single<R: rand::Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> Self::X
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        RealImpl(T::Sampler::sample_single(
            low.borrow().0,
            high.borrow().0,
            rng,
        ))
    }

    fn sample_single_inclusive<R: rand::Rng + ?Sized, B1, B2>(
        low: B1,
        high: B2,
        rng: &mut R,
    ) -> Self::X
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        RealImpl(T::Sampler::sample_single_inclusive(
            low.borrow().0,
            high.borrow().0,
            rng,
        ))
    }
}

impl<T: Float + rand::distributions::uniform::SampleUniform>
    rand::distributions::uniform::SampleUniform for RealImpl<T>
{
    type Sampler = UniformReal<T>;
}

impl<T: Float> Real for RealImpl<T> {
    const PI: Self = Self(T::PI);
    fn acos(self) -> Self {
        Self::new(T::acos(self.0))
    }
    fn asin(self) -> Self {
        Self::new(T::asin(self.0))
    }
    fn atan(self) -> Self {
        Self::new(T::atan(self.0))
    }
    fn atan2(y: Self, x: Self) -> Self {
        Self::new(T::atan2(y.0, x.0))
    }
    fn ceil(self) -> Self {
        Self::new(T::ceil(self.0))
    }
    fn cos(self) -> Self {
        Self::new(T::cos(self.0))
    }
    fn div_euclid(self, other: Self) -> Self {
        Self::new(T::div_euclid(self.0, other.0))
    }
    fn exp(self) -> Self {
        Self::new(T::exp(self.0))
    }
    fn floor(self) -> Self {
        Self::new(T::floor(self.0))
    }
    fn fract(self) -> Self {
        Self::new(T::fract(self.0))
    }
    fn ln(self) -> Self {
        Self::new(T::ln(self.0))
    }
    fn log(self, base: Self) -> Self {
        Self::new(T::log(self.0, base.0))
    }
    fn log10(self) -> Self {
        Self::new(T::log10(self.0))
    }
    fn log2(self) -> Self {
        Self::new(T::log2(self.0))
    }
    fn powf(self, n: Self) -> Self {
        Self::new(T::powf(self.0, n.0))
    }
    fn powi(self, n: i32) -> Self {
        Self::new(T::powi(self.0, n))
    }
    fn recip(self) -> Self {
        Self::new(T::recip(self.0))
    }
    fn rem_euclid(self, other: Self) -> Self {
        Self::new(T::rem_euclid(self.0, other.0))
    }
    fn round(self) -> Self {
        Self::new(T::round(self.0))
    }
    fn signum(self) -> Self {
        Self::new(T::signum(self.0))
    }
    fn sin(self) -> Self {
        Self::new(T::sin(self.0))
    }
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = T::sin_cos(self.0);
        (Self::new(sin), Self::new(cos))
    }
    fn sqrt(self) -> Self {
        Self::new(T::sqrt(self.0))
    }
    fn tan(self) -> Self {
        Self::new(T::tan(self.0))
    }
    fn from_f32(x: f32) -> Self {
        Self::new(T::from_f32(x))
    }
    fn as_f32(self) -> f32 {
        self.0.as_f32()
    }
}

// TODO: basically duplicates the trait wtf
impl<T: Float> RealImpl<T> {
    /// Archimedes’ constant (π)
    pub const PI: Self = <Self as Real>::PI;

    /// Computes the arccosine of a number.
    ///
    /// Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
    pub fn acos(self) -> Self {
        <Self as Real>::acos(self)
    }

    /// Computes the arcsine of a number.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    pub fn asin(self) -> Self {
        <Self as Real>::asin(self)
    }

    /// Computes the arctangent of a number.
    ///
    /// Return value is in radians in the range [-pi/2, pi/2];
    pub fn atan(self) -> Self {
        <Self as Real>::atan(self)
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.  
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    pub fn atan2(y: Self, x: Self) -> Self {
        <Self as Real>::atan2(y, x)
    }

    /// Returns the smallest integer greater than or equal to a number.
    pub fn ceil(self) -> Self {
        <Self as Real>::ceil(self)
    }

    /// Computes the cosine of a number (in radians).
    pub fn cos(self) -> Self {
        <Self as Real>::cos(self)
    }

    /// Calculates Euclidean division, the matching method for rem_euclid.
    ///
    /// This computes the integer n such that self = n * rhs + self.rem_euclid(rhs).
    /// In other words, the result is self / rhs rounded to the integer n such that self >= n * rhs.
    pub fn div_euclid(self, other: Self) -> Self {
        <Self as Real>::div_euclid(self, other)
    }

    /// Returns `e^(self)`, (the exponential function).
    pub fn exp(self) -> Self {
        <Self as Real>::exp(self)
    }

    /// Returns the largest integer less than or equal to `self`.
    pub fn floor(self) -> Self {
        <Self as Real>::floor(self)
    }

    /// Returns the fractional part of `self`.
    pub fn fract(self) -> Self {
        <Self as Real>::fract(self)
    }

    /// Returns the natural logarithm of the number.
    pub fn ln(self) -> Self {
        <Self as Real>::ln(self)
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    pub fn log(self, base: Self) -> Self {
        <Self as Real>::log(self, base)
    }

    /// Returns the base 10 logarithm of the number.
    pub fn log10(self) -> Self {
        <Self as Real>::log10(self)
    }

    /// Returns the base 2 logarithm of the number.
    pub fn log2(self) -> Self {
        <Self as Real>::log2(self)
    }

    /// Raises a number to a floating point power.
    pub fn powf(self, n: Self) -> Self {
        <Self as Real>::powf(self, n)
    }

    /// Raises a number to an integer power.
    pub fn powi(self, n: i32) -> Self {
        <Self as Real>::powi(self, n)
    }

    /// Takes the reciprocal (inverse) of a number, 1/x
    pub fn recip(self) -> Self {
        <Self as Real>::recip(self)
    }

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
    pub fn rem_euclid(self, other: Self) -> Self {
        <Self as Real>::rem_euclid(self, other)
    }

    /// Returns the nearest integer to `self`.
    /// Round half-way cases away from `0.0`.
    pub fn round(self) -> Self {
        <Self as Real>::round(self)
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    pub fn signum(self) -> Self {
        <Self as Real>::signum(self)
    }

    /// Computes the sine of a number (in radians).
    pub fn sin(self) -> Self {
        <Self as Real>::sin(self)
    }

    /// Simultaneously computes the sine and cosine of the number, `x`.
    /// Returns `(sin(x), cos(x))`.
    pub fn sin_cos(self) -> (Self, Self) {
        <Self as Real>::sin_cos(self)
    }

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    pub fn sqrt(self) -> Self {
        <Self as Real>::sqrt(self)
    }

    /// Computes the tangent of a number (in radians).
    pub fn tan(self) -> Self {
        <Self as Real>::tan(self)
    }

    /// Convert an [f32] into [Self]
    pub fn from_f32(x: f32) -> Self {
        <Self as Real>::from_f32(x)
    }

    /// Convert self into an [f32]
    pub fn as_f32(self) -> f32 {
        <Self as Real>::as_f32(self)
    }
}

/// Like [f32] but panics on NaN/Inf
pub type R32 = RealImpl<f32>;

/// Construct a new [R32] from an [f32]
pub fn r32(value: f32) -> R32 {
    R32::new(value)
}

/// Like [f64] but panics on NaN/Inf
pub type R64 = RealImpl<f64>;

/// Construct a new [R64] from an [f64]
pub fn r64(value: f64) -> R64 {
    R64::new(value)
}

#[test]
fn test_reals() {
    let a = r64(3.0);
    let b = r64(2.0);
    println!("a = {a:?}, b = {b:?}");
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("a / b = {:?}", a / b);
    println!("sin_cos(a) = {:?}", a.sin_cos());

    let mut arr = [r32(1.0), r32(0.0)];
    arr.sort();

    let _random = rand::Rng::gen_range(&mut rand::thread_rng(), r32(0.0)..r32(1.0));
}

#[test]
#[should_panic]
fn test_reals_fail() {
    println!("0 / 0 = {:?}", R64::ZERO / R64::ZERO);
}
