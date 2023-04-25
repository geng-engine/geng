use super::*;

/// An object in 2d space, bounded by a [Quad]
///
/// TODO: better name
pub trait Transform2d<F: Float> {
    /// Object's bounding [Quad]
    fn bounding_quad(&self) -> Quad<F>;

    /// Apply transformation matrix to this object
    fn apply_transform(&mut self, transform: mat3<F>);
}

impl<F: Float, T: Transform2d<F> + ?Sized> Transform2d<F> for Box<T> {
    fn bounding_quad(&self) -> Quad<F> {
        (**self).bounding_quad()
    }
    fn apply_transform(&mut self, transform: mat3<F>) {
        (**self).apply_transform(transform);
    }
}

/// A reference to a 2d object with additional transformation applied
///
// TODO: is this needed? should it be reference?
pub struct Transformed2d<'a, F: Float, T: Transform2d<F> + ?Sized> {
    /// Reference to the actual object
    pub inner: &'a T,

    /// Additional transformation
    pub transform: mat3<F>,
}

impl<'a, F: Float, T: Transform2d<F> + ?Sized> Transformed2d<'a, F, T> {
    /// Apply additional transformation to the given object
    pub fn new(inner: &'a T, transform: mat3<F>) -> Self {
        Self { inner, transform }
    }
}

impl<'a, F: Float, T: Transform2d<F> + ?Sized> Transform2d<F> for Transformed2d<'a, F, T> {
    fn bounding_quad(&self) -> Quad<F> {
        self.inner.bounding_quad().transform(self.transform)
    }
    fn apply_transform(&mut self, transform: mat3<F>) {
        self.transform = transform * self.transform;
    }
}

/// Extra methods for [Transform2d] types
pub trait Transform2dExt<F: Float>: Transform2d<F> {
    /// Apply transformation to the object, returning a modified value to allow chaining methods
    fn transform(self, transform: mat3<F>) -> Self
    where
        Self: Sized,
    {
        let mut result = self;
        result.apply_transform(transform);
        result
    }

    /// Align bounding box of this object, making its origin located at (0, 0)
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// # use batbox_lapp::*;
    /// let quad = Quad::unit();
    /// assert_eq!(quad.bounding_box(), Aabb2::from_corners(vec2(-1.0, -1.0), vec2(1.0, 1.0)));
    /// let quad = quad.align_bounding_box(vec2(0.0, 1.0));
    /// assert_eq!(quad.bounding_box(), Aabb2::from_corners(vec2(0.0, 0.0), vec2(2.0, -2.0)));
    /// ```
    fn align_bounding_box(self, alignment: vec2<F>) -> Self
    where
        Self: Sized,
    {
        let aabb = self.bounding_box();
        self.translate(-aabb.bottom_left() - aabb.size() * alignment)
    }

    /// Rotate object around (0, 0) by given angle (in radians)
    fn rotate(self, rot: F) -> Self
    where
        Self: Sized,
    {
        self.transform(mat3::rotate(rot))
    }

    /// Translate object by given vector
    fn translate(self, delta: vec2<F>) -> Self
    where
        Self: Sized,
    {
        self.transform(mat3::translate(delta))
    }

    /// Scale object around (0, 0) by given factor
    fn scale(self, factor: vec2<F>) -> Self
    where
        Self: Sized,
    {
        self.transform(mat3::scale(factor))
    }

    /// Scale object around (0, 0) by given factor uniformly along both axis
    fn scale_uniform(self, factor: F) -> Self
    where
        Self: Sized,
    {
        self.transform(mat3::scale_uniform(factor))
    }

    /// Create a reference to this object with additional transformation applied
    ///
    // TODO: bad naming
    fn transformed(&self) -> Transformed2d<F, Self> {
        Transformed2d::new(self, mat3::identity())
    }

    /// Calculate bounding box of this object, getting [Aabb2] instead of a [Quad]
    fn bounding_box(&self) -> Aabb2<F> {
        Aabb2::points_bounding_box(
            [
                vec2(-F::ONE, -F::ONE),
                vec2(F::ONE, -F::ONE),
                vec2(F::ONE, F::ONE),
                vec2(-F::ONE, F::ONE),
            ]
            .into_iter()
            .map(|p| (self.bounding_quad().transform * p.extend(F::ONE)).into_2d()),
        )
    }

    /// Make this object's bounding [Quad] fit into given target
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
