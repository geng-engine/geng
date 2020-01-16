use super::*;

pub type Texture = gl::types::GLuint;

impl Context {
    pub fn active_texture(&self, texture: Enum) {
        unsafe {
            gl::ActiveTexture(texture);
        }
    }

    pub fn bind_texture(&self, target: Enum, texture: &Texture) {
        unsafe {
            gl::BindTexture(target, *texture);
        }
    }

    pub fn create_texture(&self) -> Option<Texture> {
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GenTextures(1, handle.as_mut_ptr());
        }
        let handle = unsafe { handle.assume_init() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_texture(&self, texture: &Texture) {
        unsafe {
            gl::DeleteTextures(1, texture);
        }
    }

    pub fn generate_mipmap(&self, target: Enum) {
        unsafe {
            gl::GenerateMipmap(target);
        }
    }

    pub fn tex_image_2d<T>(
        &self,
        target: Enum,
        level: Int,
        internal_format: Int,
        width: SizeI,
        height: SizeI,
        border: Int,
        format: Enum,
        typ: Enum,
        pixels: Option<&[T]>,
    ) {
        unsafe {
            gl::TexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                typ,
                match pixels {
                    None => std::ptr::null(),
                    Some(pixels) => pixels.as_ptr() as _,
                },
            );
        }
    }

    pub fn tex_parameteri(&self, target: Enum, pname: Enum, param: Int) {
        unsafe {
            gl::TexParameteri(target, pname, param);
        }
    }

    pub fn tex_sub_image_2d<T>(
        &self,
        target: Enum,
        level: Int,
        x_offset: Int,
        y_offset: Int,
        width: SizeI,
        height: SizeI,
        format: Enum,
        typ: Enum,
        pixels: &[T],
    ) {
        unsafe {
            gl::TexSubImage2D(
                target,
                level,
                x_offset,
                y_offset,
                width,
                height,
                format,
                typ,
                pixels.as_ptr() as _,
            );
        }
    }

    pub fn copy_tex_sub_image_2d(
        &self,
        target: Enum,
        level: Int,
        x_offset: Int,
        y_offset: Int,
        x: Int,
        y: Int,
        width: SizeI,
        height: SizeI,
    ) {
        unsafe {
            gl::CopyTexSubImage2D(target, level, x_offset, y_offset, x, y, width, height);
        }
    }
}
