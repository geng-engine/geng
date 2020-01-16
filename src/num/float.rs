use crate::*;

pub trait Float: Num {
    const PI: Self;
    fn signum(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn atan2(y: Self, x: Self) -> Self;
    fn is_finite(self) -> bool;
    fn as_f32(self) -> f32;
}

macro_rules! impl_float {
    ($($t:ident),*) => {
        $(
            impl UNum for $t {
                const ZERO: Self = 0.0;
                const ONE: Self = 1.0;
            }

            impl Float for $t {
                const PI: Self = std::$t::consts::PI;
                fn signum(self) -> Self {
                    self.signum()
                }
                fn floor(self) -> Self {
                    self.floor()
                }
                fn ceil(self) -> Self {
                    self.ceil()
                }
                fn sqrt(self) -> Self {
                    self.sqrt()
                }
                fn tan(self) -> Self {
                    self.tan()
                }
                fn sin(self) -> Self {
                    self.sin()
                }
                fn cos(self) -> Self {
                    self.cos()
                }
                fn sin_cos(self) -> (Self, Self) {
                    self.sin_cos()
                }
                fn atan2(y: Self, x: Self) -> Self {
                    y.atan2(x)
                }
                fn is_finite(self) -> bool {
                    self.is_finite()
                }
                fn as_f32(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_float!(f32, f64);
