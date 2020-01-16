use super::*;

impl Context {
    pub fn clear(&self, mask: BitField) {
        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn draw_arrays(&self, mode: Enum, first: Int, count: SizeI) {
        unsafe {
            gl::DrawArrays(mode, first, count);
        }
    }

    pub fn draw_arrays_instanced(&self, mode: Enum, first: Int, count: SizeI, primcount: SizeI) {
        unsafe {
            gl::DrawArraysInstanced(mode, first, count, primcount);
        }
    }
}
