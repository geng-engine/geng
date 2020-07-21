use super::*;

pub type Texture = web_sys::WebGlTexture;

impl Context {
    pub fn active_texture(&self, texture: Enum) {
        self.inner.active_texture(texture);
    }

    pub fn bind_texture(&self, target: Enum, texture: &Texture) {
        self.inner.bind_texture(target, Some(texture));
    }

    pub fn create_texture(&self) -> Option<Texture> {
        self.inner.create_texture()
    }

    pub fn delete_texture(&self, texture: &Texture) {
        self.inner.delete_texture(Some(texture));
    }

    pub fn generate_mipmap(&self, target: Enum) {
        self.inner.generate_mipmap(target);
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
        self.inner
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                typ,
                pixels.map(|data| unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        std::mem::size_of_val(data),
                    )
                }),
            )
            .unwrap();
    }

    pub fn tex_image_2d_image(
        &self,
        target: Enum,
        level: Int,
        internal_format: Int,
        format: Enum,
        typ: Enum,
        source: &web_sys::HtmlImageElement,
    ) {
        self.inner
            .tex_image_2d_with_u32_and_u32_and_image(
                target,
                level,
                internal_format,
                format,
                typ,
                source,
            )
            .unwrap();
    }

    pub fn tex_parameteri(&self, target: Enum, pname: Enum, param: Int) {
        self.inner.tex_parameteri(target, pname, param);
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
        self.inner
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                target,
                level,
                x_offset,
                y_offset,
                width,
                height,
                format,
                typ,
                Some(unsafe {
                    std::slice::from_raw_parts(
                        pixels.as_ptr() as *const u8,
                        std::mem::size_of_val(pixels),
                    )
                }),
            )
            .unwrap();
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
        self.inner
            .copy_tex_sub_image_2d(target, level, x_offset, y_offset, x, y, width, height);
    }
}
