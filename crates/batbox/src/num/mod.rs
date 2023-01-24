//! Numeric types and traits
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use crate::num::{self, r32, r64, Float, Num, Real, UNum, R32, R64};
}

mod float;
mod real;

pub use float::*;
pub use real::*;

/// Generic number, possibly unsigned
pub trait UNum:
    Sized
    + Copy
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
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
pub trait Num: UNum + Neg<Output = Self> {
    /// Calculate absolute value
    fn abs(self) -> Self {
        if self >= Self::ZERO {
            self
        } else {
            -self
        }
    }
}

impl<T: UNum + Neg<Output = T>> Num for T {}

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
