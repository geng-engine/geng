use super::*;

/// # Safety
/// Don't implement yourself
pub unsafe trait RenderbufferPixel {
    const GL_FORMAT: raw::Enum;
}

unsafe impl RenderbufferPixel for Rgba<f32> {
    const GL_FORMAT: raw::Enum = raw::RGBA;
}

unsafe impl RenderbufferPixel for DepthComponent {
    const GL_FORMAT: raw::Enum = raw::DEPTH_COMPONENT;
}

pub struct DepthStencilValue;

unsafe impl RenderbufferPixel for DepthStencilValue {
    const GL_FORMAT: raw::Enum = raw::DEPTH_STENCIL;
}

pub struct Renderbuffer<T: RenderbufferPixel = Rgba<f32>> {
    pub(crate) ugli: Ugli,
    pub(crate) handle: raw::Renderbuffer,
    phantom_data: PhantomData<*mut T>,
}

impl<T: RenderbufferPixel> Drop for Renderbuffer<T> {
    fn drop(&mut self) {
        let gl = &self.ugli.inner.raw;
        gl.delete_renderbuffer(&self.handle);
    }
}

impl<T: RenderbufferPixel> Renderbuffer<T> {
    pub fn new(ugli: &Ugli, size: vec2<usize>) -> Self {
        let gl = &ugli.inner.raw;
        let handle = gl.create_renderbuffer().unwrap();
        gl.bind_renderbuffer(raw::RENDERBUFFER, &handle);
        gl.renderbuffer_storage(
            raw::RENDERBUFFER,
            T::GL_FORMAT,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
        );
        ugli.debug_check();
        Self {
            ugli: ugli.clone(),
            handle,
            phantom_data: PhantomData,
        }
    }
}
