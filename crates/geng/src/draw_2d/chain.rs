use super::*;

pub struct Chain {
    transform: Mat3<f32>,
    vertices: Vec<ColoredVertex>,
}

impl Chain {
    pub fn new(chain: batbox::Chain<f32>, width: f32, color: Color<f32>) -> Self {
        Self::new_gradient(
            chain
                .vertices
                .into_iter()
                .map(|pos| ColoredVertex {
                    a_pos: pos,
                    a_color: color,
                })
                .collect(),
            width,
        )
    }

    pub fn new_gradient(vertices: Vec<ColoredVertex>, width: f32) -> Self {
        let len = vertices.len();
        if len < 2 {
            return Self {
                transform: Mat3::identity(),
                vertices: vec![],
            };
        }

        let mut polygon = Vec::with_capacity(len * 2);

        fn add(
            polygon: &mut Vec<ColoredVertex>,
            vertex: ColoredVertex,
            direction: Vec2<f32>,
            width: f32,
        ) {
            let normal = direction.normalize_or_zero().rotate_90();
            let shift = normal * width / 2.0;
            polygon.push(ColoredVertex {
                a_pos: vertex.a_pos + shift,
                ..vertex
            });
            polygon.push(ColoredVertex {
                a_pos: vertex.a_pos - shift,
                ..vertex
            });
        }

        // Start
        add(
            &mut polygon,
            vertices[0],
            vertices[1].a_pos - vertices[0].a_pos,
            width,
        );

        // Middle
        for ((prev, current), next) in vertices
            .iter()
            .copied()
            .zip(vertices.iter().copied().skip(1))
            .zip(vertices.iter().copied().skip(2))
        {
            let forward = (current.a_pos - next.a_pos).normalize_or_zero();
            let backward = (current.a_pos - prev.a_pos).normalize_or_zero();
            let cos = -Vec2::dot(forward, backward);
            let cos_half = ((cos + 1.0) / 2.0).sqrt();
            add(
                &mut polygon,
                current,
                next.a_pos - prev.a_pos,
                width / cos_half,
            );
        }

        // End
        add(
            &mut polygon,
            vertices[len - 1],
            vertices[len - 1].a_pos - vertices[len - 2].a_pos,
            width,
        );

        let (transform, vertices) = Polygon::normalize(polygon);
        Self {
            transform,
            vertices,
        }
    }
}

impl Transform2d<f32> for Chain {
    fn bounding_quad(&self) -> batbox::Quad<f32> {
        batbox::Quad::from_matrix(self.transform)
    }

    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl Draw2d for Chain {
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
            ugli::DrawMode::TriangleStrip,
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
