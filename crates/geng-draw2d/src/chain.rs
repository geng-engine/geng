use super::*;

pub struct Chain {
    pub transform: mat3<f32>,
    pub vertices: Vec<ColoredVertex>,
}

impl Chain {
    pub fn new(
        chain: batbox_lapp::Chain<f32>,
        width: f32,
        color: Rgba<f32>,
        round_resolution: usize,
    ) -> Self {
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
            round_resolution,
        )
    }

    pub fn new_gradient(vertices: Vec<ColoredVertex>, width: f32, round_resolution: usize) -> Self {
        let len = vertices.len();
        if len < 2 {
            return Self {
                transform: mat3::identity(),
                vertices: vec![],
            };
        }

        let polygon_vertices = (len - 1) * 6;
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
        let mut vertex_iter = vertices.iter().copied();
        let (mut prev, mut current) = (vertex_iter.next().unwrap(), vertex_iter.next().unwrap());
        {
            for next in vertex_iter {
                // Calculate angles
                let backward = (prev.a_pos - current.a_pos).normalize_or_zero();
                let forward = (next.a_pos - current.a_pos).normalize_or_zero();
                if backward == vec2::ZERO || forward == vec2::ZERO {
                    // Too small distance
                    current = next;
                    continue;
                }

                let cos = -vec2::dot(forward, backward);
                let cos_half = ((cos + 1.0) / 2.0).max(0.0).sqrt();

                if cos_half.approx_eq(&1.0) {
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

                // Magic constant (0.1) avoids very large distance when the angle is small
                // (i.e. when the chain is going back at itself)
                let d = width / cos_half.max(0.1) / 2.0;

                let inside_dir = (backward + forward).normalize_or_zero();
                let inner = current.a_pos + inside_dir * d;

                // Positive side -> turn left
                // Negative side -> turn right
                let side = vec2::dot(
                    (next.a_pos - prev.a_pos).normalize_or_zero().rotate_90(),
                    inside_dir,
                )
                .signum();

                let inner_vertex = ColoredVertex {
                    a_pos: inner,
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
                {
                    let angle = vec2::dot(forward_norm, backward_norm)
                        .clamp(-1.0, 1.0) // Clamp for good measure (because of float inconsistency)
                        .acos();
                    let (start, end, shift) = if side.is_sign_positive() {
                        (back_vertex, forward_vertex, backward_norm * width)
                    } else {
                        (forward_vertex, back_vertex, forward_norm * width)
                    };
                    let mut round = Vec::with_capacity(round_resolution + 2);
                    round.push(start);
                    for i in 1..=round_resolution {
                        round.push(ColoredVertex {
                            a_pos: inner
                                + shift.rotate(angle * i as f32 / (round_resolution + 1) as f32),
                            ..current
                        });
                    }
                    round.push(end);

                    // Triangle fan
                    for i in 0..=round_resolution {
                        polygon.push(inner_vertex);
                        polygon.push(round[i]);
                        polygon.push(round[i + 1]);
                    }
                }

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

                prev = current;
                current = next;
            }
        }

        // End
        {
            let dir = (current.a_pos - prev.a_pos).normalize_or_zero().rotate_90() * width / 2.0;
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
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }

    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl Draw2d for Chain {
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
            ugli::DrawMode::Triangles,
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
