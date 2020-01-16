use std;
use stdweb;
use webgl;

pub type BitField = webgl::GLbitfield;
pub type Bool = webgl::GLboolean;
pub type ClampedFloat = webgl::GLclampf;
pub type Enum = webgl::GLenum;
pub type Float = webgl::GLfloat;
pub type Int = webgl::GLint;
pub type IntPtr = webgl::GLintptr;
pub type SizeI = webgl::GLsizei;
pub type UByte = webgl::GLubyte;
pub type UInt = webgl::GLuint;
pub type SizeIPtr = webgl::GLsizeiptr;

pub struct Context {
    inner: webgl::WebGLRenderingContext,
    // TODO: use ANGLE_instanced_arrays type directly:
    //   https://github.com/brendanzab/gl-rs/pull/462
    angle_instanced_arrays: Option<stdweb::Reference>,
}

impl Context {
    pub fn new(webgl: webgl::WebGLRenderingContext) -> Self {
        use stdweb::unstable::TryInto;
        // let angle_instanced_arrays = webgl.get_extension();
        let angle_instanced_arrays =
            js! { return @{&webgl}.getExtension("ANGLE_instanced_arrays"); }
                .try_into()
                .ok();
        Self {
            inner: webgl,
            angle_instanced_arrays,
        }
    }

    fn angle_instanced_arrays(&self) -> &stdweb::Reference {
        self.angle_instanced_arrays
            .as_ref()
            .expect("ANGLE_instance_arrays not available")
    }
}

fn as_typed_array<T>(slice: &[T]) -> stdweb::UnsafeTypedArray<u8> {
    unsafe {
        let slice =
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice));
        stdweb::UnsafeTypedArray::new(slice)
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
pub use view::*;
