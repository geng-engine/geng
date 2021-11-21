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
    fn fit_into(self, target: impl FitTarget2d) -> Self
    where
        Self: Sized,
    {
        let mut result = self;
        target.make_fit(&mut result);
        result
    }
}

impl<T: Transform2d + ?Sized> Transform2dExt for T {}

pub trait FitTarget2d {
    fn make_fit(&self, object: &mut impl Transform2d);
}

impl FitTarget2d for AABB<f32> {
    fn make_fit(&self, object: &mut impl Transform2d) {
        let current_aabb = object.bounding_box();
        let scale = partial_min(
            self.height() / current_aabb.height(),
            self.width() / current_aabb.width(),
        );
        object.apply_transform(
            Mat3::translate(self.center())
                * Mat3::scale_uniform(scale)
                * Mat3::translate(-current_aabb.center()),
        );
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
