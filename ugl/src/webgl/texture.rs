use super::*;

pub type Texture = webgl::WebGLTexture;

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
        match pixels {
            Some(pixels) => {
                js! {
                    @(no_return)
                    @{&self.inner}.texImage2D(
                        @{target},
                        @{level},
                        @{internal_format},
                        @{width},
                        @{height},
                        @{border},
                        @{format},
                        @{typ},
                        @{as_typed_array(pixels)});
                }
            }
            None => {
                js! {
                    @(no_return)
                    @{&self.inner}.texImage2D(
                        @{target},
                        @{level},
                        @{internal_format},
                        @{width},
                        @{height},
                        @{border},
                        @{format},
                        @{typ},
                        null);
                }
            }
        }
    }

    pub fn tex_image_2d_src<S: stdweb::JsSerialize>(
        &self,
        target: Enum,
        level: Int,
        internal_format: Int,
        format: Enum,
        typ: Enum,
        source: S,
    ) {
        self.inner
            .tex_image2_d_1(target, level, internal_format, format, typ, source);
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
        js! {
            @(no_return)
            @{&self.inner}.texSubImage2D(
                @{target},
                @{level},
                @{x_offset},
                @{y_offset},
                @{width},
                @{height},
                @{format},
                @{typ},
                @{as_typed_array(pixels)});
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
        self.inner
            .copy_tex_sub_image2_d(target, level, x_offset, y_offset, x, y, width, height);
    }
}
