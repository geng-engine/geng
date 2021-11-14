use super::*;

pub struct Fbo {
    pub(crate) ugli: Ugli,
    pub(crate) handle: Option<raw::Framebuffer>,
    phantom_data: PhantomData<*mut ()>,
}

impl Fbo {
    pub fn new(ugli: &Ugli) -> Self {
        let gl = &ugli.inner.raw;
        Self {
            ugli: ugli.clone(),
            handle: Some(gl.create_framebuffer().unwrap()),
            phantom_data: PhantomData,
        }
    }
    pub fn default(ugli: &Ugli) -> Self {
        Self {
            ugli: ugli.clone(),
            handle: None,
            phantom_data: PhantomData,
        }
    }
    pub fn bind(&self) {
        let gl = &self.ugli.inner.raw;
        gl.bind_framebuffer(raw::FRAMEBUFFER, self.handle.as_ref());
        self.ugli.debug_check();
    }
    pub fn check(&self) {
        let gl = &self.ugli.inner.raw;
        // TODO: text instead of raw codes
        assert_eq!(
            gl.check_framebuffer_status(raw::FRAMEBUFFER),
            raw::FRAMEBUFFER_COMPLETE,
            "Framebuffer check failed"
        );
    }
}

impl Drop for Fbo {
    fn drop(&mut self) {
        let gl = &self.ugli.inner.raw;
        if let Some(ref handle) = self.handle {
            gl.delete_framebuffer(handle);
        }
    }
}
