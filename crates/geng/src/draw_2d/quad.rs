use super::*;

pub struct Quad {
    transform: Mat3<f32>,
    color: Color<f32>,
}

impl Quad {
    pub fn new(aabb: AABB<f32>, color: Color<f32>) -> Self {
        Self::unit(color).transform(Mat3::translate(aabb.center()) * Mat3::scale(aabb.size() / 2.0))
    }
    pub fn unit(color: Color<f32>) -> Self {
        Self {
            transform: Mat3::identity(),
            color,
        }
    }
}

impl Draw2d for Quad {
    fn draw_2d(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &geng.inner.draw_2d.color_program,
            ugli::DrawMode::TriangleFan,
            &geng.inner.draw_2d.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_color: self.color,
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: transform * self.transform,
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}

impl Transform2d for Quad {
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

pub struct TexturedQuad<T: std::borrow::Borrow<ugli::Texture>> {
    transform: Mat3<f32>,
    texture: T,
    color: Color<f32>,
}

impl<T: std::borrow::Borrow<ugli::Texture>> TexturedQuad<T> {
    pub fn new(aabb: AABB<f32>, texture: T) -> Self {
        Self::colored(aabb, texture, Color::WHITE)
    }
    pub fn colored(aabb: AABB<f32>, texture: T, color: Color<f32>) -> Self {
        Self::unit_colored(texture, color)
            .transform(Mat3::translate(aabb.center()) * Mat3::scale(aabb.size() / 2.0))
    }
    pub fn unit(texture: T) -> Self {
        Self::unit_colored(texture, Color::WHITE)
    }
    pub fn unit_colored(texture: T, color: Color<f32>) -> Self {
        Self {
            transform: Mat3::identity(),
            texture,
            color,
        }
    }
}

impl<T: std::borrow::Borrow<ugli::Texture>> Draw2d for TexturedQuad<T> {
    fn draw_2d(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &geng.inner.draw_2d.textured_program,
            ugli::DrawMode::TriangleFan,
            &geng.inner.draw_2d.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_color: self.color,
                    u_texture: self.texture.borrow(),
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: transform * self.transform,
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}

impl<T: std::borrow::Borrow<ugli::Texture>> Transform2d for TexturedQuad<T> {
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
