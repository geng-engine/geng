use super::*;

#[derive(ugli::Vertex, Copy, Clone, Debug)]
pub struct Vertex {
    pub a_pos: Vec2<f32>,
    pub a_color: Color<f32>,
}

#[derive(ugli::Vertex, Copy, Clone, Debug)]
struct EllipseVertex {
    a_quad_pos: Vec2<f32>,
}

#[derive(ugli::Vertex, Copy, Clone, Debug)]
pub struct TexturedVertex {
    pub a_pos: Vec2<f32>,
    pub a_color: Color<f32>,
    pub a_vt: Vec2<f32>,
}

impl From<Vec2<f32>> for Vertex {
    fn from(v: Vec2<f32>) -> Vertex {
        Vertex {
            a_pos: v,
            a_color: Color::WHITE,
        }
    }
}

pub struct Draw2D {
    geometry: RefCell<ugli::VertexBuffer<Vertex>>,
    textured_geometry: RefCell<ugli::VertexBuffer<TexturedVertex>>,
    program: ugli::Program,
    textured_program: ugli::Program,
    ellipse_geometry: ugli::VertexBuffer<EllipseVertex>,
    ellipse_program: ugli::Program,
}

impl Draw2D {
    pub(crate) fn new(shader_lib: &ShaderLib, ugli: &Rc<Ugli>) -> Self {
        Self {
            geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(ugli, Vec::new())),
            textured_geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(ugli, Vec::new())),
            program: shader_lib.compile(include_str!("color.glsl")).unwrap(),
            textured_program: shader_lib.compile(include_str!("textured.glsl")).unwrap(),
            ellipse_geometry: ugli::VertexBuffer::new_static(
                ugli,
                vec![
                    EllipseVertex {
                        a_quad_pos: vec2(-1.0, -1.0),
                    },
                    EllipseVertex {
                        a_quad_pos: vec2(1.0, -1.0),
                    },
                    EllipseVertex {
                        a_quad_pos: vec2(1.0, 1.0),
                    },
                    EllipseVertex {
                        a_quad_pos: vec2(-1.0, 1.0),
                    },
                ],
            ),
            ellipse_program: shader_lib.compile(include_str!("ellipse.glsl")).unwrap(),
        }
    }

    pub fn draw<V>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        vertices: &[V],
        color: Color<f32>,
        mode: ugli::DrawMode,
    ) where
        V: Copy + Into<Vertex>,
    {
        let framebuffer_size = framebuffer.size();
        let mut geometry = self.geometry.borrow_mut();
        {
            let geometry: &mut Vec<Vertex> = &mut geometry;
            geometry.clear();
            for &vertex in vertices {
                geometry.push(vertex.into());
            }
        }
        ugli::draw(
            framebuffer,
            &self.program,
            mode,
            &*geometry,
            ugli::uniforms! {
                u_color: color,
                u_framebuffer_size: framebuffer_size,
            },
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn quad(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: AABB<f32>,
        color: Color<f32>,
    ) {
        self.draw(
            framebuffer,
            &[
                position.bottom_left(),
                position.bottom_right(),
                position.top_right(),
                position.top_left(),
            ],
            color,
            ugli::DrawMode::TriangleFan,
        );
    }

    pub fn textured<V>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        vertices: &[V],
        texture: &ugli::Texture,
        color: Color<f32>,
        mode: ugli::DrawMode,
    ) where
        V: Copy + Into<TexturedVertex>,
    {
        let framebuffer_size = framebuffer.size();
        let mut geometry = self.textured_geometry.borrow_mut();
        {
            let geometry: &mut Vec<TexturedVertex> = &mut geometry;
            geometry.clear();
            for &vertex in vertices {
                geometry.push(vertex.into());
            }
        }
        ugli::draw(
            framebuffer,
            &self.textured_program,
            mode,
            &*geometry,
            ugli::uniforms! {
                u_color: color,
                u_framebuffer_size: framebuffer_size,
                u_texture: texture,
            },
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn textured_quad(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: AABB<f32>,
        texture: &ugli::Texture,
        color: Color<f32>,
    ) {
        self.textured(
            framebuffer,
            &[
                TexturedVertex {
                    a_pos: position.bottom_left(),
                    a_vt: vec2(0.0, 0.0),
                    a_color: Color::WHITE,
                },
                TexturedVertex {
                    a_pos: position.bottom_right(),
                    a_vt: vec2(1.0, 0.0),
                    a_color: Color::WHITE,
                },
                TexturedVertex {
                    a_pos: position.top_right(),
                    a_vt: vec2(1.0, 1.0),
                    a_color: Color::WHITE,
                },
                TexturedVertex {
                    a_pos: position.top_left(),
                    a_vt: vec2(0.0, 1.0),
                    a_color: Color::WHITE,
                },
            ],
            texture,
            color,
            ugli::DrawMode::TriangleFan,
        )
    }

    pub fn ellipse(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<f32>,
        radius: Vec2<f32>,
        color: Color<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &self.ellipse_program,
            ugli::DrawMode::TriangleFan,
            &self.ellipse_geometry,
            ugli::uniforms! {
                u_pos: position,
                u_radius: radius,
                u_color: color,
                u_framebuffer_size: framebuffer_size,
            },
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        self.ellipse(framebuffer, position, vec2(radius, radius), color);
    }
}
