use super::*;

mod storage;

pub use storage::*;

pub(crate) static mut UNIFORM_TEXTURE_COUNT: usize = 0;

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

impl Uniform for Mat3<f32> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        gl.uniform_matrix3fv(&info.location, 1, raw::FALSE, self.as_flat_array());
    }
}

impl Uniform for Mat4<f32> {
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

impl<U> Uniform for Vec2<U>
where
    [U; 2]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 2]>::apply(self, gl, info)
    }
}

impl<U> Uniform for Vec3<U>
where
    [U; 3]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 3]>::apply(self, gl, info)
    }
}

impl<U> Uniform for Vec4<U>
where
    [U; 4]: Uniform,
{
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[U; 4]>::apply(self, gl, info)
    }
}

impl Uniform for Color<f32> {
    fn apply(&self, gl: &raw::Context, info: &UniformInfo) {
        <[f32; 4]>::apply(self, gl, info)
    }
}
