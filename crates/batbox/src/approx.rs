use super::*;

const DEFAULT_EPS: f32 = 1e-9;

pub trait ApproxEq {
    fn approx_distance_to(&self, other: &Self) -> f32;

    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, DEFAULT_EPS)
    }
    fn approx_eq_eps(&self, other: &Self, eps: f32) -> bool {
        self.approx_distance_to(other) <= eps
    }
}

impl<T: Float> ApproxEq for T {
    fn approx_distance_to(&self, other: &T) -> f32 {
        (self.as_f32() - other.as_f32()).abs()
    }
}

#[test]
fn test_approx_eq_f32() {
    assert!(1.0_f32.approx_eq_eps(&1.1_f32, 0.15_f32));
    assert!(23424.2143_f32.approx_eq(&23424.2143_f32));
    assert!(!1.0_f32.approx_eq(&1.1_f32));
    assert!(!24352.64_f32.approx_eq(&-54.0_f32));
    assert!(!1.0_f32.approx_eq_eps(&2.0_f32, 0.5_f32));
}

#[test]
fn test_approx_eq_f64() {
    assert!(1.0_f64.approx_eq_eps(&1.1_f64, 0.15_f32));
    assert!(23424.2143_f64.approx_eq(&23424.2143_f64));
    assert!(!1.0_f64.approx_eq(&1.1_f64));
    assert!(!24352.64_f64.approx_eq(&-54.0_f64));
    assert!(!1.0_f64.approx_eq_eps(&2.0_f64, 0.5_f32));
}
