use super::*;

pub struct PixelPerfectCamera;

impl AbstractCamera2d for PixelPerfectCamera {
    fn view_matrix(&self) -> Mat3<f32> {
        Mat3::identity()
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat3<f32> {
        Mat3::translate(vec2(-1.0, -1.0))
            * Mat3::scale(vec2(2.0 / framebuffer_size.x, 2.0 / framebuffer_size.y))
    }
}
