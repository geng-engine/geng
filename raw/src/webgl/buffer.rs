use super::*;

pub type Buffer = webgl::WebGLBuffer;

impl Context {
    pub fn bind_buffer(&self, target: Enum, buffer: &Buffer) {
        self.inner.bind_buffer(target, Some(buffer));
    }

    pub fn buffer_data<T>(&self, target: Enum, data: &[T], usage: Enum) {
        js! {
            @(no_return)
            @{&self.inner}.bufferData(@{target}, @{as_typed_array(data)}, @{usage});
        }
    }

    pub fn buffer_sub_data<T>(&self, target: Enum, offset: IntPtr, data: &[T]) {
        js! {
            @(no_return)
            @{&self.inner}.bufferSubData(@{target}, @{offset as i32}, @{as_typed_array(data)});
        }
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        self.inner.create_buffer()
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        self.inner.delete_buffer(Some(buffer));
    }
}
