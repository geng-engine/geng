use crate::*;

pub struct FBO {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: Option<ugl::Framebuffer>,
    phantom_data: PhantomData<*mut ()>,
}

impl FBO {
    pub fn new(ugli: &Rc<Ugli>) -> Self {
        let gl = &ugli.inner;
        Self {
            ugli: ugli.clone(),
            handle: Some(gl.create_framebuffer().unwrap()),
            phantom_data: PhantomData,
        }
    }
    pub fn default(ugli: &Rc<Ugli>) -> Self {
        Self {
            ugli: ugli.clone(),
            handle: None,
            phantom_data: PhantomData,
        }
    }
    pub fn bind(&self) {
        let gl = &self.ugli.inner;
        gl.bind_framebuffer(ugl::FRAMEBUFFER, self.handle.as_ref());
        self.ugli.debug_check();
    }
    pub fn check(&self) {
        let gl = &self.ugli.inner;
        // TODO: text instead of raw codes
        assert_eq!(
            gl.check_framebuffer_status(ugl::FRAMEBUFFER),
            ugl::FRAMEBUFFER_COMPLETE,
            "Framebuffer check failed"
        );
    }
}

impl Drop for FBO {
    fn drop(&mut self) {
        let gl = &self.ugli.inner;
        if let Some(ref handle) = self.handle {
            gl.delete_framebuffer(handle);
        }
    }
}
