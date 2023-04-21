use super::*;

pub struct ColorData<'a> {
    width: usize,
    height: usize,
    buffer: Vec<raw::UByte>, // TODO: Box<[T]>
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> ColorData<'a> {
    pub fn get(&self, x: usize, y: usize) -> Rgba<u8> {
        assert!(x < self.width);
        assert!(y < self.height);
        Rgba::new(
            self.buffer[(y * self.width + x) * 4],
            self.buffer[(y * self.width + x) * 4 + 1],
            self.buffer[(y * self.width + x) * 4 + 2],
            self.buffer[(y * self.width + x) * 4 + 3],
        )
    }
}

impl<'a> FramebufferRead<'a> {
    pub fn read_color(&self) -> ColorData {
        let gl = &self.fbo.ugli.inner.raw;
        // TODO
        // if self.fbo.handle != 0 {
        //     if let ColorAttachmentRead::None = self.color {
        //         panic!("Framebuffer has no color attached");
        //     }
        // }
        self.fbo.bind();
        let result = unsafe {
            let buffer_len = self.size.x * self.size.y * 4;
            let mut buffer = Vec::with_capacity(buffer_len);
            gl.read_pixels(
                0,
                0,
                self.size.x as raw::SizeI,
                self.size.y as raw::SizeI,
                raw::RGBA,
                raw::UNSIGNED_BYTE,
                std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer_len),
            );
            buffer.set_len(buffer_len);
            ColorData {
                width: self.size.x,
                height: self.size.y,
                buffer,
                phantom_data: PhantomData,
            }
        };
        self.fbo.ugli.debug_check();
        result
    }

    pub fn copy_to_texture(
        &self,
        texture: &mut Texture,
        source_rect: Aabb2<usize>,
        dest: vec2<usize>,
    ) {
        let gl = &self.fbo.ugli.inner.raw;
        self.fbo.bind();
        gl.bind_texture(raw::TEXTURE_2D, &texture.handle);
        gl.copy_tex_sub_image_2d(
            raw::TEXTURE_2D,
            0,
            dest.x as raw::Int,
            dest.y as raw::Int,
            source_rect.bottom_left().x as raw::Int,
            source_rect.bottom_left().y as raw::Int,
            source_rect.width() as raw::SizeI,
            source_rect.height() as raw::SizeI,
        );
        self.fbo.ugli.debug_check();
    }
}
