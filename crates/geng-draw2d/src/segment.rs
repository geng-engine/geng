use super::*;

pub struct Segment {
    pub transform: mat3<f32>,
    pub vertices: Vec<ColoredVertex>,
}

impl Segment {
    pub fn new(segment: batbox_lapp::Segment<f32>, width: f32, color: Rgba<f32>) -> Self {
        Self::new_gradient(
            ColoredVertex {
                a_pos: segment.0,
                a_color: color,
            },
            ColoredVertex {
                a_pos: segment.1,
                a_color: color,
            },
            width,
        )
    }

    pub fn new_gradient(start: ColoredVertex, end: ColoredVertex, width: f32) -> Self {
        let half_width = width / 2.0;
        let mut vertices = Vec::with_capacity(4);
        let normal = (end.a_pos - start.a_pos).normalize_or_zero().rotate_90();
        vertices.push(ColoredVertex {
            a_pos: start.a_pos - normal * half_width,
            a_color: start.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: start.a_pos + normal * half_width,
            a_color: start.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: end.a_pos + normal * half_width,
            a_color: end.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: end.a_pos - normal * half_width,
            a_color: end.a_color,
        });
        let (transform, vertices) = Polygon::normalize(vertices);
        Self {
            transform,
            vertices,
        }
    }
}

impl Transform2d<f32> for Segment {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }

    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl Draw2d for Segment {
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
