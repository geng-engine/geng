use super::*;

pub type Renderbuffer = gl::types::GLuint;

impl Context {
    pub fn bind_renderbuffer(&self, target: Enum, renderbuffer: &Renderbuffer) {
        unsafe {
            gl::BindRenderbuffer(target, *renderbuffer);
        }
    }

    pub fn create_renderbuffer(&self) -> Option<Renderbuffer> {
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GenRenderbuffers(1, handle.as_mut_ptr());
        }
        let handle = unsafe { handle.assume_init() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_renderbuffer(&self, renderbuffer: &Renderbuffer) {
        unsafe {
            gl::DeleteRenderbuffers(1, renderbuffer);
        }
    }

    pub fn renderbuffer_storage(
        &self,
        target: Enum,
        internal_format: Enum,
        width: SizeI,
        height: SizeI,
    ) {
        unsafe {
            gl::RenderbufferStorage(target, internal_format, width, height);
        }
    }
}
