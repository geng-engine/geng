use super::*;

impl Context {
    pub fn viewport(&self, x: Int, y: Int, width: SizeI, height: SizeI) {
        self.inner.viewport(x, y, width, height);
    }
}
