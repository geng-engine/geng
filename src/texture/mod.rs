use super::*;

pub unsafe trait TexturePixel {
    const INTERNAL_FORMAT: raw::Enum;
    const FORMAT: raw::Enum;
    const TYPE: raw::Enum;
}

unsafe impl TexturePixel for Color<f32> {
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

pub struct Texture2d<P: TexturePixel> {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: raw::Texture,
    size: Cell<Vec2<usize>>,
    phantom_data: PhantomData<*mut P>,
}

impl<P: TexturePixel> Drop for Texture2d<P> {
    fn drop(&mut self) {
        let gl = &self.ugli.inner;
        gl.delete_texture(&self.handle);
    }
}

pub type Texture = Texture2d<Color<f32>>;

impl<P: TexturePixel> Texture2d<P> {
    fn new_raw(ugli: &Rc<Ugli>, size: Vec2<usize>) -> Self {
        let gl = &ugli.inner;
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

    pub fn new_uninitialized(ugli: &Rc<Ugli>, size: Vec2<usize>) -> Self {
        let texture = Self::new_raw(ugli, size);
        let gl = &ugli.inner;
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
    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        assert!(self.is_pot() || wrap_mode == WrapMode::Clamp);
        let gl = &self.ugli.inner;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(raw::TEXTURE_2D, raw::TEXTURE_WRAP_S, wrap_mode as raw::Int);
        gl.tex_parameteri(raw::TEXTURE_2D, raw::TEXTURE_WRAP_T, wrap_mode as raw::Int);
        self.ugli.debug_check();
    }

    pub fn set_filter(&mut self, filter: Filter) {
        assert!(self.is_pot() || filter == Filter::Nearest || filter == Filter::Linear);
        let gl = &self.ugli.inner;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(raw::TEXTURE_2D, raw::TEXTURE_MAG_FILTER, filter as raw::Int);
        self.ugli.debug_check();
    }

    pub fn size(&self) -> Vec2<usize> {
        self.size.get()
    }

    #[doc(hidden)]
    pub fn _set_size(&self, size: Vec2<usize>) {
        self.size.set(size);
    }

    #[doc(hidden)]
    pub fn _get_handle(&self) -> &raw::Texture {
        &self.handle
    }

    // TODO: use like Matrix<Color>?
    pub unsafe fn sub_image(&mut self, pos: Vec2<usize>, size: Vec2<usize>, data: &[u8]) {
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
        let gl = &self.ugli.inner;
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
        let gl = &self.ugli.inner;
        gl.bind_texture(raw::TEXTURE_2D, &self.handle);
        gl.generate_mipmap(raw::TEXTURE_2D);
        gl.tex_parameteri(
            raw::TEXTURE_2D,
            raw::TEXTURE_MIN_FILTER,
            raw::LINEAR_MIPMAP_LINEAR as raw::Int,
        );
        self.ugli.debug_check();
    }

    pub fn new_with<F: FnMut(Vec2<usize>) -> Color<f32>>(
        ugli: &Rc<Ugli>,
        size: Vec2<usize>,
        mut f: F,
    ) -> Self {
        let texture = Texture2d::new_raw(ugli, size);
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
        let gl = &ugli.inner;
        gl.tex_image_2d(
            raw::TEXTURE_2D,
            0,
            raw::RGBA as raw::Int,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
            0,
            raw::RGBA as raw::Enum,
            raw::UNSIGNED_BYTE,
            Some(&data),
        );
        ugli.debug_check();
        texture
    }

    pub fn from_image_image(ugli: &Rc<Ugli>, image: image::RgbaImage) -> Self {
        let size = vec2(image.width() as usize, image.height() as usize);
        let mut texture = Texture2d::new_raw(ugli, size);
        let gl = &ugli.inner;
        gl.tex_image_2d(
            raw::TEXTURE_2D,
            0,
            raw::RGBA as raw::Int,
            size.x as raw::SizeI,
            size.y as raw::SizeI,
            0,
            raw::RGBA as raw::Enum,
            raw::UNSIGNED_BYTE,
            Some(&image.into_raw()),
        );
        if texture.is_pot() {
            texture.gen_mipmaps();
        }
        ugli.debug_check();
        texture
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_image(ugli: &Rc<Ugli>, image: image::RgbaImage) -> Self {
        Self::from_image_image(ugli, image)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_image(ugli: &Rc<Ugli>, image: &web_sys::HtmlImageElement) -> Self {
        let mut texture =
            Texture2d::new_raw(ugli, vec2(image.width() as usize, image.height() as usize));
        let gl = &ugli.inner;
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
