use super::*;

mod camera_2d;
mod pixel_perfect;

pub use camera_2d::*;
pub use pixel_perfect::*;

/// Represents any 3d camera.
pub trait AbstractCamera3d: Sized {
    fn view_matrix(&self) -> Mat4<f32>;
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32>;
}

/// Represents any 2d camera.
pub trait AbstractCamera2d: Sized {
    fn view_matrix(&self) -> Mat3<f32>;
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat3<f32>;
}

pub struct Camera2dAs3d<T>(pub T);

impl<C: AbstractCamera2d> AbstractCamera3d for Camera2dAs3d<C> {
    fn view_matrix(&self) -> Mat4<f32> {
        self.0.view_matrix().extend3d()
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32> {
        self.0.projection_matrix(framebuffer_size).extend3d()
    }
}

/// Extra methods available for 2d cameras.
pub trait Camera2dExt: AbstractCamera2d {
    fn screen_to_world(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> Vec2<f32> {
        let pos = vec2(
            pos.x / framebuffer_size.x * 2.0 - 1.0,
            pos.y / framebuffer_size.y * 2.0 - 1.0,
        );
        let pos = (AbstractCamera2d::projection_matrix(self, framebuffer_size)
            * AbstractCamera2d::view_matrix(self))
        .inverse()
            * pos.extend(1.0);
        pos.xy()
    }

    fn world_to_screen(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> Option<Vec2<f32>> {
        let pos = (self.projection_matrix(framebuffer_size) * self.view_matrix()) * pos.extend(1.0);
        let pos = pos.xy() / pos.z;
        if pos.x.abs() > 1.0 || pos.y.abs() > 1.0 {
            return None;
        }
        Some(vec2(
            (pos.x + 1.0) / 2.0 * framebuffer_size.x,
            (pos.y + 1.0) / 2.0 * framebuffer_size.y,
        ))
    }
}

impl<C: AbstractCamera2d> Camera2dExt for C {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CameraRay {
    pub from: Vec3<f32>,
    pub dir: Vec3<f32>,
}

/// Extra methods available for 3d cameras.
pub trait Camera3dExt: AbstractCamera3d {
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

impl<C: AbstractCamera3d> Camera3dExt for C {}

pub fn camera3d_uniforms<C: AbstractCamera3d>(
    camera: &C,
    framebuffer_size: Vec2<f32>,
) -> impl ugli::Uniforms {
    ugli::uniforms! {
        u_projection_matrix: camera.projection_matrix(framebuffer_size),
        u_view_matrix: camera.view_matrix(),
    }
}

pub fn camera2d_uniforms<C: AbstractCamera2d>(
    camera: &C,
    framebuffer_size: Vec2<f32>,
) -> impl ugli::Uniforms {
    ugli::uniforms! {
        u_projection_matrix: camera.projection_matrix(framebuffer_size),
        u_view_matrix: camera.view_matrix(),
    }
}
