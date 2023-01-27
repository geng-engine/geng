use super::*;

/// [Generalizes a circle](https://en.wikipedia.org/wiki/Ellipse)
pub struct Ellipse<T> {
    /// Transformation from a [unit](Ellipse::unit)
    pub transform: mat3<T>,
}

impl<T: Float> Ellipse<T> {
    /// Create a new ellipse with given center and given half size
    pub fn new(center: vec2<T>, half_size: vec2<T>) -> Self {
        Self {
            transform: mat3::translate(center) * mat3::scale(half_size),
        }
    }

    /// Create a circle with given center and radius
    pub fn circle(center: vec2<T>, radius: T) -> Self {
        Self {
            transform: mat3::translate(center) * mat3::scale_uniform(radius),
        }
    }

    /// Create a unit ellipse - a circle with center at (0, 0) and radius of 1
    pub fn unit() -> Self {
        Self {
            transform: mat3::identity(),
        }
    }
}

impl<T: Float> Transform2d<T> for Ellipse<T> {
    fn bounding_quad(&self) -> Quad<T> {
        Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<T>) {
        self.transform = transform * self.transform;
    }
}

impl<T: Float> FitTarget2d<T> for Ellipse<T> {
    fn make_fit(&self, object: &mut impl Transform2d<T>) {
        let inversed_matrix = self.transform.inverse();
        let quad_in_circle = object.bounding_quad().transform(inversed_matrix);
        let center = (quad_in_circle.transform * vec3(T::ZERO, T::ZERO, T::ONE)).into_2d();
        let corner = (quad_in_circle.transform * vec3(T::ONE, T::ONE, T::ONE)).into_2d();
        let local_transform =
            mat3::scale_uniform(T::ONE / (corner - center).len()) * mat3::translate(-center);
        object.apply_transform(self.transform * local_transform * inversed_matrix)
    }
}
