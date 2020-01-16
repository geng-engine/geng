use crate::*;

pub unsafe trait TexturePixel {
    const INTERNAL_FORMAT: ugl::Enum;
    const FORMAT: ugl::Enum;
    const TYPE: ugl::Enum;
}

unsafe impl TexturePixel for Color<f32> {
    const INTERNAL_FORMAT: ugl::Enum = ugl::RGBA;
    const FORMAT: ugl::Enum = ugl::RGBA;
    const TYPE: ugl::Enum = ugl::UNSIGNED_BYTE;
}

unsafe impl TexturePixel for u8 {
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    const INTERNAL_FORMAT: ugl::Enum = ugl::ALPHA;
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    const INTERNAL_FORMAT: ugl::Enum = ugl::RGBA;
    const FORMAT: ugl::Enum = ugl::ALPHA;
    const TYPE: ugl::Enum = ugl::UNSIGNED_BYTE;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum WrapMode {
    Repeat = ugl::REPEAT as _,
    Clamp = ugl::CLAMP_TO_EDGE as _,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Filter {
    Nearest = ugl::NEAREST as _,
    Linear = ugl::LINEAR as _,
}

pub struct Texture2d<P: TexturePixel> {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: ugl::Texture,
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
        gl.bind_texture(ugl::TEXTURE_2D, &handle);
        gl.tex_parameteri(
            ugl::TEXTURE_2D,
            ugl::TEXTURE_MIN_FILTER,
            ugl::LINEAR as ugl::Int,
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
            ugl::TEXTURE_2D,
            0,
            P::INTERNAL_FORMAT as ugl::Int,
            size.x as ugl::SizeI,
            size.y as ugl::SizeI,
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
        gl.bind_texture(ugl::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(ugl::TEXTURE_2D, ugl::TEXTURE_WRAP_S, wrap_mode as ugl::Int);
        gl.tex_parameteri(ugl::TEXTURE_2D, ugl::TEXTURE_WRAP_T, wrap_mode as ugl::Int);
        self.ugli.debug_check();
    }

    pub fn set_filter(&mut self, filter: Filter) {
        assert!(self.is_pot() || filter == Filter::Nearest || filter == Filter::Linear);
        let gl = &self.ugli.inner;
        gl.bind_texture(ugl::TEXTURE_2D, &self.handle);
        gl.tex_parameteri(ugl::TEXTURE_2D, ugl::TEXTURE_MAG_FILTER, filter as ugl::Int);
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
    pub fn _get_handle(&self) -> &ugl::Texture {
        &self.handle
    }

    // TODO: use like Matrix<Color>?
    pub unsafe fn sub_image(&mut self, pos: Vec2<usize>, size: Vec2<usize>, data: &[u8]) {
        assert_eq!(
            size.x
                * size.y
                * match P::FORMAT {
                    ugl::RGBA => 4,
                    ugl::ALPHA => 1,
                    _ => unreachable!(),
                },
            data.len()
        );
        let gl = &self.ugli.inner;
        gl.bind_texture(ugl::TEXTURE_2D, &self.handle);
        gl.tex_sub_image_2d(
            ugl::TEXTURE_2D,
            0,
            pos.x as ugl::Int,
            pos.y as ugl::Int,
            size.x as ugl::SizeI,
            size.y as ugl::SizeI,
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
        gl.bind_texture(ugl::TEXTURE_2D, &self.handle);
        gl.generate_mipmap(ugl::TEXTURE_2D);
        gl.tex_parameteri(
            ugl::TEXTURE_2D,
            ugl::TEXTURE_MIN_FILTER,
            ugl::LINEAR_MIPMAP_LINEAR as ugl::Int,
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
            ugl::TEXTURE_2D,
            0,
            ugl::RGBA as ugl::Int,
            size.x as ugl::SizeI,
            size.y as ugl::SizeI,
            0,
            ugl::RGBA as ugl::Enum,
            ugl::UNSIGNED_BYTE,
            Some(&data),
        );
        ugli.debug_check();
        texture
    }

    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    pub fn from_image(ugli: &Rc<Ugli>, image: image::RgbaImage) -> Self {
        let size = vec2(image.width() as usize, image.height() as usize);
        let mut texture = Texture2d::new_raw(ugli, size);
        let gl = &ugli.inner;
        gl.tex_image_2d(
            ugl::TEXTURE_2D,
            0,
            ugl::RGBA as ugl::Int,
            size.x as ugl::SizeI,
            size.y as ugl::SizeI,
            0,
            ugl::RGBA as ugl::Enum,
            ugl::UNSIGNED_BYTE,
            Some(&image.into_raw()),
        );
        if texture.is_pot() {
            texture.gen_mipmaps();
        }
        ugli.debug_check();
        texture
    }

    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    pub fn from_image(ugli: &Rc<Ugli>, image: stdweb::web::html_element::ImageElement) -> Self {
        let mut texture =
            Texture2d::new_raw(ugli, vec2(image.width() as usize, image.height() as usize));
        let gl = &ugli.inner;
        gl.tex_image_2d_src(
            ugl::TEXTURE_2D,
            0,
            ugl::RGBA as ugl::Int,
            ugl::RGBA,
            ugl::UNSIGNED_BYTE,
            image,
        );
        if texture.is_pot() {
            texture.gen_mipmaps();
        }
        ugli.debug_check();
        texture
    }
}
