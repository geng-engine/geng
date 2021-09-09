use super::*;

mod camera_2d;
mod pixel_perfect;

pub use camera_2d::*;
pub use pixel_perfect::*;

pub trait Camera {
    fn view_matrix(&self) -> Mat4<f32>;
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CameraRay {
    pub from: Vec3<f32>,
    pub dir: Vec3<f32>,
}

pub trait CameraExt: Camera {
    fn world_to_screen(&self, framebuffer_size: Vec2<f32>, pos: Vec3<f32>) -> Option<Vec2<f32>> {
        let pos = (self.projection_matrix(framebuffer_size) * self.view_matrix()) * pos.extend(1.0);
        let pos = pos.xyz() / pos.w;
        if pos.x.abs() > 1.0 || pos.y.abs() > 1.0 || pos.z.abs() > 1.0 {
            return None;
        }
        Some(vec2(
            (pos.x + 1.0) / 2.0 * framebuffer_size.x,
            (pos.y + 1.0) / 2.0 * framebuffer_size.y,
        ))
    }
    fn pixel_ray(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> CameraRay {
        let pos = vec2(
            pos.x / framebuffer_size.x as f32 * 2.0 - 1.0,
            pos.y / framebuffer_size.y as f32 * 2.0 - 1.0,
        );
        // proj * view * (rx, ry, 0, 1 / w) = (px, py, ?, 1)
        let inv_matrix = (self.projection_matrix(framebuffer_size) * self.view_matrix()).inverse();
        let p1 = inv_matrix * pos.extend(0.0).extend(1.0);
        let p2 = inv_matrix * pos.extend(1.0).extend(1.0);
        let p1 = p1.xyz() / p1.w;
        let p2 = p2.xyz() / p2.w;
        CameraRay {
            from: p1,
            dir: p2 - p1,
        }
    }
}

impl<C: Camera> CameraExt for C {}

pub fn camera_uniforms<C: Camera>(camera: &C, framebuffer_size: Vec2<f32>) -> impl ugli::Uniforms {
    ugli::uniforms! {
        u_projection_matrix: camera.projection_matrix(framebuffer_size),
        u_view_matrix: camera.view_matrix(),
    }
}
