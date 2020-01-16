use crate::*;

pub struct ColorData<'a> {
    width: usize,
    height: usize,
    buffer: Vec<ugl::UByte>,
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> ColorData<'a> {
    pub fn get(&self, x: usize, y: usize) -> Color<u8> {
        assert!(x < self.width);
        assert!(y < self.height);
        Color::rgba(
            self.buffer[(y * self.width + x) * 4],
            self.buffer[(y * self.width + x) * 4 + 1],
            self.buffer[(y * self.width + x) * 4 + 2],
            self.buffer[(y * self.width + x) * 4 + 3],
        )
    }
}

impl<'a> FramebufferRead<'a> {
    pub fn read_color(&self) -> ColorData {
        let gl = &self.fbo.ugli.inner;
        // TODO
        // if self.fbo.handle != 0 {
        //     if let ColorAttachmentRead::None = self.color {
        //         panic!("Framebuffer has no color attached");
        //     }
        // }
        self.fbo.bind();
        let result = unsafe {
            let mut buffer =
                vec![mem::uninitialized::<ugl::UByte>(); self.size.x * self.size.y * 4];
            gl.read_pixels(
                0,
                0,
                self.size.x as ugl::SizeI,
                self.size.y as ugl::SizeI,
                ugl::RGBA,
                ugl::UNSIGNED_BYTE,
                &mut buffer,
            );
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
        source_rect: AABB<usize>,
        dest: Vec2<usize>,
    ) {
        let gl = &self.fbo.ugli.inner;
        self.fbo.bind();
        gl.bind_texture(ugl::TEXTURE_2D, &texture.handle);
        gl.copy_tex_sub_image_2d(
            ugl::TEXTURE_2D,
            0,
            dest.x as ugl::Int,
            dest.y as ugl::Int,
            source_rect.bottom_left().x as ugl::Int,
            source_rect.bottom_left().y as ugl::Int,
            source_rect.width() as ugl::SizeI,
            source_rect.height() as ugl::SizeI,
        );
        self.fbo.ugli.debug_check();
    }
}
