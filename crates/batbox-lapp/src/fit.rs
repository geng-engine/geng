use super::*;

/// Represents a type that can be used to fit [Transform2d] objects into
pub trait FitTarget2d<F: Float> {
    /// Make given object's bounding [Quad] fit into self
    fn make_fit(&self, object: &mut impl Transform2d<F>);
}

impl<T: Float> FitTarget2d<T> for Aabb2<T> {
    fn make_fit(&self, object: &mut impl Transform2d<T>) {
        let current_aabb = object.bounding_box();
        let current_width = current_aabb.width();
        let current_height = current_aabb.height();
        if current_width == T::ZERO || current_height == T::ZERO {
            return;
        }
        let scale = partial_min(self.height() / current_height, self.width() / current_width);
        object.apply_transform(
            mat3::translate(self.center())
                * mat3::scale_uniform(scale)
                * mat3::translate(-current_aabb.center()),
        );
    }
}
