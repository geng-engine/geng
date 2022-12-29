use super::*;

mod float;
mod real;

pub use float::*;
pub use real::*;

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
    const ZERO: Self;
    const ONE: Self;
}

pub trait Num: UNum + Neg<Output = Self> {
    fn abs(self) -> Self {
        if self >= Self::ZERO {
            self
        } else {
            -self
        }
    }
}

impl<T: UNum + Neg<Output = T>> Num for T {}

macro_rules! impl_uint {
    ($($t:ty),*) => {
        $(
            impl UNum for $t {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )*
    };
}

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

impl_uint! { u8, u16, u32, u64, usize }
impl_int! { i8, i16, i32, i64, isize }

pub trait MulExt: Mul + Sized + Copy {
    fn sqr(self) -> <Self as Mul>::Output {
        self * self
    }
}

impl<T: Mul + Copy> MulExt for T {}
