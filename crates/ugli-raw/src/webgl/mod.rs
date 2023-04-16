pub type BitField = u32;
pub type Bool = bool;
pub type ClampedFloat = f32;
pub type Enum = u32;
pub type Float = f32;
pub type Int = i32;
pub type IntPtr = i32;
pub type SizeI = i32;
pub type UByte = u8;
pub type UInt = u32;
pub type SizeIPtr = i32;

pub struct Context {
    inner: web_sys::WebGlRenderingContext,
    angle_instanced_arrays: web_sys::AngleInstancedArrays,
    #[allow(dead_code)]
    oes_standard_derivatives: web_sys::OesStandardDerivatives,
    #[allow(dead_code)]
    blend_minmax: web_sys::ExtBlendMinmax,
}

impl Context {
    pub fn new(webgl_rendering_context: web_sys::WebGlRenderingContext) -> Self {
        use wasm_bindgen::JsCast;
        let angle_instanced_arrays = webgl_rendering_context
            .get_extension("ANGLE_instanced_arrays")
            .unwrap()
            .expect("ANGLE_instanced_arrays not supported?");
        let oes_standard_derivatives = webgl_rendering_context
            .get_extension("OES_standard_derivatives")
            .unwrap()
            .expect("OES_standard_derivatives not supported?");
        let blend_minmax = webgl_rendering_context
            .get_extension("EXT_blend_minmax")
            .unwrap()
            .expect("EXT_blend_minmax not supported?");
        Self {
            inner: webgl_rendering_context,
            // Unchecked casts here because the type is different in different browsers
            angle_instanced_arrays: angle_instanced_arrays.unchecked_into(),
            oes_standard_derivatives: oes_standard_derivatives.unchecked_into(),
            blend_minmax: blend_minmax.unchecked_into(),
        }
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
