use super::*;

pub struct PixelPerfectCamera;

impl Camera for PixelPerfectCamera {
    fn view_matrix(&self) -> Mat4<f32> {
        Mat4::identity()
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32> {
        Mat4::translate(vec3(-1.0, -1.0, 0.0))
            * Mat4::scale(vec3(
                2.0 / framebuffer_size.x,
                2.0 / framebuffer_size.y,
                1.0,
            ))
    }
}
