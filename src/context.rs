use crate::*;

pub struct Ugli {
    pub(crate) inner: ugl::Context,
    size: Cell<Vec2<usize>>,
    phantom_data: PhantomData<*mut ()>,
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
impl Ugli {
    pub fn create_webgl(canvas: stdweb::web::html_element::CanvasElement) -> Self {
        let webgl = stdweb::unstable::TryInto::try_into(js! {
            var canvas = @{canvas};
            var options = {
                "alpha": false,
                "preserveDrawingBuffer": true,
                "stencil": false,
                "premultipliedAlpha": false,
                "powerPreference": "high-performance",
                "depth": true,
                "antialias": false
            };
            var gl = canvas.getContext("webgl", options);
            if (!gl) {
                gl = canvas.getContext("experimental-webgl", options);
            }
            return gl;
        })
        .unwrap();
        let ugli = Ugli {
            inner: ugl::Context::new(webgl),
            size: Cell::new(vec2(1, 1)),
            phantom_data: PhantomData,
        };
        ugli.init();
        ugli
    }
}

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
impl Ugli {
    pub fn create_from_glutin(glutin_context: &glutin::Context<glutin::PossiblyCurrent>) -> Self {
        let ugli = Ugli {
            inner: ugl::Context::new(|symbol| {
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
        gl.enable(ugl::DEPTH_TEST);
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        gl.enable(ugl::PROGRAM_POINT_SIZE);
        #[cfg(target_os = "windows")]
        gl.enable(ugl::POINT_SPRITE);
        gl.pixel_store(ugl::UNPACK_ALIGNMENT, 1);
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
