use super::*;

/// Represents a [cardinal spline](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Cardinal_spline).
#[derive(Debug)]
pub struct CardinalSpline<T> {
    /// The key points
    pub points: Vec<vec2<T>>,

    /// Tension parameter
    pub tension: T,
}

impl<T> CardinalSpline<T> {
    /// Create a cardinal spline passing through points.
    /// Tension should be in range `0..=1`.
    /// For example, if tension is `0.5`, then the curve is a Catmull-Rom spline.
    pub fn new(points: Vec<vec2<T>>, tension: T) -> Self {
        Self { points, tension }
    }
}

impl<F: Float> CubicHermiteCurve<F> for CardinalSpline<F> {
    fn intervals(&self) -> Vec<CurveInterval<F>> {
        // Tangents
        let len = self.points.len();
        let mut m = Vec::with_capacity(len);
        if len > 1 {
            m.push((
                0,
                (self.points[1] - self.points[0]) / (F::ONE - F::ZERO) * (F::ONE - self.tension),
            ));
        }
        m.extend(
            self.points
                .iter()
                .zip(self.points.iter().skip(2))
                .map(|(&p0, &p2)| (p2 - p0) / (F::ONE - F::ZERO) * (F::ONE - self.tension))
                .enumerate()
                .map(|(i, m)| (i + 1, m)),
        );
        if len > 2 {
            m.push((
                len - 1,
                (self.points[len - 1] - self.points[len - 2]) / (F::ONE - F::ZERO)
                    * (F::ONE - self.tension),
            ));
        }
        let mut m = m.into_iter();

        let (_, mut prev) = match m.next() {
            Some(first) => first,
            None => return Vec::new(),
        };

        let mut intervals = Vec::with_capacity(len - 1);
        for (index, next) in m {
            intervals.push(CurveInterval {
                point_start: self.points[index - 1],
                point_end: self.points[index],
                tangent_start: prev,
                tangent_end: next,
            });
            prev = next;
        }

        intervals
    }
}
