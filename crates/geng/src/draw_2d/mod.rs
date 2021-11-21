use super::*;

mod ellipse;
mod polygon;
mod quad;
mod text;

pub use ellipse::Ellipse;
pub use ellipse::*;
pub use polygon::*;
pub use quad::Quad;
pub use quad::*;
pub use text::*;

#[derive(ugli::Vertex, Copy, Clone, Debug)]
pub struct ColoredVertex {
    pub a_pos: Vec2<f32>,
    pub a_color: Color<f32>,
}

#[derive(ugli::Vertex, Copy, Clone, Debug)]
pub struct Vertex {
    a_pos: Vec2<f32>,
}

#[derive(ugli::Vertex, Copy, Clone, Debug)]
pub struct TexturedVertex {
    pub a_pos: Vec2<f32>,
    pub a_color: Color<f32>,
    pub a_vt: Vec2<f32>,
}

impl From<Vec2<f32>> for ColoredVertex {
    fn from(v: Vec2<f32>) -> ColoredVertex {
        ColoredVertex {
            a_pos: v,
            a_color: Color::WHITE,
        }
    }
}

pub struct Helper {
    geometry: RefCell<ugli::VertexBuffer<ColoredVertex>>,
    textured_geometry: RefCell<ugli::VertexBuffer<TexturedVertex>>,
    pub(crate) color_program: ugli::Program,
    pub(crate) textured_program: ugli::Program,
    unit_quad_geometry: ugli::VertexBuffer<TexturedVertex>,
    pub(crate) ellipse_program: ugli::Program,
}

pub trait Draw2d: Transform2d {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    );
    fn draw_2d(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
    ) {
        self.draw_2d_transformed(geng, framebuffer, camera, Mat3::identity());
    }
}

impl<T: Draw2d + ?Sized> Draw2d for Box<T> {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        (**self).draw_2d_transformed(geng, framebuffer, camera, transform);
    }
}

impl<'a, T: Draw2d + ?Sized> Draw2d for Transformed2d<'a, T> {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        self.inner
            .draw_2d_transformed(geng, framebuffer, camera, transform * self.transform);
    }
}

impl Geng {
    pub fn draw_2d(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        drawable: &impl Draw2d,
    ) {
        self.draw_2d_transformed(framebuffer, camera, drawable, Mat3::identity());
    }
    pub fn draw_2d_transformed(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        drawable: &impl Draw2d,
        transform: Mat3<f32>,
    ) {
        drawable.draw_2d_transformed(self, framebuffer, camera, transform);
    }
    #[deprecated]
    pub fn draw_2d_helper(&self) -> &Helper {
        &self.inner.draw_2d
    }
}

impl Helper {
    pub(crate) fn new(shader_lib: &ShaderLib, ugli: &Ugli) -> Self {
        Self {
            geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(ugli, Vec::new())),
            textured_geometry: RefCell::new(ugli::VertexBuffer::new_dynamic(ugli, Vec::new())),
            color_program: shader_lib
                .compile(include_str!("shaders/color.glsl"))
                .unwrap(),
            textured_program: shader_lib
                .compile(include_str!("shaders/textured.glsl"))
                .unwrap(),
            unit_quad_geometry: ugli::VertexBuffer::new_static(
                ugli,
                vec![
                    TexturedVertex {
                        a_pos: vec2(-1.0, -1.0),
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: vec2(1.0, -1.0),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: vec2(1.0, 1.0),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 1.0),
                    },
                    TexturedVertex {
                        a_pos: vec2(-1.0, 1.0),
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 1.0),
                    },
                ],
            ),
            ellipse_program: shader_lib
                .compile(include_str!("shaders/ellipse.glsl"))
                .unwrap(),
        }
    }

    pub fn draw<V>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        vertices: &[V],
        color: Color<f32>,
        mode: ugli::DrawMode,
    ) where
        V: Copy + Into<ColoredVertex>,
    {
        let framebuffer_size = framebuffer.size();
        let mut geometry = self.geometry.borrow_mut();
        {
            let geometry: &mut Vec<ColoredVertex> = &mut geometry;
            geometry.clear();
            for &vertex in vertices {
                geometry.push(vertex.into());
            }
        }
        ugli::draw(
            framebuffer,
            &self.color_program,
            mode,
            &*geometry,
            (
                ugli::uniforms! {
                    u_color: color,
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: Mat3::<f32>::identity(),
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn draw_textured<V>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
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
            (
                ugli::uniforms! {
                    u_color: color,
                    u_texture: texture,
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: Mat3::<f32>::identity(),
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn quad(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: AABB<f32>,
        color: Color<f32>,
    ) {
        self.draw(
            framebuffer,
            camera,
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

    pub fn textured_quad(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: AABB<f32>,
        texture: &ugli::Texture,
        color: Color<f32>,
    ) {
        self.draw_textured(
            framebuffer,
            camera,
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

    pub fn ellipse_with_cut(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: Vec2<f32>,
        radius: Vec2<f32>,
        inner_cut: f32,
        color: Color<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &self.ellipse_program,
            ugli::DrawMode::TriangleFan,
            &self.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_model_matrix: Mat3::translate(position) * Mat3::scale(radius),
                    u_color: color,
                    u_framebuffer_size: framebuffer_size,
                    u_inner_cut: inner_cut,
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        )
    }

    pub fn ellipse(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: Vec2<f32>,
        radius: Vec2<f32>,
        color: Color<f32>,
    ) {
        self.ellipse_with_cut(framebuffer, camera, position, radius, 0.0, color);
    }

    pub fn circle_with_cut(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: Vec2<f32>,
        inner_radius: f32,
        outer_radius: f32,
        color: Color<f32>,
    ) {
        self.ellipse_with_cut(
            framebuffer,
            camera,
            position,
            vec2(outer_radius, outer_radius),
            inner_radius / outer_radius,
            color,
        );
    }

    pub fn circle(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
        position: Vec2<f32>,
        radius: f32,
        color: Color<f32>,
    ) {
        self.ellipse(framebuffer, camera, position, vec2(radius, radius), color);
    }
}
