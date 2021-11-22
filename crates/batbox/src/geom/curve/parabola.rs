use super::*;

/// Represents a parabola curve `f(t) = at^2 + bt + c`.
pub struct ParabolaCurve<T> {
    equation: [Vec2<T>; 3],
}

impl<F: Float> ParabolaCurve<F> {
    /// Creates a new parabol passing through three points
    pub fn new(points: [Vec2<F>; 3]) -> Self {
        let [p0, p1, p2] = points;
        // f(0) = p0 (start)    | c = p0
        // f(0.5) = p1 (middle) | 0.25 * a + 0.5 * b + c = p1
        // f(1) = p2 (end)      | a + b + c = p2
        let c = p0;
        let b = p1 * F::from_f32(4.0) - p2 - p0 * F::from_f32(3.0);
        let a = p2 - p0 - b;

        Self {
            equation: [a, b, c],
        }
    }

    /// Returns a point on the parabola
    pub fn get(&self, t: F) -> Vec2<F> {
        let [a, b, c] = self.equation;
        a * t * t + b * t + c
    }
}

impl<F> Curve<F> for ParabolaCurve<F> {
    fn chain(&self, resolution: usize) -> Chain<F>
    where
        F: Float,
    {
        let mut vertices = Vec::with_capacity(resolution * 2);

        let step = 0.5 / resolution as f32;
        for i in 0..=resolution * 2 {
            let t = F::from_f32(step * i as f32);
            vertices.push(self.get(t));
        }

        Chain { vertices }
    }
}
