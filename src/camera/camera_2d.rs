use super::*;

pub struct Camera2d {
    pub center: Vec2<f32>,
    pub max_vertical_fov: f32,
    pub max_horizontal_fov: f32,
}

impl Camera2d {
    pub fn new(center: Vec2<f32>, max_vertical_fov: f32, max_horizontal_fov: f32) -> Self {
        Self {
            center,
            max_horizontal_fov,
            max_vertical_fov,
        }
    }
    pub fn set_max_fov(&mut self, fov: f32) {
        self.max_horizontal_fov = fov;
        self.max_vertical_fov = fov;
    }
    pub fn containing(rect: AABB<f32>) -> Self {
        Self {
            center: rect.center(),
            max_horizontal_fov: rect.width(),
            max_vertical_fov: rect.height(),
        }
    }
    pub fn screen_to_world(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> Vec2<f32> {
        let pos = vec2(
            pos.x / framebuffer_size.x * 2.0 - 1.0,
            pos.y / framebuffer_size.y * 2.0 - 1.0,
        );
        let pos = (self.projection_matrix(framebuffer_size) * self.view_matrix()).inverse()
            * pos.extend(0.0).extend(1.0);
        pos.xy()
    }

    pub fn world_to_screen(
        &self,
        framebuffer_size: Vec2<f32>,
        pos: Vec2<f32>,
    ) -> Option<Vec2<f32>> {
        CameraExt::world_to_screen(self, framebuffer_size, pos.extend(0.0))
    }
}

impl Camera for Camera2d {
    fn view_matrix(&self) -> Mat4<f32> {
        Mat4::translate(-self.center.extend(0.0))
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32> {
        let mut vertical_fov = self.max_vertical_fov;
        let horizontal_fov = vertical_fov * framebuffer_size.x / framebuffer_size.y;
        if horizontal_fov > self.max_horizontal_fov {
            vertical_fov = self.max_horizontal_fov * framebuffer_size.y / framebuffer_size.x;
        }
        Mat4::scale(vec3(
            2.0 * framebuffer_size.y / framebuffer_size.x,
            2.0,
            1.0,
        )) * Mat4::scale_uniform(1.0 / vertical_fov)
    }
}
