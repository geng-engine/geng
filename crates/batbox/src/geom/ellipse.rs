use super::*;

pub struct Ellipse {
    matrix: Mat3<f32>,
}

impl Ellipse {
    pub fn new(center: Vec2<f32>, size: Vec2<f32>) -> Self {
        Self::unit().transform(Mat3::translate(center) * Mat3::scale(size))
    }
    pub fn circle(center: Vec2<f32>, radius: f32) -> Self {
        Self::unit().transform(Mat3::translate(center) * Mat3::scale_uniform(radius))
    }
    pub fn circle_with_cut(center: Vec2<f32>, radius: f32) -> Self {
        Self {
            ..Self::unit().transform(Mat3::translate(center) * Mat3::scale_uniform(radius))
        }
    }
    pub fn unit() -> Self {
        Self {
            matrix: Mat3::identity(),
        }
    }
    pub fn unit_with_cut() -> Self {
        Self {
            matrix: Mat3::identity(),
        }
    }
    pub fn matrix(&self) -> Mat3<f32> {
        self.matrix
    }
}

impl Transform2d for Ellipse {
    fn bounding_quad(&self) -> Quad<f32> {
        Quad::from_matrix(self.matrix)
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.matrix = transform * self.matrix;
    }
}
