use crate::*;

#[derive(ugli::Vertex)]
pub struct Vertex {
    a_pos: Vec2<f32>,
}

pub struct EmptyLoadingScreen;

impl ProgressScreen for EmptyLoadingScreen {}

impl State for EmptyLoadingScreen {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {}
}
