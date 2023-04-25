use super::*;

/// 2-dimensional camera.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Camera2d {
    pub center: vec2<f32>,
    pub rotation: f32,
    pub fov: f32,
}

impl AbstractCamera2d for Camera2d {
    fn view_matrix(&self) -> mat3<f32> {
        mat3::rotate(self.rotation) * mat3::translate(-self.center)
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32> {
        mat3::scale(vec2(2.0 * framebuffer_size.y / framebuffer_size.x, 2.0) / self.fov)
    }
}
