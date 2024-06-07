use super::*;

pub(crate) struct Vao {
    #[cfg(not(target_arch = "wasm32"))]
    pub handle: raw::VertexArrayObject,
    pub ugli: Ugli,
}

impl Vao {
    pub fn new(ugli: &Ugli) -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            handle: ugli.inner.raw.create_vertex_array().unwrap(),
            ugli: ugli.clone(),
        }
    }
    pub fn bind(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.ugli.inner.raw.bind_vertex_array(&self.handle);
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.ugli.inner.raw.delete_vertex_array(&self.handle);
    }
}
