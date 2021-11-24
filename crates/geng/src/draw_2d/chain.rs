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

        let polygon_vertices = (len - 1) * 6; // + (len - 2) * (round_resolution + 1) * 3;
        let mut polygon = Vec::with_capacity(polygon_vertices);

        // Start
        {
            let dir = (vertices[1].a_pos - vertices[0].a_pos)
                .normalize_or_zero()
                .rotate_90()
                * width
                / 2.0;
            polygon.push(ColoredVertex {
                a_pos: vertices[0].a_pos + dir,
                ..vertices[0]
            });
            let right = ColoredVertex {
                a_pos: vertices[0].a_pos - dir,
                ..vertices[0]
            };
            polygon.push(right);
            polygon.push(right); // Temp
            polygon.push(right);
        }

        // Middle
        for ((prev, current), next) in vertices
            .iter()
            .copied()
            .zip(vertices.iter().skip(1).copied())
            .zip(vertices.iter().skip(2).copied())
        {
            // Calculate angles
            let backward = (prev.a_pos - current.a_pos).normalize_or_zero();
            let forward = (next.a_pos - current.a_pos).normalize_or_zero();
            let cos = -Vec2::dot(forward, backward);
            let cos_half = ((cos + 1.0) / 2.0).max(0.0).sqrt();

            if cos_half.approx_eq(&0.0) {
                // Straight line -> no rounding
                let dir =
                    (current.a_pos - prev.a_pos).normalize_or_zero().rotate_90() * width / 2.0;
                let left = ColoredVertex {
                    a_pos: current.a_pos + dir,
                    ..current
                };
                let right = ColoredVertex {
                    a_pos: current.a_pos - dir,
                    ..current
                };
                // Finish incoming segment
                let temp = polygon.len() - 2;
                polygon[temp] = left;
                polygon.push(left);
                polygon.push(right);
                // Start outcoming segment
                polygon.push(left);
                polygon.push(right);
                polygon.push(right); // Temp
                polygon.push(right);
                continue;
            }

            let d = width / cos_half.max(0.1) / 2.0; // Magic constant (0.1) avoids very large distance

            let inside_dir = (backward + forward).normalize_or_zero();
            let inner = current.a_pos + inside_dir * d;

            // Positive side -> turn left
            // Negative side -> turn right
            let side = Vec2::dot(
                (next.a_pos - prev.a_pos).normalize_or_zero().rotate_90(),
                inside_dir,
            )
            .signum();

            let inner_vertex = ColoredVertex {
                a_pos: inner,
                ..current
            };

            let middle_vertex = ColoredVertex {
                a_pos: inner - inside_dir * width,
                ..current
            };

            let backward_norm = backward.rotate_90() * side;
            let back_vertex = ColoredVertex {
                a_pos: inner + backward_norm * width,
                ..current
            };

            let forward_norm = -forward.rotate_90() * side;
            let forward_vertex = ColoredVertex {
                a_pos: inner + forward_norm * width,
                ..current
            };

            // Finish incoming segment
            {
                let (left, right) = if side.is_sign_positive() {
                    (inner_vertex, back_vertex) // Turn left
                } else {
                    (back_vertex, inner_vertex) // Turn right
                };
                let temp = polygon.len() - 2;
                polygon[temp] = left;
                polygon.push(left);
                polygon.push(right);
            }

            // Round
            polygon.push(back_vertex);
            polygon.push(inner_vertex);
            polygon.push(middle_vertex);

            polygon.push(forward_vertex);
            polygon.push(inner_vertex);
            polygon.push(middle_vertex);

            // Start outcoming segment
            {
                let (left, right) = if side.is_sign_positive() {
                    (inner_vertex, forward_vertex) // Turn left
                } else {
                    (forward_vertex, inner_vertex) // Turn right
                };
                polygon.push(left);
                polygon.push(right);
                polygon.push(right); // Temp
                polygon.push(right);
            }
        }

        // End
        {
            let dir = (vertices[len - 1].a_pos - vertices[len - 2].a_pos)
                .normalize_or_zero()
                .rotate_90()
                * width
                / 2.0;
            let left = ColoredVertex {
                a_pos: vertices[len - 1].a_pos + dir,
                ..vertices[len - 1]
            };
            let temp = polygon.len() - 2;
            polygon[temp] = left; // Temp
            polygon.push(left);
            polygon.push(ColoredVertex {
                a_pos: vertices[len - 1].a_pos - dir,
                ..vertices[len - 1]
            });
        }

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
            ugli::DrawMode::Triangles,
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
