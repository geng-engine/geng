use super::*;

pub type Buffer = gl::types::GLuint;

impl Context {
    pub fn bind_buffer(&self, target: Enum, buffer: &Buffer) {
        unsafe {
            gl::BindBuffer(target, *buffer);
        }
    }

    pub fn buffer_data<T>(&self, target: Enum, data: &[T], usage: Enum) {
        unsafe {
            gl::BufferData(
                target,
                std::mem::size_of_val(data) as SizeIPtr,
                data.as_ptr() as _,
                usage,
            );
        }
    }

    pub fn buffer_sub_data<T>(&self, target: Enum, offset: IntPtr, data: &[T]) {
        unsafe {
            gl::BufferSubData(
                target,
                offset,
                std::mem::size_of_val(data) as SizeIPtr,
                data.as_ptr() as _,
            );
        }
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GenBuffers(1, handle.as_mut_ptr());
        }
        let handle = unsafe { handle.assume_init() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        unsafe {
            gl::DeleteBuffers(1, buffer);
        }
    }
}
