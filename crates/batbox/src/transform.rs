use super::*;

pub trait Transform2d {
    fn apply_transform(&mut self, transform: Mat3<f32>);
}

impl<T: Transform2d + ?Sized> Transform2d for Box<T> {
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        (**self).apply_transform(transform);
    }
}

pub trait Transform2dExt: Transform2d + Sized {
    fn transform(self, transform: Mat3<f32>) -> Self {
        let mut result = self;
        result.apply_transform(transform);
        result
    }
}

impl<T: Transform2d> Transform2dExt for T {}
