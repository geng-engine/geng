use super::*;

/// 2d camera used for rendering in pixel space.
pub struct PixelPerfectCamera;

impl AbstractCamera2d for PixelPerfectCamera {
    fn view_matrix(&self) -> mat3<f32> {
        mat3::identity()
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32> {
        mat3::translate(vec2(-1.0, -1.0))
            * mat3::scale(vec2(2.0 / framebuffer_size.x, 2.0 / framebuffer_size.y))
    }
}
