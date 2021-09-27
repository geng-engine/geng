use super::*;

pub type Renderbuffer = web_sys::WebGlRenderbuffer;

impl Context {
    pub fn bind_renderbuffer(&self, target: Enum, renderbuffer: &Renderbuffer) {
        self.inner.bind_renderbuffer(target, Some(renderbuffer));
    }

    pub fn create_renderbuffer(&self) -> Option<Renderbuffer> {
        self.inner.create_renderbuffer()
    }

    pub fn delete_renderbuffer(&self, renderbuffer: &Renderbuffer) {
        self.inner.delete_renderbuffer(Some(renderbuffer));
    }

    pub fn renderbuffer_storage(
        &self,
        target: Enum,
        internal_format: Enum,
        width: SizeI,
        height: SizeI,
    ) {
        self.inner
            .renderbuffer_storage(target, internal_format, width, height);
    }
}
