use crate::*;

pub unsafe trait RenderbufferPixel {
    const GL_FORMAT: raw::Enum;
}

unsafe impl RenderbufferPixel for Color<f32> {
    const GL_FORMAT: raw::Enum = raw::RGBA;
}

unsafe impl RenderbufferPixel for DepthComponent {
    const GL_FORMAT: raw::Enum = raw::DEPTH_COMPONENT;
}

pub struct Renderbuffer<T: RenderbufferPixel = Color<f32>> {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: raw::Renderbuffer,
    phantom_data: PhantomData<*mut T>,
}

impl<T: RenderbufferPixel> Drop for Renderbuffer<T> {
    fn drop(&mut self) {
        let gl = &self.ugli.inner;
        gl.delete_renderbuffer(&self.handle);
    }
}

impl<T: RenderbufferPixel> Renderbuffer<T> {
    pub fn new(ugli: &Rc<Ugli>, size: Vec2<usize>) -> Self {
        let gl = &ugli.inner;
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
