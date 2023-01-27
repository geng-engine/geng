use super::*;

/// Represents any curve given by a parametric function `f(t)`.
pub struct Trajectory<T> {
    function: Box<dyn Fn(T) -> vec2<T>>,
    interval: RangeInclusive<T>,
}

impl<T> Trajectory<T> {
    /// Construct a new trajectory with a function and an interval for the parameter.
    pub fn new(function: Box<dyn Fn(T) -> vec2<T>>, interval: RangeInclusive<T>) -> Self {
        Self { function, interval }
    }

    /// Get a point on the trajectory.
    pub fn get(&self, t: T) -> vec2<T> {
        (self.function)(t)
    }

    /// Construct a parabolic trajectory passing through three points.
    /// The interval between the points is `-1.0..=1.0`
    pub fn parabola(points: [vec2<T>; 3], interval: RangeInclusive<T>) -> Self
    where
        T: Float + 'static,
    {
        let [p0, p1, p2] = points;
        // f(t)  = a * t^2 + b * t + c
        // f(-1) = p0    (start)     | a - b + c = p0
        // f(0)  = p1    (middle)    | c = p1
        // f(1)  = p2    (end)       | a + b + c = p2
        let c = p1;
        let a = (p2 + p0) / T::from_f32(2.0) - c;
        let b = (p2 - p0) / T::from_f32(2.0);

        Self {
            function: Box::new(move |t| a * t * t + b * t + c),
            interval,
        }
    }
}

impl<T> Curve<T> for Trajectory<T> {
    fn chain(&self, resolution: usize) -> Chain<T>
    where
        T: Float,
    {
        let mut vertices = Vec::with_capacity(resolution);

        let start = *self.interval.start();
        let end = *self.interval.end();
        let step = (end - start) / T::from_f32(resolution as f32);
        for i in 0..=resolution {
            let t = start + step * T::from_f32(i as f32);
            vertices.push(self.get(t));
        }

        Chain { vertices }
    }
}
