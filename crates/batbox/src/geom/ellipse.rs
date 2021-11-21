use super::*;

pub struct Ellipse<T> {
    matrix: Mat3<T>,
}

impl<T: Float> Ellipse<T> {
    pub fn new(center: Vec2<T>, size: Vec2<T>) -> Self {
        Self {
            matrix: Mat3::translate(center) * Mat3::scale(size),
        }
    }
    pub fn circle(center: Vec2<T>, radius: T) -> Self {
        Self {
            matrix: Mat3::translate(center) * Mat3::scale_uniform(radius),
        }
    }
    pub fn unit() -> Self {
        Self {
            matrix: Mat3::identity(),
        }
    }
    pub fn matrix(&self) -> Mat3<T> {
        self.matrix
    }
}

impl<T: Float> Transform2d<T> for Ellipse<T> {
    fn bounding_quad(&self) -> Quad<T> {
        Quad::from_matrix(self.matrix)
    }
    fn apply_transform(&mut self, transform: Mat3<T>) {
        self.matrix = transform * self.matrix;
    }
}

impl<T: Float> FitTarget2d<T> for Ellipse<T> {
    fn make_fit(&self, object: &mut impl Transform2d<T>) {
        let inversed_matrix = self.matrix().inverse();
        let quad_in_circle = object.bounding_quad().transform(inversed_matrix);
        let center = (quad_in_circle.matrix() * vec3(T::ZERO, T::ZERO, T::ONE)).into_2d();
        let corner = (quad_in_circle.matrix() * vec3(T::ONE, T::ONE, T::ONE)).into_2d();
        let local_transform =
            Mat3::scale_uniform(T::ONE / (corner - center).len()) * Mat3::translate(-center);
        object.apply_transform(self.matrix() * local_transform * inversed_matrix)
    }
}
