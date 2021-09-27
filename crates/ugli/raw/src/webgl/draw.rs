use super::*;

impl Context {
    pub fn clear(&self, mask: BitField) {
        self.inner.clear(mask);
    }

    pub fn draw_arrays(&self, mode: Enum, first: Int, count: SizeI) {
        self.inner.draw_arrays(mode, first, count);
    }

    pub fn draw_arrays_instanced(&self, mode: Enum, first: Int, count: SizeI, primcount: SizeI) {
        self.angle_instanced_arrays
            .draw_arrays_instanced_angle(mode, first, count, primcount);
    }
}
