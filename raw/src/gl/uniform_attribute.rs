use super::*;

pub type UniformLocation = gl::types::GLint;

#[derive(Debug)]
pub struct ActiveInfo {
    pub name: String,
    pub size: Int,
    pub typ: Enum,
}

impl Context {
    pub fn disable_vertex_attrib_array(&self, index: UInt) {
        unsafe {
            gl::DisableVertexAttribArray(index);
        }
    }

    pub fn enable_vertex_attrib_array(&self, index: UInt) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    pub fn get_active_attrib(&self, program: &Program, index: UInt) -> ActiveInfo {
        let mut max_length = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetProgramiv(
                *program,
                gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
                max_length.as_mut_ptr(),
            );
        }
        let max_length = unsafe { max_length.assume_init() } as usize;
        let mut buf = vec![std::mem::MaybeUninit::<u8>::uninit(); max_length];
        let mut length = std::mem::MaybeUninit::uninit();
        let mut size = std::mem::MaybeUninit::uninit();
        let mut typ = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetActiveAttrib(
                *program,
                index,
                buf.len() as SizeI,
                length.as_mut_ptr(),
                size.as_mut_ptr(),
                typ.as_mut_ptr(),
                buf.as_mut_ptr() as *mut Char,
            );
        }
        let length = unsafe { length.assume_init() } as usize;
        let size = unsafe { size.assume_init() };
        let typ = unsafe { typ.assume_init() };
        buf.truncate(length);
        let name = String::from_utf8(unsafe {
            #[allow(clippy::unsound_collection_transmute)]
            std::mem::transmute(buf)
        })
        .unwrap();
        ActiveInfo { name, size, typ }
    }

    pub fn get_active_uniform(&self, program: &Program, index: UInt) -> ActiveInfo {
        let mut max_length = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetProgramiv(
                *program,
                gl::ACTIVE_UNIFORM_MAX_LENGTH,
                max_length.as_mut_ptr(),
            );
        }
        let max_length = unsafe { max_length.assume_init() } as usize;
        let mut buf = vec![std::mem::MaybeUninit::<u8>::uninit(); max_length];
        let mut length = std::mem::MaybeUninit::uninit();
        let mut size = std::mem::MaybeUninit::uninit();
        let mut typ = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetActiveUniform(
                *program,
                index,
                buf.len() as SizeI,
                length.as_mut_ptr(),
                size.as_mut_ptr(),
                typ.as_mut_ptr(),
                buf.as_mut_ptr() as *mut Char,
            );
        }
        let length = unsafe { length.assume_init() } as usize;
        let size = unsafe { size.assume_init() };
        let typ = unsafe { typ.assume_init() };
        buf.truncate(length);
        let name = String::from_utf8(unsafe {
            #[allow(clippy::unsound_collection_transmute)]
            std::mem::transmute(buf)
        })
        .unwrap();
        ActiveInfo { name, size, typ }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Int {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { gl::GetAttribLocation(*program, name.as_ptr()) }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation> {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(*program, name.as_ptr());
            if location < 0 {
                None
            } else {
                Some(location)
            }
        }
    }

    pub fn uniform_1i(&self, location: &UniformLocation, v: Int) {
        unsafe {
            gl::Uniform1i(*location, v);
        }
    }

    pub fn uniform_1f(&self, location: &UniformLocation, v: Float) {
        unsafe {
            gl::Uniform1f(*location, v);
        }
    }

    pub fn uniform_2i(&self, location: &UniformLocation, v0: Int, v1: Int) {
        unsafe {
            gl::Uniform2i(*location, v0, v1);
        }
    }

    pub fn uniform_2f(&self, location: &UniformLocation, v0: Float, v1: Float) {
        unsafe {
            gl::Uniform2f(*location, v0, v1);
        }
    }

    pub fn uniform_3i(&self, location: &UniformLocation, v0: Int, v1: Int, v2: Int) {
        unsafe {
            gl::Uniform3i(*location, v0, v1, v2);
        }
    }

    pub fn uniform_3f(&self, location: &UniformLocation, v0: Float, v1: Float, v2: Float) {
        unsafe {
            gl::Uniform3f(*location, v0, v1, v2);
        }
    }

    pub fn uniform_4i(&self, location: &UniformLocation, v0: Int, v1: Int, v2: Int, v3: Int) {
        unsafe {
            gl::Uniform4i(*location, v0, v1, v2, v3);
        }
    }

    pub fn uniform_4f(
        &self,
        location: &UniformLocation,
        v0: Float,
        v1: Float,
        v2: Float,
        v3: Float,
    ) {
        unsafe {
            gl::Uniform4f(*location, v0, v1, v2, v3);
        }
    }

    pub fn uniform_matrix4fv(
        &self,
        location: &UniformLocation,
        count: SizeI,
        transpose: Bool,
        v: &[Float],
    ) {
        debug_assert_eq!(v.len(), count as usize * 4 * 4);
        unsafe {
            gl::UniformMatrix4fv(*location, count, transpose, v.as_ptr());
        }
    }

    pub fn vertex_attrib_divisor(&self, index: UInt, divisor: UInt) {
        unsafe {
            gl::VertexAttribDivisor(index, divisor);
        }
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
        unsafe {
            gl::VertexAttribPointer(index, size, typ, normalized, stride, offset as _);
        }
    }
}
