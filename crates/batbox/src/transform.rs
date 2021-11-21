use super::*;

pub trait Transform2d {
    fn bounding_quad(&self) -> Quad<f32>;
    fn apply_transform(&mut self, transform: Mat3<f32>);
}

impl<T: Transform2d + ?Sized> Transform2d for Box<T> {
    fn bounding_quad(&self) -> Quad<f32> {
        (**self).bounding_quad()
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        (**self).apply_transform(transform);
    }
}

pub struct Transformed2d<'a, T: Transform2d + ?Sized> {
    pub inner: &'a T,
    pub transform: Mat3<f32>,
}

impl<'a, T: Transform2d + ?Sized> Transformed2d<'a, T> {
    pub fn new(inner: &'a T, transform: Mat3<f32>) -> Self {
        Self { inner, transform }
    }
}

impl<'a, T: Transform2d + ?Sized> Transform2d for Transformed2d<'a, T> {
    fn bounding_quad(&self) -> Quad<f32> {
        self.inner.bounding_quad().transform(self.transform)
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

pub trait Transform2dExt: Transform2d {
    fn transform(self, transform: Mat3<f32>) -> Self
    where
        Self: Sized,
    {
        let mut result = self;
        result.apply_transform(transform);
        result
    }
    fn transformed(&self) -> Transformed2d<Self> {
        Transformed2d::new(self, Mat3::identity())
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
            .map(|p| (self.bounding_quad().matrix() * p.extend(1.0)).into_2d()),
        )
    }
    fn fit_into(self, aabb: AABB<f32>) -> Self
    where
        Self: Sized,
    {
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

impl<T: Transform2d + ?Sized> Transform2dExt for T {}
