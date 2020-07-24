use super::*;

pub struct Ugli {
    pub(crate) inner: raw::Context,
    size: Cell<Vec2<usize>>,
    phantom_data: PhantomData<*mut ()>,
}

#[cfg(target_arch = "wasm32")]
impl Ugli {
    pub fn create_webgl(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let context_options = JsValue::from_serde({
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct ContextOptions {
                alpha: bool,
                preserve_drawing_buffer: bool,
                stencil: bool,
                premultiplied_alpha: bool,
                power_preference: &'static str,
                depth: bool,
                antialias: bool,
            }
            &ContextOptions {
                alpha: false,
                preserve_drawing_buffer: true,
                stencil: false,
                premultiplied_alpha: false,
                power_preference: "high-performance",
                depth: true,
                antialias: false,
            }
        })
        .unwrap();
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
            inner: raw::Context::new(webgl),
            size: Cell::new(vec2(1, 1)),
            phantom_data: PhantomData,
        };
        ugli.init();
        ugli
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Ugli {
    pub fn create_from_glutin(glutin_context: &glutin::Context<glutin::PossiblyCurrent>) -> Self {
        let ugli = Ugli {
            inner: raw::Context::new(|symbol| {
                glutin_context.get_proc_address(symbol) as *const c_void
            }),
            size: Cell::new(vec2(1, 1)),
            phantom_data: PhantomData,
        };
        ugli.init();
        ugli
    }
}

impl Ugli {
    pub fn init(&self) {
        let gl = &self.inner;
        info!("GL version: {:?}", gl.get_version_string());
        gl.enable(raw::DEPTH_TEST);
        #[cfg(not(target_arch = "wasm32"))]
        gl.enable(raw::PROGRAM_POINT_SIZE);
        #[cfg(target_os = "windows")]
        gl.enable(raw::POINT_SPRITE);
        gl.pixel_store(raw::UNPACK_ALIGNMENT, 1);
        self.check();
    }
    #[doc(hidden)]
    pub fn _set_size(&self, size: Vec2<usize>) {
        self.size.set(size);
    }
    pub(crate) fn size(&self) -> Vec2<usize> {
        self.size.get()
    }
}
