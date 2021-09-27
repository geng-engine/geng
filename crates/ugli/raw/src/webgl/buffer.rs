use super::*;

pub type Buffer = web_sys::WebGlBuffer;

impl Context {
    pub fn bind_buffer(&self, target: Enum, buffer: &Buffer) {
        self.inner.bind_buffer(target, Some(buffer));
    }

    pub fn buffer_data<T>(&self, target: Enum, data: &[T], usage: Enum) {
        self.inner.buffer_data_with_u8_array(
            target,
            unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, std::mem::size_of_val(data))
            },
            usage,
        );
    }

    pub fn buffer_sub_data<T>(&self, target: Enum, offset: IntPtr, data: &[T]) {
        self.inner
            .buffer_sub_data_with_i32_and_u8_array(target, offset, unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, std::mem::size_of_val(data))
            });
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        self.inner.create_buffer()
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        self.inner.delete_buffer(Some(buffer));
    }
}
