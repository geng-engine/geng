use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Quad<T: Float> {
    matrix: Mat3<T>,
}

impl<T: Float> Quad<T> {
    pub fn unit() -> Self {
        Self {
            matrix: Mat3::identity(),
        }
    }
    pub fn from_matrix(matrix: Mat3<T>) -> Self {
        Self { matrix }
    }
    pub fn matrix(&self) -> Mat3<T> {
        self.matrix
    }
}

impl Transform2d for Quad<f32> {
    fn bounding_quad(&self) -> Quad<f32> {
        *self
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.matrix = transform * self.matrix;
    }
}

impl FitTarget2d for Quad<f32> {
    fn make_fit(&self, object: &mut impl Transform2d) {
        let inversed_matrix = self.matrix().inverse();
        let local_transform = object
            .bounding_quad()
            .transform(inversed_matrix)
            .transformed()
            .fit_into(AABB::point(Vec2::ZERO).extend_uniform(1.0))
            .transform;
        object.apply_transform(self.matrix() * local_transform * inversed_matrix)
    }
}
