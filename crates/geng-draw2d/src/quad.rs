use super::*;

pub struct Quad {
    pub transform: mat3<f32>,
    pub color: Rgba<f32>,
}

impl Quad {
    pub fn new(aabb: Aabb2<f32>, color: Rgba<f32>) -> Self {
        Self::unit(color).transform(mat3::translate(aabb.center()) * mat3::scale(aabb.size() / 2.0))
    }
    pub fn unit(color: Rgba<f32>) -> Self {
        Self {
            transform: mat3::identity(),
            color,
        }
    }
}

impl Draw2d for Quad {
    fn draw2d_transformed(
        &self,
        helper: &Helper,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &helper.color_program,
            ugli::DrawMode::TriangleFan,
            &helper.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_color: self.color,
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: transform * self.transform,
                },
                camera.uniforms(framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..Default::default()
            },
        );
    }
}

impl Transform2d<f32> for Quad {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

pub struct TexturedQuad<T: std::borrow::Borrow<ugli::Texture>> {
    transform: mat3<f32>,
    texture: T,
    color: Rgba<f32>,
}

impl<T: std::borrow::Borrow<ugli::Texture>> TexturedQuad<T> {
    pub fn new(aabb: Aabb2<f32>, texture: T) -> Self {
        Self::colored(aabb, texture, Rgba::WHITE)
    }
    pub fn colored(aabb: Aabb2<f32>, texture: T, color: Rgba<f32>) -> Self {
        Self::unit_colored(texture, color)
            .transform(mat3::translate(aabb.center()) * mat3::scale(aabb.size() / 2.0))
    }
    pub fn unit(texture: T) -> Self {
        Self::unit_colored(texture, Rgba::WHITE)
    }
    pub fn unit_colored(texture: T, color: Rgba<f32>) -> Self {
        Self {
            transform: mat3::identity(),
            texture,
            color,
        }
    }
}

impl<T: std::borrow::Borrow<ugli::Texture>> Draw2d for TexturedQuad<T> {
    fn draw2d_transformed(
        &self,
        helper: &Helper,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &helper.textured_program,
            ugli::DrawMode::TriangleFan,
            &helper.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_color: self.color,
                    u_texture: self.texture.borrow(),
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: transform * self.transform,
                },
                camera.uniforms(framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..Default::default()
            },
        );
    }
}

impl<T: std::borrow::Borrow<ugli::Texture>> Transform2d<f32> for TexturedQuad<T> {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
