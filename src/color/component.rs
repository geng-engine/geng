#[allow(unused_imports)]
use crate::*;

pub trait ColorComponent: Copy {
    const ZERO: Self;
    const HALF: Self;
    const MAX: Self;
    fn as_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;

    fn convert<U: ColorComponent>(self) -> U {
        U::from_f32(self.as_f32())
    }
}

impl ColorComponent for f32 {
    const ZERO: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
    fn as_f32(self) -> f32 {
        self
    }
    fn from_f32(x: f32) -> Self {
        x
    }
}

impl ColorComponent for f64 {
    const ZERO: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
    fn as_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(x: f32) -> Self {
        Self::from(x)
    }
}

impl ColorComponent for u8 {
    const ZERO: Self = 0;
    const HALF: Self = 0x7f;
    const MAX: Self = 0xff;
    fn as_f32(self) -> f32 {
        f32::from(self) / f32::from(Self::MAX)
    }
    fn from_f32(x: f32) -> Self {
        (x * f32::from(Self::MAX)) as Self
    }
}

#[test]
fn test_convert() {
    use crate::*;
    fn assert_eq<T: ColorComponent>(val: T, val_f32: f32, val_f64: f64, val_u8: u8) {
        assert!(val.convert::<f32>().approx_eq_eps(&val_f32, 1.0 / 255.0));
        assert!(val.convert::<f64>().approx_eq_eps(&val_f64, 1.0 / 255.0));
        assert_eq!(val.convert::<u8>(), val_u8);
    }
    fn assert_all_eq(val_f32: f32, val_f64: f64, val_u8: u8) {
        assert_eq(val_f32, val_f32, val_f64, val_u8);
        assert_eq(val_f64, val_f32, val_f64, val_u8);
        assert_eq(val_u8, val_f32, val_f64, val_u8);
    }
    assert_all_eq(0.0, 0.0, 0);
    assert_all_eq(1.0, 1.0, 0xff);
    assert_all_eq(0.5, 0.5, 0x7f);
}
