use super::*;

#[derive(Debug, Clone)]
pub struct Camera2d {
    pub center: Vec2<f32>,
    pub rotation: f32,
    pub fov: f32,
}

impl AbstractCamera2d for Camera2d {
    fn view_matrix(&self) -> Mat3<f32> {
        Mat3::rotate(self.rotation) * Mat3::translate(-self.center)
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat3<f32> {
        Mat3::scale(vec2(2.0 * framebuffer_size.y / framebuffer_size.x, 2.0) / self.fov)
    }
}
