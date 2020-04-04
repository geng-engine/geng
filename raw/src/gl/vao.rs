use crate::*;

pub type VertexArrayObject = gl::types::GLuint;

impl Context {
    pub fn bind_vertex_array(&self, vao: &VertexArrayObject) {
        unsafe {
            gl::BindVertexArray(*vao);
        }
    }

    pub fn create_vertex_array(&self) -> Option<VertexArrayObject> {
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GenVertexArrays(1, handle.as_mut_ptr());
        }
        let handle = unsafe { handle.assume_init() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_vertex_array(&self, vao: &VertexArrayObject) {
        unsafe {
            gl::DeleteVertexArrays(1, vao);
        }
    }
}
