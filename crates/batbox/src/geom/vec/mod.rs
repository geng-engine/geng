use super::*;

mod _2d;
mod _3d;
mod _4d;

pub use _2d::*;
pub use _3d::*;
pub use _4d::*;

macro_rules! left_mul_impl {
    ($name:ident for $($typ:ty),*) => {$(
        impl Mul<$name<$typ>> for $typ {
            type Output = $name<$typ>;
            fn mul(self, rhs: $name<$typ>) -> $name<$typ> {
                rhs * self
            }
        }
    )*}
}

macro_rules! vec_impl_ops {
    ($name:ident : $($f:tt),*) => {
        impl<T: Add<Output=T>> Add for $name<T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f + rhs.$f,)*
                }
            }
        }

        impl<T: AddAssign> AddAssign for $name<T> {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$f += rhs.$f;)*
            }
        }

        impl<T: Sub<Output=T>> Sub for $name<T> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f - rhs.$f,)*
                }
            }
        }

        impl<T: SubAssign> SubAssign for $name<T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$f -= rhs.$f;)*
            }
        }

        impl<T: Neg<Output=T>> Neg for $name<T> {
            type Output = Self;
            fn neg(self) -> Self {
                Self {
                    $($f: -self.$f,)*
                }
            }
        }

        impl<T: Mul<Output=T>> Mul for $name<T> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f * rhs.$f,)*
                }
            }
        }

        impl<T: MulAssign> MulAssign for $name<T> {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$f *= rhs.$f;)*
            }
        }

        impl<T: Div<Output=T>> Div for $name<T> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f / rhs.$f,)*
                }
            }
        }

        impl<T: DivAssign> DivAssign for $name<T> {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$f /= rhs.$f;)*
            }
        }

        impl<T: Copy + Mul<Output=T>> Mul<T> for $name<T> {
            type Output = Self;
            fn mul(self, rhs: T) -> Self {
                Self {
                    $($f: self.$f * rhs,)*
                }
            }
        }

        left_mul_impl!($name for f32, f64, i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

        impl<T: Copy + MulAssign> MulAssign<T> for $name<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$f *= rhs;)*
            }
        }

        impl<T: Copy + Div<Output=T>> Div<T> for $name<T> {
            type Output = Self;
            fn div(self, rhs: T) -> Self {
                Self {
                    $($f: self.$f / rhs,)*
                }
            }
        }

        impl<T: Copy + DivAssign> DivAssign<T> for $name<T> {
            fn div_assign(&mut self, rhs: T) {
                $(self.$f /= rhs;)*
            }
        }
    };
}

vec_impl_ops!(vec2: 0, 1);
vec_impl_ops!(vec3: 0, 1, 2);
vec_impl_ops!(vec4: 0, 1, 2, 3);
