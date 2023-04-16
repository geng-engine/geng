use batbox_la::*;
use batbox_lapp::*;
use serde::{Deserialize, Serialize};

mod camera_2d;
mod pixel_perfect;

pub use camera_2d::*;
pub use pixel_perfect::*;

/// Represents any 3d camera.
pub trait AbstractCamera3d {
    fn view_matrix(&self) -> mat4<f32>;
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32>;

    fn uniforms(&self, framebuffer_size: vec2<f32>) -> Uniforms3d {
        Uniforms3d::new(self, framebuffer_size)
    }

    fn world_to_screen(&self, framebuffer_size: vec2<f32>, pos: vec3<f32>) -> Option<vec2<f32>> {
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

    fn pixel_ray(&self, framebuffer_size: vec2<f32>, pos: vec2<f32>) -> Ray {
        let pos = vec2(
            pos.x / framebuffer_size.x * 2.0 - 1.0,
            pos.y / framebuffer_size.y * 2.0 - 1.0,
        );
        // proj * view * (rx, ry, 0, 1 / w) = (px, py, ?, 1)
        let inv_matrix = (self.projection_matrix(framebuffer_size) * self.view_matrix()).inverse();
        let p1 = inv_matrix * pos.extend(0.0).extend(1.0);
        let p2 = inv_matrix * pos.extend(1.0).extend(1.0);
        let p1 = p1.xyz() / p1.w;
        let p2 = p2.xyz() / p2.w;
        Ray {
            from: p1,
            dir: p2 - p1,
        }
    }
}

/// Represents any 2d camera.
pub trait AbstractCamera2d {
    fn view_matrix(&self) -> mat3<f32>;
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32>;

    fn uniforms(&self, framebuffer_size: vec2<f32>) -> Uniforms2d {
        Uniforms2d::new(self, framebuffer_size)
    }

    fn screen_to_world(&self, framebuffer_size: vec2<f32>, pos: vec2<f32>) -> vec2<f32> {
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

    fn world_to_screen(&self, framebuffer_size: vec2<f32>, pos: vec2<f32>) -> Option<vec2<f32>> {
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

    fn view_area(&self, framebuffer_size: vec2<f32>) -> Quad<f32> {
        Quad {
            transform: (self.projection_matrix(framebuffer_size) * self.view_matrix()).inverse(),
        }
    }
}

pub struct Camera2dAs3d<T>(pub T);

impl<C: AbstractCamera2d> AbstractCamera3d for Camera2dAs3d<C> {
    fn view_matrix(&self) -> mat4<f32> {
        self.0.view_matrix().extend3d()
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32> {
        self.0.projection_matrix(framebuffer_size).extend3d()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ray {
    pub from: vec3<f32>,
    pub dir: vec3<f32>,
}

#[derive(ugli::Uniforms)]
pub struct Uniforms3d {
    pub u_projection_matrix: mat4<f32>,
    pub u_view_matrix: mat4<f32>,
}

impl Uniforms3d {
    pub fn new<C: AbstractCamera3d + ?Sized>(camera: &C, framebuffer_size: vec2<f32>) -> Self {
        Self {
            u_projection_matrix: camera.projection_matrix(framebuffer_size),
            u_view_matrix: camera.view_matrix(),
        }
    }
}

#[derive(ugli::Uniforms)]
pub struct Uniforms2d {
    pub u_projection_matrix: mat3<f32>,
    pub u_view_matrix: mat3<f32>,
}

impl Uniforms2d {
    pub fn new<C: AbstractCamera2d + ?Sized>(camera: &C, framebuffer_size: vec2<f32>) -> Self {
        Self {
            u_projection_matrix: camera.projection_matrix(framebuffer_size),
            u_view_matrix: camera.view_matrix(),
        }
    }
}
