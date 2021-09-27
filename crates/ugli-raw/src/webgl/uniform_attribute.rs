use super::*;

pub type UniformLocation = web_sys::WebGlUniformLocation;

#[derive(Debug)]
pub struct ActiveInfo {
    pub name: String,
    pub size: Int,
    pub typ: Enum,
}

impl ActiveInfo {
    fn from(info: web_sys::WebGlActiveInfo) -> Self {
        Self {
            name: info.name(),
            size: info.size(),
            typ: info.type_(),
        }
    }
}

impl Context {
    pub fn disable_vertex_attrib_array(&self, index: UInt) {
        self.inner.disable_vertex_attrib_array(index);
    }

    pub fn enable_vertex_attrib_array(&self, index: UInt) {
        self.inner.enable_vertex_attrib_array(index);
    }

    pub fn get_active_attrib(&self, program: &Program, index: UInt) -> ActiveInfo {
        ActiveInfo::from(self.inner.get_active_attrib(program, index).unwrap())
    }

    pub fn get_active_uniform(&self, program: &Program, index: UInt) -> ActiveInfo {
        ActiveInfo::from(self.inner.get_active_uniform(program, index).unwrap())
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Int {
        self.inner.get_attrib_location(program, name)
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation> {
        self.inner.get_uniform_location(program, name)
    }

    pub fn uniform_1i(&self, location: &UniformLocation, v: Int) {
        self.inner.uniform1i(Some(location), v);
    }

    pub fn uniform_1f(&self, location: &UniformLocation, v: Float) {
        self.inner.uniform1f(Some(location), v);
    }

    pub fn uniform_2i(&self, location: &UniformLocation, v0: Int, v1: Int) {
        self.inner.uniform2i(Some(location), v0, v1);
    }

    pub fn uniform_2f(&self, location: &UniformLocation, v0: Float, v1: Float) {
        self.inner.uniform2f(Some(location), v0, v1);
    }

    pub fn uniform_3i(&self, location: &UniformLocation, v0: Int, v1: Int, v2: Int) {
        self.inner.uniform3i(Some(location), v0, v1, v2);
    }

    pub fn uniform_3f(&self, location: &UniformLocation, v0: Float, v1: Float, v2: Float) {
        self.inner.uniform3f(Some(location), v0, v1, v2);
    }

    pub fn uniform_4i(&self, location: &UniformLocation, v0: Int, v1: Int, v2: Int, v3: Int) {
        self.inner.uniform4i(Some(location), v0, v1, v2, v3);
    }

    pub fn uniform_4f(
        &self,
        location: &UniformLocation,
        v0: Float,
        v1: Float,
        v2: Float,
        v3: Float,
    ) {
        self.inner.uniform4f(Some(location), v0, v1, v2, v3);
    }

    pub fn uniform_matrix3fv(
        &self,
        location: &UniformLocation,
        count: SizeI,
        transpose: Bool,
        v: &[Float],
    ) {
        debug_assert_eq!(v.len(), count as usize * 3 * 3);
        self.inner
            .uniform_matrix3fv_with_f32_array(Some(location), transpose, v);
    }

    pub fn uniform_matrix4fv(
        &self,
        location: &UniformLocation,
        count: SizeI,
        transpose: Bool,
        v: &[Float],
    ) {
        debug_assert_eq!(v.len(), count as usize * 4 * 4);
        self.inner
            .uniform_matrix4fv_with_f32_array(Some(location), transpose, v);
    }

    pub fn vertex_attrib_divisor(&self, index: UInt, divisor: UInt) {
        self.angle_instanced_arrays
            .vertex_attrib_divisor_angle(index, divisor);
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: UInt,
        size: Int,
        typ: Enum,
        normalized: Bool,
        stride: SizeI,
        offset: IntPtr,
    ) {
        self.inner
            .vertex_attrib_pointer_with_i32(index, size, typ, normalized, stride, offset);
    }
}
