use super::*;

pub trait Real: Num + Copy {
    const PI: Self;
    fn signum(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }
    fn atan2(y: Self, x: Self) -> Self;
    fn as_f32(self) -> f32;
}

impl<T: Real> Float for T {
    const PI: Self = <Self as Real>::PI;
    fn signum(self) -> Self {
        Real::signum(self)
    }
    fn floor(self) -> Self {
        Real::floor(self)
    }
    fn ceil(self) -> Self {
        Real::ceil(self)
    }
    fn sqrt(self) -> Self {
        Real::sqrt(self)
    }
    fn tan(self) -> Self {
        Real::tan(self)
    }
    fn sin(self) -> Self {
        Real::sin(self)
    }
    fn cos(self) -> Self {
        Real::cos(self)
    }
    fn sin_cos(self) -> (Self, Self) {
        Real::sin_cos(self)
    }
    fn atan2(y: Self, x: Self) -> Self {
        Real::atan2(y, x)
    }
    fn is_finite(self) -> bool {
        true
    }
    fn as_f32(self) -> f32 {
        Real::as_f32(self)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Schematic, Serialize)]
#[repr(transparent)]
pub struct RealImpl<T: Float>(T);

impl<T: Float + Trans> Trans for RealImpl<T> {
    fn read_from(reader: &mut dyn std::io::Read) -> Result<Self, std::io::Error> {
        let value = T::read_from(reader)?;
        if value.is_finite() {
            Ok(Self::new_unchecked(value))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Value must be finite",
            ))
        }
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        self.0.write_to(writer)
    }
}

macro_rules! impl_for {
    ($t:ty) => {
        impl<'de> Deserialize<'de> for RealImpl<$t> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <$t as Deserialize>::deserialize(deserializer)?
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

impl<T: Float> Debug for RealImpl<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        <T as std::fmt::Debug>::fmt(&self.0, fmt)
    }
}

impl<T: Float> Display for RealImpl<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        <T as std::fmt::Display>::fmt(&self.0, fmt)
    }
}

impl<T: Float> RealImpl<T> {
    pub fn new(value: T) -> Self {
        assert!(value.is_finite());
        Self { 0: value }
    }
    pub fn new_unchecked(value: T) -> Self {
        Self { 0: value }
    }
    pub fn raw(self) -> T {
        self.0
    }
}

impl<T: Float> Eq for RealImpl<T> {}

impl<T: Float> Ord for RealImpl<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

macro_rules! impl_op {
    ($($op:ident: $fn:ident,)*) => {
        $(
            impl<T: Float> $op for RealImpl<T> {
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
            impl<T: Float> $op for RealImpl<T> {
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

impl<T: Float> Neg for RealImpl<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

impl<T: Float> UNum for RealImpl<T> {
    const ZERO: Self = Self { 0: T::ZERO };
    const ONE: Self = Self { 0: T::ONE };
}

impl<T: Float> RealImpl<T> {
    pub const PI: Self = <Self as Real>::PI;
    pub fn signum(self) -> Self {
        Real::signum(self)
    }
    pub fn floor(self) -> Self {
        Real::floor(self)
    }
    pub fn ceil(self) -> Self {
        Real::ceil(self)
    }
    pub fn sqrt(self) -> Self {
        Real::sqrt(self)
    }
    pub fn tan(self) -> Self {
        Real::tan(self)
    }
    pub fn sin(self) -> Self {
        Real::sin(self)
    }
    pub fn cos(self) -> Self {
        Real::cos(self)
    }
    pub fn sin_cos(self) -> (Self, Self) {
        Real::sin_cos(self)
    }
    pub fn atan2(y: Self, x: Self) -> Self {
        Real::atan2(y, x)
    }
    pub fn as_f32(self) -> f32 {
        Real::as_f32(self)
    }
}

impl<T: Float> Real for RealImpl<T> {
    const PI: Self = Self(T::PI);
    fn signum(self) -> Self {
        Self::new(self.0.signum())
    }
    fn floor(self) -> Self {
        Self::new(self.0.floor())
    }
    fn ceil(self) -> Self {
        Self::new(self.0.ceil())
    }
    fn sqrt(self) -> Self {
        Self::new(self.0.sqrt())
    }
    fn tan(self) -> Self {
        Self::new(self.0.tan())
    }
    fn sin(self) -> Self {
        Self::new(self.0.sin())
    }
    fn cos(self) -> Self {
        Self::new(self.0.cos())
    }
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self::new(sin), Self::new(cos))
    }
    fn atan2(y: Self, x: Self) -> Self {
        Self::new(T::atan2(y.0, x.0))
    }
    fn as_f32(self) -> f32 {
        self.0.as_f32()
    }
}

pub type R32 = RealImpl<f32>;
pub fn r32(value: f32) -> R32 {
    R32::new(value)
}
pub type R64 = RealImpl<f64>;
pub fn r64(value: f64) -> R64 {
    R64::new(value)
}

#[test]
fn test_reals() {
    let a = r64(3.0);
    let b = r64(2.0);
    println!("a = {:?}, b = {:?}", a, b);
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("a / b = {:?}", a / b);
    println!("sin_cos(a) = {:?}", a.sin_cos());
}

#[test]
#[should_panic]
fn test_reals_fail() {
    println!("0 / 0 = {:?}", R64::ZERO / R64::ZERO);
}
