use super::*;

#[derive(ugli::Uniforms)]
pub struct Uniforms {
    u_color: Option<Color<f32>>,
    u_projection_matrix: Mat3<f32>,
    u_view_matrix: Mat3<f32>,
}

#[derive(ugli::Uniforms)]
pub struct UniformsRef<'a> {
    u_color: Option<Color<f32>>,
    u_texture: Option<&'a ugli::Texture>,
    u_projection_matrix: Mat3<f32>,
    u_view_matrix: Mat3<f32>,
}

#[derive(ugli::Vertex)]
pub struct EmptyVertex;

impl<Camera: AbstractCamera2d + ?Sized> Drawable<Camera, Color<f32>> for AABB<f32> {
    type Vertex = draw_2d::Vertex;
    type Instance = EmptyVertex;
    type Uniforms = Uniforms;
    fn vertices(&self) -> Vec<Self::Vertex> {
        vec![
            draw_2d::Vertex {
                a_pos: self.bottom_left(),
                a_color: Color::WHITE,
            },
            draw_2d::Vertex {
                a_pos: self.bottom_right(),
                a_color: Color::WHITE,
            },
            draw_2d::Vertex {
                a_pos: self.top_right(),
                a_color: Color::WHITE,
            },
            draw_2d::Vertex {
                a_pos: self.top_left(),
                a_color: Color::WHITE,
            },
        ]
    }
    fn instances(&self) -> Option<Vec<Self::Instance>> {
        None
    }
    fn draw_mode() -> ugli::DrawMode {
        ugli::DrawMode::TriangleFan
    }
    fn draw_parameters(&self, _options: Color<f32>) -> ugli::DrawParameters {
        ugli::DrawParameters {
            blend_mode: Some(default()),
            ..default()
        }
    }
    fn uniforms(
        &self,
        framebuffer: &ugli::Framebuffer,
        camera: &Camera,
        color: Color<f32>,
    ) -> Uniforms {
        Uniforms {
            u_color: Some(color),
            u_projection_matrix: camera.projection_matrix(framebuffer.size().map(|x| x as f32)),
            u_view_matrix: camera.view_matrix(),
        }
    }
    fn program(geng: &Geng) -> &ugli::Program {
        &geng.draw_2d().program
    }
}

impl<'a, Camera: AbstractCamera2d + ?Sized> Drawable<Camera, &'a ugli::Texture> for AABB<f32> {
    type Vertex = draw_2d::TexturedVertex;
    type Instance = EmptyVertex;
    type Uniforms = UniformsRef<'a>;
    fn vertices(&self) -> Vec<Self::Vertex> {
        vec![
            draw_2d::TexturedVertex {
                a_pos: self.bottom_left(),
                a_vt: vec2(0.0, 0.0),
                a_color: Color::WHITE,
            },
            draw_2d::TexturedVertex {
                a_pos: self.bottom_right(),
                a_vt: vec2(1.0, 0.0),
                a_color: Color::WHITE,
            },
            draw_2d::TexturedVertex {
                a_pos: self.top_right(),
                a_vt: vec2(1.0, 1.0),
                a_color: Color::WHITE,
            },
            draw_2d::TexturedVertex {
                a_pos: self.top_left(),
                a_vt: vec2(0.0, 1.0),
                a_color: Color::WHITE,
            },
        ]
    }
    fn instances(&self) -> Option<Vec<Self::Instance>> {
        None
    }
    fn draw_mode() -> ugli::DrawMode {
        ugli::DrawMode::TriangleFan
    }
    fn draw_parameters(&self, _options: &ugli::Texture) -> ugli::DrawParameters {
        ugli::DrawParameters {
            blend_mode: Some(default()),
            ..default()
        }
    }
    fn uniforms(
        &self,
        framebuffer: &ugli::Framebuffer,
        camera: &Camera,
        texture: &'a ugli::Texture,
    ) -> UniformsRef<'a> {
        UniformsRef {
            u_color: Some(Color::WHITE),
            u_projection_matrix: camera.projection_matrix(framebuffer.size().map(|x| x as f32)),
            u_view_matrix: camera.view_matrix(),
            u_texture: Some(texture),
        }
    }
    fn program(geng: &Geng) -> &ugli::Program {
        &geng.draw_2d().textured_program
    }
}
