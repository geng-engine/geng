pub type BitField = gl::types::GLbitfield;
pub type Bool = gl::types::GLboolean;
pub type Char = gl::types::GLchar;
pub type ClampedFloat = gl::types::GLclampf;
pub type Enum = gl::types::GLenum;
pub type Float = gl::types::GLfloat;
pub type Int = gl::types::GLint;
pub type IntPtr = gl::types::GLintptr;
pub type SizeI = gl::types::GLsizei;
pub type UByte = gl::types::GLubyte;
pub type UInt = gl::types::GLuint;
pub type SizeIPtr = gl::types::GLsizeiptr;

pub struct Context {}

impl Context {
    pub fn new<F: Fn(&str) -> *const std::os::raw::c_void>(get_proc_address: F) -> Self {
        gl::load_with(get_proc_address);
        Self {}
    }
}

mod buffer;
mod constants;
mod draw;
mod framebuffer;
mod program_shader;
mod renderbuffer;
mod state;
mod texture;
mod uniform_attribute;
mod vao;
mod view;

pub use buffer::*;
pub use constants::*;
pub use draw::*;
pub use framebuffer::*;
pub use program_shader::*;
pub use renderbuffer::*;
pub use state::*;
pub use texture::*;
pub use uniform_attribute::*;
pub use vao::*;
pub use view::*;
