use geng::Draw2d;

use super::*;

pub struct Painter {
    geng: Geng,
    egui_texture: ugli::Texture,
}

impl Painter {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            egui_texture: ugli::Texture2d::new_uninitialized(geng.ugli(), vec2(1, 1)),
        }
    }

    pub fn paint(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        meshes: Vec<egui::ClippedMesh>,
        texture: &egui::Texture,
    ) {
        for egui::ClippedMesh(clip_rect, mesh) in meshes {
            self.paint_job(framebuffer, clip_rect, mesh);
        }
    }

    fn paint_job(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        clip_rect: egui::Rect,
        mesh: egui::epaint::Mesh,
    ) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let clip_aabb = AABB::from_corners(
            pos_to_vec(clip_rect.min, framebuffer_size.y),
            pos_to_vec(clip_rect.max, framebuffer_size.y),
        );

        let texture = match mesh.texture_id {
            egui::TextureId::Egui => &self.egui_texture,
            egui::TextureId::User(id) => todo!(),
        };

        let mut vertices = mesh.vertices.into_iter();
        for triangle in (0..mesh.indices.len() / 3).map(|_| {
            (
                textured_vertex(vertices.next().unwrap()),
                textured_vertex(vertices.next().unwrap()),
                textured_vertex(vertices.next().unwrap()),
            )
        }) {
            let vertices = vec![triangle.0, triangle.1, triangle.2];
            draw_2d::TexturedPolygon::new(clip_aabb, texture).draw_2d(
                &self.geng,
                framebuffer,
                &geng::PixelPerfectCamera,
            );
        }
    }
}

/// Converts [egui::Pos2] to [Vec2]. Moves the origin from top-left to bottom-left.
fn pos_to_vec(pos: egui::Pos2, height: f32) -> Vec2<f32> {
    vec2(pos.x, height - pos.y)
}

fn textured_vertex(egui_vertex: egui::epaint::Vertex) -> draw_2d::TexturedVertex {
    draw_2d::TexturedVertex {
        a_pos: pos_to_vec(egui_vertex.pos, todo!()),
        a_vt: pos_to_vec(egui_vertex.uv, 1.0),
        a_color: Color::rgba(
            egui_vertex.color.r(),
            egui_vertex.color.g(),
            egui_vertex.color.b(),
            egui_vertex.color.a(),
        )
        .convert(),
    }
}
