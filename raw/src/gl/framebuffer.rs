use super::*;

pub type Framebuffer = gl::types::GLuint;

impl Context {
    pub fn bind_framebuffer(&self, target: Enum, framebuffer: Option<&Framebuffer>) {
        unsafe {
            gl::BindFramebuffer(target, *framebuffer.unwrap_or(&0));
        }
    }

    pub fn check_framebuffer_status(&self, target: Enum) -> Enum {
        unsafe { gl::CheckFramebufferStatus(target) }
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer> {
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GenFramebuffers(1, handle.as_mut_ptr());
        }
        let handle = unsafe { handle.assume_init() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_framebuffer(&self, framebuffer: &Framebuffer) {
        unsafe {
            gl::DeleteFramebuffers(1, framebuffer);
        }
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: Enum,
        attachment: Enum,
        renderbuffer_target: Enum,
        renderbuffer: Option<&Renderbuffer>,
    ) {
        unsafe {
            gl::FramebufferRenderbuffer(
                target,
                attachment,
                renderbuffer_target,
                *renderbuffer.unwrap_or(&0),
            );
        }
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: Enum,
        attachment: Enum,
        texture_target: Enum,
        texture: Option<&Texture>,
        level: Int,
    ) {
        unsafe {
            gl::FramebufferTexture2D(
                target,
                attachment,
                texture_target,
                *texture.unwrap_or(&0),
                level,
            );
        }
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
        unsafe {
            gl::ReadPixels(x, y, width, height, format, typ, pixels.as_mut_ptr() as _);
        }
    }
}
