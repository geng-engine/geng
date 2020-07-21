use super::*;

pub type Framebuffer = web_sys::WebGlFramebuffer;

impl Context {
    pub fn bind_framebuffer(&self, target: Enum, framebuffer: Option<&Framebuffer>) {
        self.inner.bind_framebuffer(target, framebuffer);
    }

    pub fn check_framebuffer_status(&self, target: Enum) -> Enum {
        self.inner.check_framebuffer_status(target)
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer> {
        self.inner.create_framebuffer()
    }

    pub fn delete_framebuffer(&self, framebuffer: &Framebuffer) {
        self.inner.delete_framebuffer(Some(framebuffer));
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: Enum,
        attachment: Enum,
        renderbuffer_target: Enum,
        renderbuffer: Option<&Renderbuffer>,
    ) {
        self.inner
            .framebuffer_renderbuffer(target, attachment, renderbuffer_target, renderbuffer);
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: Enum,
        attachment: Enum,
        texture_target: Enum,
        texture: Option<&Texture>,
        level: Int,
    ) {
        self.inner
            .framebuffer_texture_2d(target, attachment, texture_target, texture, level);
    }

    pub fn read_pixels<T>(
        &self,
        x: Int,
        y: Int,
        width: SizeI,
        height: SizeI,
        format: Enum,
        typ: Enum,
        pixels: &mut [T],
    ) {
        self.inner
            .read_pixels_with_opt_u8_array(
                x,
                y,
                width,
                height,
                format,
                typ,
                Some(unsafe {
                    std::slice::from_raw_parts_mut(
                        pixels.as_mut_ptr() as *mut u8,
                        std::mem::size_of_val(pixels),
                    )
                }),
            )
            .unwrap();
    }
}
