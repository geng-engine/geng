use super::*;

impl Context {
    pub fn viewport(&self, x: Int, y: Int, width: SizeI, height: SizeI) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
}
