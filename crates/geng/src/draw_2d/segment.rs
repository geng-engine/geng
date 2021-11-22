use super::*;

pub struct Segment {
    transform: Mat3<f32>,
    vertices: Vec<ColoredVertex>,
}

impl Segment {
    pub fn new(segment: batbox::Segment<f32>, width: f32, color: Color<f32>) -> Self {
        Self::new_gradient(
            ColoredVertex {
                a_pos: segment.start,
                a_color: color,
            },
            ColoredVertex {
                a_pos: segment.end,
                a_color: color,
            },
            width,
        )
    }

    pub fn new_gradient(start: ColoredVertex, end: ColoredVertex, width: f32) -> Self {
        let mut vertices = Vec::with_capacity(4);
        let normal = (end.a_pos - start.a_pos).normalize_or_zero().rotate_90();
        vertices.push(ColoredVertex {
            a_pos: start.a_pos - normal * width,
            a_color: start.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: start.a_pos + normal * width,
            a_color: start.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: end.a_pos + normal * width,
            a_color: end.a_color,
        });
        vertices.push(ColoredVertex {
            a_pos: end.a_pos - normal * width,
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
    fn bounding_quad(&self) -> batbox::Quad<f32> {
        batbox::Quad::from_matrix(self.transform)
    }

    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl Draw2d for Segment {
    fn draw_2d_transformed(
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
            &ugli::VertexBuffer::new_dynamic(geng.ugli(), self.vertices.clone()),
            (
                ugli::uniforms! {
                    u_color: Color::WHITE,
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
