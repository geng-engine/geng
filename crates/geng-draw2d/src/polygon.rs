use super::*;

pub struct Polygon {
    pub transform: mat3<f32>,
    pub draw_mode: ugli::DrawMode,
    pub vertices: Vec<ColoredVertex>,
}

impl Polygon {
    pub fn new(vertices: Vec<vec2<f32>>, color: Rgba<f32>) -> Self {
        Self::new_gradient(
            vertices
                .into_iter()
                .map(|vertex| ColoredVertex {
                    a_pos: vertex,
                    a_color: color,
                })
                .collect(),
        )
    }
    pub fn new_gradient(vertices: Vec<ColoredVertex>) -> Self {
        let (transform, vertices) = Self::normalize(vertices);
        Self {
            transform,
            vertices,
            draw_mode: ugli::DrawMode::TriangleFan,
        }
    }
    pub fn strip(vertices: Vec<vec2<f32>>, color: Rgba<f32>) -> Self {
        Self::strip_gradient(
            vertices
                .into_iter()
                .map(|vertex| ColoredVertex {
                    a_pos: vertex,
                    a_color: color,
                })
                .collect(),
        )
    }
    pub fn strip_gradient(vertices: Vec<ColoredVertex>) -> Self {
        let (transform, vertices) = Self::normalize(vertices);
        Self {
            transform,
            vertices,
            draw_mode: ugli::DrawMode::TriangleStrip,
        }
    }

    pub(super) fn normalize(mut vertices: Vec<ColoredVertex>) -> (mat3<f32>, Vec<ColoredVertex>) {
        let aabb = Aabb2::points_bounding_box(vertices.iter().map(|vertex| vertex.a_pos));
        let transform = mat3::translate(aabb.center()) * mat3::scale(aabb.size() / 2.0);
        let inverse = transform.inverse();
        for vertex in &mut vertices {
            vertex.a_pos = (inverse * vertex.a_pos.extend(1.0)).xy();
        }
        (transform, vertices)
    }
}

impl Draw2d for Polygon {
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
            self.draw_mode,
            &ugli::VertexBuffer::new_dynamic(helper.ugli(), self.vertices.clone()),
            (
                ugli::uniforms! {
                    u_color: Rgba::WHITE,
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

impl Transform2d<f32> for Polygon {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

pub struct TexturedPolygon<T: std::borrow::Borrow<ugli::Texture>> {
    transform: mat3<f32>,
    texture: T,
    draw_mode: ugli::DrawMode,
    vertices: Vec<TexturedVertex>,
}

impl<T: std::borrow::Borrow<ugli::Texture>> TexturedPolygon<T> {
    pub fn new(vertices: Vec<TexturedVertex>, texture: T) -> Self {
        let (transform, vertices) = Self::normalize(vertices);
        Self {
            transform,
            texture,
            vertices,
            draw_mode: ugli::DrawMode::TriangleFan,
        }
    }
    pub fn with_mode(vertices: Vec<TexturedVertex>, texture: T, draw_mode: ugli::DrawMode) -> Self {
        let (transform, vertices) = Self::normalize(vertices);
        Self {
            transform,
            texture,
            vertices,
            draw_mode,
        }
    }
    pub fn strip(vertices: Vec<TexturedVertex>, texture: T) -> Self {
        let (transform, vertices) = Self::normalize(vertices);
        Self {
            transform,
            texture,
            vertices,
            draw_mode: ugli::DrawMode::TriangleStrip,
        }
    }

    pub(super) fn normalize(mut vertices: Vec<TexturedVertex>) -> (mat3<f32>, Vec<TexturedVertex>) {
        let aabb = Aabb2::points_bounding_box(vertices.iter().map(|vertex| vertex.a_pos));
        let transform = mat3::translate(aabb.center()) * mat3::scale(aabb.size() / 2.0);
        let inverse = transform.inverse();
        for vertex in &mut vertices {
            vertex.a_pos = (inverse * vertex.a_pos.extend(1.0)).xy();
        }
        (transform, vertices)
    }
}

impl<T: std::borrow::Borrow<ugli::Texture>> Draw2d for TexturedPolygon<T> {
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
            self.draw_mode,
            &ugli::VertexBuffer::new_dynamic(helper.ugli(), self.vertices.clone()),
            (
                ugli::uniforms! {
                    u_color: Rgba::WHITE,
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

impl<T: std::borrow::Borrow<ugli::Texture>> Transform2d<f32> for TexturedPolygon<T> {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
