use super::*;

pub struct Painter {}

impl Painter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn paint(&mut self, geng: &Geng, meshes: Vec<egui::ClippedMesh>, texture: &egui::Texture) {}
}
