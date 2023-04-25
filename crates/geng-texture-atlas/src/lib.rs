use batbox_la::*;
use ugli::Ugli;

pub struct TextureAtlas {
    texture: ugli::Texture,
    uvs: Vec<Aabb2<f32>>,
}

impl TextureAtlas {
    pub fn new(ugli: &Ugli, textures: &[&ugli::Texture], filter: ugli::Filter) -> Self {
        let mut width = 0;
        let mut height = 0;
        for texture in textures {
            width += texture.size().x;
            height = height.max(texture.size().y);
        }
        let mut atlas_texture = ugli::Texture::new_uninitialized(ugli, vec2(width, height));
        let mut uvs = Vec::with_capacity(textures.len());
        let mut x = 0;
        for texture in textures {
            let framebuffer = ugli::FramebufferRead::new(
                ugli,
                ugli::ColorAttachmentRead::Texture(texture),
                ugli::DepthAttachmentRead::None,
            );
            framebuffer.copy_to_texture(
                &mut atlas_texture,
                Aabb2::ZERO.extend_positive(texture.size()),
                vec2(x, 0),
            );
            uvs.push(
                Aabb2::point(vec2(x as f32 / width as f32, 0.0)).extend_positive(vec2(
                    texture.size().x as f32 / width as f32,
                    texture.size().y as f32 / height as f32,
                )),
            );
            x += texture.size().x;
        }
        atlas_texture.set_filter(filter);
        Self {
            texture: atlas_texture,
            uvs,
        }
    }
    pub fn uv(&self, texture_index: usize) -> Aabb2<f32> {
        self.uvs[texture_index]
    }
    pub fn texture(&self) -> &ugli::Texture {
        &self.texture
    }
    pub fn set_filter(&mut self, filter: ugli::Filter) {
        self.texture.set_filter(filter);
    }
}
