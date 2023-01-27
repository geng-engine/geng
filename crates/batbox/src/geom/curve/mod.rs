use super::*;

mod cardinal;
mod trajectory;

pub use cardinal::*;
pub use trajectory::*;

/// A trait representing a generic curve.
pub trait Curve<F> {
    /// Converts a curve into a chain (a list of segments) for rendering and collision detection.
    fn chain(&self, resolution: usize) -> Chain<F>
    where
        F: Float;
}

/// A trait representing a [curve](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Cardinal_spline)
/// defined by intervals with key points and tangents.
pub trait CubicHermiteCurve<T> {
    /// Get the intervals of the curve.
    fn intervals(&self) -> Vec<CurveInterval<T>>;

    /// Converts a curve into a chain (a list of segments) for rendering and collision detection.
    fn chain(&self, resolution: usize) -> Chain<T>
    where
        T: Float,
    {
        let intervals = self.intervals();
        let mut vertices = Vec::with_capacity(resolution * intervals.len());
        let mut intervals = intervals.into_iter();

        // First interval includes the start
        if let Some(interval) = intervals.next() {
            let step = 1. / resolution as f32;
            for i in 0..=resolution {
                let t = T::from_f32(step * i as f32);
                vertices.push(interval.get(t));
            }
        }

        // Other intervals exclude the start
        for interval in intervals {
            let step = 1. / resolution as f32;
            for i in 1..=resolution {
                let t = T::from_f32(step * i as f32);
                vertices.push(interval.get(t));
            }
        }

        Chain { vertices }
    }
}

/// Represents a single interval of the curve.
#[derive(Debug)]
pub struct CurveInterval<T> {
    /// Starting point
    pub point_start: vec2<T>,
    /// End point
    pub point_end: vec2<T>,
    /// Starting tangent
    pub tangent_start: vec2<T>,
    /// End tangent
    pub tangent_end: vec2<T>,
}

impl<F: Float> CurveInterval<F> {
    /// Returns a point on the curve interval
    pub fn get(&self, t: F) -> vec2<F> {
        let p0 = self.point_start;
        let p1 = self.point_end;
        let m0 = self.tangent_start;
        let m1 = self.tangent_end;

        let t2 = t * t; // t^2
        let t3 = t2 * t; // t^3
        let one = F::ONE;
        let two = one + F::ONE;
        let three = two + F::ONE;
        p0 * (two * t3 - three * t2 + one)
            + m0 * (t3 - two * t2 + t)
            + p1 * (-two * t3 + three * t2)
            + m1 * (t3 - t2)
    }
}
