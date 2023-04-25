use super::*;

pub(crate) struct UgliImpl {
    pub(crate) raw: raw::Context,
    size: Cell<vec2<usize>>,
    phantom_data: PhantomData<*mut ()>,
}

#[derive(Clone)]
pub struct Ugli {
    pub(crate) inner: Rc<UgliImpl>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebGLContextOptions {
    pub alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub stencil: bool,
    pub premultiplied_alpha: bool,
    pub power_preference: &'static str,
    pub depth: bool,
    pub antialias: bool,
}

#[cfg(target_arch = "wasm32")]
impl Default for WebGLContextOptions {
    fn default() -> Self {
        Self {
            alpha: false,
            preserve_drawing_buffer: true,
            stencil: false,
            premultiplied_alpha: false,
            power_preference: "high-performance",
            depth: true,
            antialias: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Ugli {
    pub fn create_webgl(canvas: &web_sys::HtmlCanvasElement, options: WebGLContextOptions) -> Self {
        let context_options = serde_wasm_bindgen::to_value(&options).unwrap();
        let webgl;
        if let Some(context) = canvas
            .get_context_with_context_options("webgl", &context_options)
            .unwrap()
        {
            webgl = context;
        } else if let Some(context) = canvas
            .get_context_with_context_options("experimental-webgl", &context_options)
            .unwrap()
        {
            webgl = context;
        } else {
            panic!("Could not get webgl context");
        }
        let webgl: web_sys::WebGlRenderingContext = webgl.dyn_into().unwrap();
        let ugli = Ugli {
            inner: Rc::new(UgliImpl {
                raw: raw::Context::new(webgl),
                size: Cell::new(vec2(1, 1)),
                phantom_data: PhantomData,
            }),
        };
        ugli.init();
        ugli
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Ugli {
    pub fn create_from_glutin<F: Fn(&str) -> *const std::os::raw::c_void>(
        get_proc_address: F,
    ) -> Self {
        let ugli = Ugli {
            inner: Rc::new(UgliImpl {
                raw: raw::Context::new(get_proc_address),
                size: Cell::new(vec2(1, 1)),
                phantom_data: PhantomData,
            }),
        };
        ugli.init();
        ugli
    }
}

impl Ugli {
    pub fn init(&self) {
        let gl = &self.inner.raw;
        log::info!("GL version: {:?}", gl.get_version_string());
        gl.enable(raw::DEPTH_TEST);
        #[cfg(not(target_arch = "wasm32"))]
        gl.enable(raw::PROGRAM_POINT_SIZE);
        gl.pixel_store(raw::UNPACK_ALIGNMENT, 1);
        self.check();
    }
    #[doc(hidden)]
    pub fn _set_size(&self, size: vec2<usize>) {
        self.inner.size.set(size);
    }
    pub(crate) fn size(&self) -> vec2<usize> {
        self.inner.size.get()
    }
}
