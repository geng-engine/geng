use super::*;

pub trait Transform2d<F: Float> {
    fn bounding_quad(&self) -> Quad<F>;
    fn apply_transform(&mut self, transform: Mat3<F>);
}

impl<F: Float, T: Transform2d<F> + ?Sized> Transform2d<F> for Box<T> {
    fn bounding_quad(&self) -> Quad<F> {
        (**self).bounding_quad()
    }
    fn apply_transform(&mut self, transform: Mat3<F>) {
        (**self).apply_transform(transform);
    }
}

pub struct Transformed2d<'a, F: Float, T: Transform2d<F> + ?Sized> {
    pub inner: &'a T,
    pub transform: Mat3<F>,
}

impl<'a, F: Float, T: Transform2d<F> + ?Sized> Transformed2d<'a, F, T> {
    pub fn new(inner: &'a T, transform: Mat3<F>) -> Self {
        Self { inner, transform }
    }
}

impl<'a, F: Float, T: Transform2d<F> + ?Sized> Transform2d<F> for Transformed2d<'a, F, T> {
    fn bounding_quad(&self) -> Quad<F> {
        self.inner.bounding_quad().transform(self.transform)
    }
    fn apply_transform(&mut self, transform: Mat3<F>) {
        self.transform = transform * self.transform;
    }
}

pub trait Transform2dExt<F: Float>: Transform2d<F> {
    fn transform(self, transform: Mat3<F>) -> Self
    where
        Self: Sized,
    {
        let mut result = self;
        result.apply_transform(transform);
        result
    }
    fn transformed(&self) -> Transformed2d<F, Self> {
        Transformed2d::new(self, Mat3::identity())
    }
    fn bounding_box(&self) -> AABB<F> {
        AABB::points_bounding_box(
            [
                vec2(-F::ONE, -F::ONE),
                vec2(F::ONE, -F::ONE),
                vec2(F::ONE, F::ONE),
                vec2(-F::ONE, F::ONE),
            ]
            .into_iter()
            .map(|p| (self.bounding_quad().matrix() * p.extend(F::ONE)).into_2d()),
        )
    }
    fn fit_into(self, target: impl FitTarget2d<F>) -> Self
    where
        Self: Sized,
    {
        let mut result = self;
        target.make_fit(&mut result);
        result
    }
}

impl<F: Float, T: Transform2d<F> + ?Sized> Transform2dExt<F> for T {}

pub trait FitTarget2d<F: Float> {
    fn make_fit(&self, object: &mut impl Transform2d<F>);
}
