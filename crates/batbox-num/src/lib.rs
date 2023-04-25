//! Numeric types and traits

mod float;
mod real;

pub use float::*;
pub use real::*;

/// Generic number, possibly unsigned
pub trait UNum:
    Sized
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + PartialEq
    + PartialOrd
    + ::rand::distributions::uniform::SampleUniform
{
    /// Additive identity
    const ZERO: Self;

    /// Multiplicative identity
    const ONE: Self;

    /// Calculate squared value (`self * self`)
    fn sqr(self) -> Self {
        self * self
    }
}

/// Generic signed number type
pub trait Num: UNum + std::ops::Neg<Output = Self> {
    /// Calculate absolute value
    fn abs(self) -> Self {
        if self >= Self::ZERO {
            self
        } else {
            -self
        }
    }
}

impl<T: UNum + std::ops::Neg<Output = T>> Num for T {}

macro_rules! impl_int {
    ($($t:ty),*) => {
        $(
            impl UNum for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )*
    };
}

impl_int! { u8, u16, u32, u64, usize }
impl_int! { i8, i16, i32, i64, isize }
