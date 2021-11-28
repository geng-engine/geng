use super::*;

pub struct Painter {
    geng: Geng,
    textured_program: ugli::Program,
    egui_texture_version: u64,
    egui_texture: ugli::Texture,
}

impl Painter {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            textured_program: geng
                .shader_lib()
                .compile(include_str!("textured.glsl"))
                .unwrap(),
            egui_texture_version: 0,
            egui_texture: ugli::Texture2d::new_uninitialized(geng.ugli(), vec2(1, 1)),
        }
    }

    pub fn paint(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        meshes: Vec<egui::ClippedMesh>,
        egui_texture: &egui::Texture,
    ) {
        // Update texture
        if self.egui_texture_version != egui_texture.version {
            self.rebuild_texture(egui_texture);
        }

        // Render mesh
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

        // Convert egui clip_rect to geng clip_aabb
        let clip_aabb = AABB::from_corners(
            pos_to_vec(clip_rect.min, framebuffer_size.y),
            pos_to_vec(clip_rect.max, framebuffer_size.y),
        )
        .map(|x| x as usize);

        // Get font texture
        let texture = match mesh.texture_id {
            egui::TextureId::Egui => &self.egui_texture,
            egui::TextureId::User(_id) => todo!(),
        };

        // Convert egui vertices to geng vertices
        let vertex_shift = clip_aabb.bottom_left().map(|x| x as f32);
        let vertices: Vec<_> = mesh
            .indices
            .into_iter()
            .map(|i| {
                let mut vertex = textured_vertex(mesh.vertices[i as usize], framebuffer_size.y);
                vertex.a_pos -= vertex_shift; // Because mask is applied relative to the origin
                vertex
            })
            .collect();

        // Render triangles
        ugli::draw(
            framebuffer,
            &self.textured_program,
            ugli::DrawMode::Triangles,
            &ugli::VertexBuffer::new_dynamic(self.geng.ugli(), vertices),
            (
                ugli::uniforms! {
                    u_color: Color::WHITE,
                    u_texture: texture,
                    u_framebuffer_size: clip_aabb.size(),
                    u_model_matrix: Mat3::identity(),
                },
                geng::camera2d_uniforms(
                    &geng::PixelPerfectCamera,
                    clip_aabb.size().map(|x| x as f32),
                ),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                viewport: Some(clip_aabb),
                ..default()
            },
        );
    }

    fn rebuild_texture(&mut self, egui_texture: &egui::Texture) {
        // Update version
        self.egui_texture_version = egui_texture.version;

        // Update texture
        self.egui_texture = ugli::Texture::new_with(
            self.geng.ugli(),
            vec2(egui_texture.width, egui_texture.height),
            |pixel| {
                Color::rgba(
                    255,
                    255,
                    255,
                    egui_texture.pixels
                        [pixel.x + (egui_texture.height - 1 - pixel.y) * egui_texture.width], // Geng textures have origin in the bottom-left
                )
                .convert()
            },
        );
    }
}

fn textured_vertex(egui_vertex: egui::epaint::Vertex, height: f32) -> draw_2d::TexturedVertex {
    draw_2d::TexturedVertex {
        a_pos: pos_to_vec(egui_vertex.pos, height),
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
