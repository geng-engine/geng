use super::*;

/// # Safety
/// Don't implement yourself
pub unsafe trait TexturePixel {
    const INTERNAL_FORMAT: raw::Enum;
    const FORMAT: raw::Enum;
    const TYPE: raw::Enum;
}

unsafe impl TexturePixel for Rgba<f32> {
    const INTERNAL_FORMAT: raw::Enum = raw::RGBA;
    const FORMAT: raw::Enum = raw::RGBA;
    const TYPE: raw::Enum = raw::UNSIGNED_BYTE;
}

unsafe impl TexturePixel for u8 {
    #[cfg(target_arch = "wasm32")]
    const INTERNAL_FORMAT: raw::Enum = raw::ALPHA;
    #[cfg(not(target_arch = "wasm32"))]
    const INTERNAL_FORMAT: raw::Enum = raw::RGBA;
    const FORMAT: raw::Enum = raw::ALPHA;
    const TYPE: raw::Enum = raw::UNSIGNED_BYTE;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum WrapMode {
    Repeat = raw::REPEAT as _,
    Clamp = raw::CLAMP_TO_EDGE as _,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Filter {
    Nearest = raw::NEAREST as _,
    Linear = raw::LINEAR as _,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Format {
    R = raw::R8 as _,
    RG = raw::RG as _,
    RGB = raw::RGB as _,
    RGBA = raw::RGBA as _,
    #[cfg(not(target_arch = "wasm32"))]
    R16 = raw::R16 as _,
    #[cfg(not(target_arch = "wasm32"))]
    RG16 = raw::RG16 as _,
    #[cfg(not(target_arch = "wasm32"))]
    RGB16 = raw::RGB16 as _,
    #[cfg(not(target_arch = "wasm32"))]
    RGBA16 = raw::RGBA16 as _,
    RGB32FLOAT = raw::RGB32F as _,
    RGBA32FLOAT = raw::RGBA32F as _,
}

pub enum Type {
    UnsignedByte = raw::UNSIGNED_BYTE as _,
    Byte = raw::BYTE as _,
    UnsignedShort = raw::UNSIGNED_SHORT as _,
    Short = raw::SHORT as _,
    UnsignedInt = raw::UNSIGNED_INT as _,
    Int = raw::INT as _,
    HalfFloat = raw::HALF_FLOAT as _,
    Float = raw::FLOAT as _,
}

pub struct Texture2d<P: TexturePixel> {
    pub(crate) ugli: Ugli,
    pub(crate) handle: raw::Texture,
    size: Cell<vec2<usize>>,
    phantom_data: PhantomData<*mut P>,
}

impl<P: TexturePixel> Drop for Texture2d<P> {
    fn drop(&mut self) {
        let gl = &self.ugli.inner.raw;
        gl.delete_texture(&self.handle);
    }
}

pub type Texture = Texture2d<Rgba<f32>>;

impl<P: TexturePixel> Texture2d<P> {
    fn new_raw(ugli: &Ugli, size: vec2<usize>) -> Self {
        let gl = &ugli.inner.raw;
        let handle = gl.create_texture().unwrap();
        gl.bind_texture(raw::TEXTURE_2D, &handle);
        gl.tex_parameteri(
            raw::TEXTURE_2D,
            raw::TEXTURE_MIN_FILTER,
            raw::LINEAR as raw::Int,
        );
        let mut texture = Self {
            ugli: ugli.clone(),
            handle,
            size: Cell::new(size),
            phantom_data: PhantomData,
        };
        texture.set_filter(Filter::Linear);
        texture.set_wrap_mode(WrapMode::Clamp);
        ugli.debug_check();
        texture
    }

    pub fn is_pot(&self) -> bool {
        let size = self.size.get();
        size.x & (size.x - 1) == 0 && size.y & (size.y - 1) == 0
    }

    pub fn new_uninitialized(ugli: &Ugli, size: vec2<usize>) -> Self {
        let texture = Self::new_raw(ugli, size);
        let gl = &ugli.inner.raw;
        gl.tex_image_2d::<u8>(
            raw::TEXTURE_2D,
            0,
            P::INTERNAL_FORMAT as raw::Int,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
            0,
            P::FORMAT,
            P::TYPE,
            None,
        );
        ugli.debug_check();
        texture
    }

    pub fn from_raw<T>(
        ugli: &Ugli,
        size: vec2<usize>,
        data: &[T],
        format: Format,
        r#type: Type,
        flip_y: bool,
    ) -> Self {
        let texture = Self::new_raw(ugli, size);
        let gl = &ugli.inner.raw;
        gl.pixel_store_flip_y(flip_y);
        gl.tex_image_2d(
            raw::TEXTURE_2D,
            0,
            P::INTERNAL_FORMAT as raw::Int,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
            0,
            format as raw::Enum,
            r#type as raw::Enum,
            Some(data),
        );
        ugli.debug_check();
        texture
    }

    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.set_wrap_mode_separate(wrap_mode, wrap_mode);
    }

    pub fn set_wrap_mode_separate(&mut self, wrap_mode_x: WrapMode, wrap_mode_y: WrapMode) {
        if wrap_mode_x == WrapMode::Repeat || wrap_mode_y == WrapMode::Repeat {
            assert!(
                self.is_pot(),
                "Repeat wrap mode only supported for power of two textures"
            ); // Because of webgl
        }
        let gl = &self.ugli.inner.raw;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(
            raw::TEXTURE_2D,
            raw::TEXTURE_WRAP_S,
            wrap_mode_x as raw::Int,
        );
        gl.tex_parameteri(
            raw::TEXTURE_2D,
            raw::TEXTURE_WRAP_T,
            wrap_mode_y as raw::Int,
        );
        self.ugli.debug_check();
    }

    pub fn set_filter(&mut self, filter: Filter) {
        assert!(self.is_pot() || filter == Filter::Nearest || filter == Filter::Linear);
        let gl = &self.ugli.inner.raw;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(raw::TEXTURE_2D, raw::TEXTURE_MAG_FILTER, filter as raw::Int);
        gl.tex_parameteri(raw::TEXTURE_2D, raw::TEXTURE_MIN_FILTER, filter as raw::Int);
        self.ugli.debug_check();
    }

    pub fn size(&self) -> vec2<usize> {
        self.size.get()
    }

    // TODO: use like Matrix<Color>?
    pub fn sub_image(&mut self, pos: vec2<usize>, size: vec2<usize>, data: &[u8]) {
        assert_eq!(
            size.x
                * size.y
                * match P::FORMAT {
                    raw::RGBA => 4,
                    raw::ALPHA => 1,
                    _ => unreachable!(),
                },
            data.len()
        );
        let gl = &self.ugli.inner.raw;
        gl.pixel_store_flip_y(false);
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.tex_sub_image_2d(
            raw::TEXTURE_2D,
            0,
            pos.x as raw::Int,
            pos.y as raw::Int,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
            P::FORMAT,
            P::TYPE,
            data,
        );
        self.ugli.debug_check();
    }
}

impl Texture {
    pub fn gen_mipmaps(&mut self) {
        assert!(self.is_pot());
        let gl = &self.ugli.inner.raw;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.generate_mipmap(raw::TEXTURE_2D);
        gl.tex_parameteri(
            raw::TEXTURE_2D,
            raw::TEXTURE_MIN_FILTER,
            raw::LINEAR_MIPMAP_LINEAR as raw::Int,
        );
        self.ugli.debug_check();
    }

    pub fn new_with<F: FnMut(vec2<usize>) -> Rgba<f32>>(
        ugli: &Ugli,
        size: vec2<usize>,
        mut f: F,
    ) -> Self {
        let mut data: Vec<u8> = Vec::with_capacity(size.x * size.y * 4);
        for y in 0..size.y {
            for x in 0..size.x {
                let color = f(vec2(x, y));
                data.push((color.r * 255.0) as u8);
                data.push((color.g * 255.0) as u8);
                data.push((color.b * 255.0) as u8);
                data.push((color.a * 255.0) as u8);
            }
        }
        Texture2d::from_raw(ugli, size, &data, Format::RGBA, Type::UnsignedByte, false)
    }

    pub fn from_image_image(ugli: &Ugli, mut image: image::RgbaImage) -> Self {
        let size = vec2(image.width() as usize, image.height() as usize);
        image::imageops::flip_vertical_in_place(&mut image);
        let mut texture = Texture2d::from_raw(
            ugli,
            size,
            &image.into_raw(),
            Format::RGBA,
            Type::UnsignedByte,
            false,
        );
        if texture.is_pot() {
            texture.gen_mipmaps();
        }
        ugli.debug_check();
        texture
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_html_image_element(
        ugli: &Ugli,
        image: &web_sys::HtmlImageElement,
        premultiply_alpha: bool,
    ) -> Self {
        let mut texture =
            Texture2d::new_raw(ugli, vec2(image.width() as usize, image.height() as usize));
        let gl = &ugli.inner.raw;
        gl.pixel_store_flip_y(true);
        gl.pixel_store_premultiply_alpha(premultiply_alpha);
        gl.tex_image_2d_image(
            raw::TEXTURE_2D,
            0,
            raw::RGBA as raw::Int,
            raw::RGBA,
            raw::UNSIGNED_BYTE,
            image,
        );
        if texture.is_pot() {
            texture.gen_mipmaps();
        }
        ugli.debug_check();
        texture
    }
}
