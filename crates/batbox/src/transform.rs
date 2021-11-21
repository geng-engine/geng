use super::*;

pub trait Transform2d {
    fn bounding_quad(&self) -> Mat3<f32>;
    fn apply_transform(&mut self, transform: Mat3<f32>);
}

impl<T: Transform2d + ?Sized> Transform2d for Box<T> {
    fn bounding_quad(&self) -> Mat3<f32> {
        (**self).bounding_quad()
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        (**self).apply_transform(transform);
    }
}

impl Transform2d for Mat3<f32> {
    fn bounding_quad(&self) -> Mat3<f32> {
        *self
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        *self = transform * *self;
    }
}

pub trait Transform2dExt: Transform2d + Sized {
    fn transform(self, transform: Mat3<f32>) -> Self {
        let mut result = self;
        result.apply_transform(transform);
        result
    }
    fn bounding_box(&self) -> AABB<f32> {
        AABB::points_bounding_box(
            [
                vec2(-1.0, -1.0),
                vec2(1.0, -1.0),
                vec2(1.0, 1.0),
                vec2(-1.0, 1.0),
            ]
            .into_iter()
            .map(|p| (self.bounding_quad() * p.extend(1.0)).into_2d()),
        )
    }
    fn fit_into(self, aabb: AABB<f32>) -> Self {
        let current_aabb = self.bounding_box();
        let scale = partial_min(
            aabb.height() / current_aabb.height(),
            aabb.width() / current_aabb.width(),
        );
        self.transform(
            Mat3::translate(aabb.center())
                * Mat3::scale_uniform(scale)
                * Mat3::translate(-current_aabb.center()),
        )
    }
}

impl<T: Transform2d> Transform2dExt for T {}
