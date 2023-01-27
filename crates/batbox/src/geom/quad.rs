use super::*;

/// A 2d quad
#[derive(Clone, Copy, Debug)]
pub struct Quad<T: Float> {
    /// Transformation matrix
    ///
    /// Get corners of the quad by transforming [vec2] of -1/+1 by this
    pub transform: mat3<T>,
}

impl<T: Float> Quad<T> {
    /// A unit quad - one with corners at -1/+1
    pub fn unit() -> Self {
        Self {
            transform: mat3::identity(),
        }
    }
}

impl<T: Float> Transform2d<T> for Quad<T> {
    fn bounding_quad(&self) -> Quad<T> {
        *self
    }
    fn apply_transform(&mut self, transform: mat3<T>) {
        self.transform = transform * self.transform;
    }
}

impl<T: Float> FitTarget2d<T> for Quad<T> {
    fn make_fit(&self, object: &mut impl Transform2d<T>) {
        let inversed_matrix = self.transform.inverse();
        let local_transform = object
            .bounding_quad()
            .transform(inversed_matrix)
            .transformed()
            .fit_into(Aabb2::point(vec2::ZERO).extend_uniform(T::ONE))
            .transform;
        object.apply_transform(self.transform * local_transform * inversed_matrix)
    }
}
