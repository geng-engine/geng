use super::*;

mod storage;

pub use storage::*;

pub(crate) static mut UNIFORM_TEXTURE_COUNT: usize = 0; // TODO: multiple contexts, threads?

pub trait Uniform {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo);
}

pub trait UniformVisitor {
    fn visit<U: Uniform>(&mut self, name: &str, uniform: &U);
}

macro_rules! impl_primitive_uniform {
    ($t:ty as $glt:ty: [$f1:ident, $f2:ident, $f3:ident, $f4:ident]) => {
        impl Uniform for $t {
            fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
                gl.$f1(&info.location, *self as $glt);
            }
        }
        impl Uniform for [$t; 2] {
            fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
                gl.$f2(&info.location, self[0] as $glt, self[1] as $glt);
            }
        }
        impl Uniform for [$t; 3] {
            fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
                gl.$f3(
                    &info.location,
                    self[0] as $glt,
                    self[1] as $glt,
                    self[2] as $glt,
                );
            }
        }
        impl Uniform for [$t; 4] {
            fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
                gl.$f4(
                    &info.location,
                    self[0] as $glt,
                    self[1] as $glt,
                    self[2] as $glt,
                    self[3] as $glt,
                );
            }
        }
    };
}

impl_primitive_uniform!(f32 as raw::Float: [uniform_1f, uniform_2f, uniform_3f, uniform_4f]);
impl_primitive_uniform!(f64 as raw::Float: [uniform_1f, uniform_2f, uniform_3f, uniform_4f]);
impl_primitive_uniform!(i8 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(i16 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(i32 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(i64 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(isize as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(u8 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(u16 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(u32 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(u64 as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);
impl_primitive_uniform!(usize as raw::Int: [uniform_1i, uniform_2i, uniform_3i, uniform_4i]);

impl Uniform for [[f32; 2]; 2] {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix2fv(&info.location, 1, raw::FALSE, unsafe {
            mem::transmute::<&Self, &[f32; 2 * 2]>(self)
        });
    }
}

impl Uniform for [[f32; 3]; 3] {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix3fv(&info.location, 1, raw::FALSE, unsafe {
            mem::transmute::<&Self, &[f32; 3 * 3]>(self)
        });
    }
}

impl Uniform for [[f32; 4]; 4] {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix4fv(&info.location, 1, raw::FALSE, unsafe {
            mem::transmute::<&Self, &[f32; 4 * 4]>(self)
        });
    }
}

impl Uniform for mat3<f32> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix3fv(&info.location, 1, raw::FALSE, self.as_flat_array());
    }
}

impl Uniform for mat4<f32> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix4fv(&info.location, 1, raw::FALSE, self.as_flat_array());
    }
}

impl<P: TexturePixel> Uniform for Texture2d<P> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.active_texture(raw::TEXTURE0 + unsafe { UNIFORM_TEXTURE_COUNT } as raw::Enum);
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.uniform_1i(&info.location, unsafe { UNIFORM_TEXTURE_COUNT } as raw::Int);
        unsafe {
            UNIFORM_TEXTURE_COUNT += 1;
        }
    }
}

impl<U: Uniform> Uniform for Option<U> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        if let Some(uniform) = self {
            uniform.apply(gl, info);
        } else if let Some(default) = &info.default {
            log::warn!("{:?} reset to {:?}", info.name, default);
            default.apply(gl, info);
        } else {
            panic!("Optional uniform with unknown default");
        }
    }
}

impl<'a, U: Uniform> Uniform for &'a U {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        U::apply(self, gl, info)
    }
}

impl<'a, U: Uniform> Uniform for Ref<'a, U> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        U::apply(self, gl, info)
    }
}

impl<U> Uniform for vec2<U>
where
    [U; 2]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 2]>::apply(self, gl, info)
    }
}

impl<U> Uniform for vec3<U>
where
    [U; 3]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 3]>::apply(self, gl, info)
    }
}

impl<U> Uniform for vec4<U>
where
    [U; 4]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 4]>::apply(self, gl, info)
    }
}

impl Uniform for Rgba<f32> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[f32; 4]>::apply(self, gl, info)
    }
}

#[derive(Debug)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),
    // TODO: Bool(bool),
    // TODO: BVec2([bool; 2]),
    // TODO: BVec3([bool; 3]),
    // TODO: BVec4([bool; 4]),
    Mat2([[f32; 2]; 2]),
    Mat3([[f32; 3]; 3]),
    Mat4([[f32; 4]; 4]),
    // TODO: Sampler2d,
    // TODO: SamplerCube
}

impl Uniform for UniformValue {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        match self {
            UniformValue::Float(value) => value.apply(gl, info),
            UniformValue::Vec2(value) => value.apply(gl, info),
            UniformValue::Vec3(value) => value.apply(gl, info),
            UniformValue::Vec4(value) => value.apply(gl, info),
            UniformValue::Int(value) => value.apply(gl, info),
            UniformValue::IVec2(value) => value.apply(gl, info),
            UniformValue::IVec3(value) => value.apply(gl, info),
            UniformValue::IVec4(value) => value.apply(gl, info),
            UniformValue::Mat2(value) => value.apply(gl, info),
            UniformValue::Mat3(value) => value.apply(gl, info),
            UniformValue::Mat4(value) => value.apply(gl, info),
        }
    }
}

impl UniformValue {
    pub(crate) fn get_value(
        gl: &raw::Context,
        program: &raw::Program,
        location: &raw::UniformLocation,
        info: &raw::ActiveInfo,
    ) -> Option<Self> {
        Some(match info.typ {
            raw::FLOAT => Self::Float({
                let mut values = [0.0];
                gl.get_uniform_float(program, location, &mut values);
                values[0]
            }),
            raw::FLOAT_VEC2 => Self::Vec2({
                let mut values = [0.0; 2];
                gl.get_uniform_float(program, location, &mut values);
                values
            }),
            raw::FLOAT_VEC3 => Self::Vec3({
                let mut values = [0.0; 3];
                gl.get_uniform_float(program, location, &mut values);
                values
            }),
            raw::FLOAT_VEC4 => Self::Vec4({
                let mut values = [0.0; 4];
                gl.get_uniform_float(program, location, &mut values);
                values
            }),
            raw::INT => Self::Int({
                let mut values = [0];
                gl.get_uniform_int(program, location, &mut values);
                values[0]
            }),
            raw::INT_VEC2 => Self::IVec2({
                let mut values = [0; 2];
                gl.get_uniform_int(program, location, &mut values);
                values
            }),
            raw::INT_VEC3 => Self::IVec3({
                let mut values = [0; 3];
                gl.get_uniform_int(program, location, &mut values);
                values
            }),
            raw::INT_VEC4 => Self::IVec4({
                let mut values = [0; 4];
                gl.get_uniform_int(program, location, &mut values);
                values
            }),
            raw::FLOAT_MAT2 => Self::Mat2({
                let mut values = [[0.0f32; 2]; 2];
                gl.get_uniform_float(program, location, unsafe {
                    mem::transmute::<&mut [[f32; 2]; 2], &mut [f32; 2 * 2]>(&mut values)
                });
                values
            }),
            raw::FLOAT_MAT3 => Self::Mat3({
                let mut values = [[0.0f32; 3]; 3];
                gl.get_uniform_float(program, location, unsafe {
                    mem::transmute::<&mut [[f32; 3]; 3], &mut [f32; 3 * 3]>(&mut values)
                });
                values
            }),
            raw::FLOAT_MAT4 => Self::Mat4({
                let mut values = [[0.0f32; 4]; 4];
                gl.get_uniform_float(program, location, unsafe {
                    mem::transmute::<&mut [[f32; 4]; 4], &mut [f32; 4 * 4]>(&mut values)
                });
                values
            }),
            _ => return None,
        })
    }
}
