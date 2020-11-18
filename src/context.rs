use super::*;

pub struct Geng {
    window: Window,
    #[cfg(feature = "audio")]
    pub(crate) audio: AudioContext,
    shader_lib: ShaderLib,
    draw_2d: Rc<Draw2D>,
    pub(crate) asset_manager: AssetManager,
    default_font: Rc<Font>,
    max_delta_time: Cell<f64>,
}

pub struct ContextOptions {
    pub title: String,
    pub vsync: bool,
    pub max_delta_time: f64,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            title: "Geng Application".to_string(),
            vsync: true,
            max_delta_time: 0.1,
        }
    }
}

impl Geng {
    pub fn new(options: ContextOptions) -> Self {
        let window = Window::new(&options.title, options.vsync);
        let ugli = window.ugli().clone();
        let shader_lib = ShaderLib::new(window.ugli());
        let draw_2d = Rc::new(Draw2D::new(&shader_lib, &ugli));
        let default_font = Rc::new({
            let data = include_bytes!("font/default.ttf") as &[u8];
            Font::new_with(window.ugli(), &shader_lib, data.to_owned()).unwrap()
        });
        Geng {
            window,
            #[cfg(feature = "audio")]
            audio: AudioContext::new(),
            shader_lib,
            draw_2d,
            asset_manager: AssetManager::new(),
            default_font,
            max_delta_time: Cell::new(options.max_delta_time),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn ugli(&self) -> &Rc<Ugli> {
        self.window.ugli()
    }

    pub fn shader_lib(&self) -> &ShaderLib {
        &self.shader_lib
    }

    pub fn draw_2d(&self) -> &Rc<Draw2D> {
        &self.draw_2d
    }

    pub fn default_font(&self) -> &Rc<Font> {
        &self.default_font
    }
}

fn run_impl(geng: Rc<Geng>, state: impl State) {
    let state = Rc::new(RefCell::new(state));
    geng.window.set_event_handler(Box::new({
        let state = state.clone();
        move |event| {
            state.borrow_mut().handle_event(event);
        }
    }));

    let mut timer = Timer::new();
    let mut main_loop = {
        let geng = geng.clone();
        move || {
            let delta_time = timer.tick();
            let delta_time = delta_time.min(geng.max_delta_time.get());
            state.borrow_mut().update(delta_time);

            let mut framebuffer = ugli::Framebuffer::default(geng.ugli());
            state.borrow_mut().draw(&mut framebuffer);

            geng.window.swap_buffers();

            !matches!(state.borrow_mut().transition(), Some(Transition::Pop))
        }
    };

    #[cfg(target_arch = "wasm32")]
    {
        #[wasm_bindgen(inline_js = r#"
        export function run(main_loop) {
            function main_loop_wrapper() {
                main_loop();
                window.requestAnimationFrame(main_loop_wrapper);
            }
            main_loop_wrapper();
        }
        "#)]
        extern "C" {
            fn run(main_loop: &wasm_bindgen::JsValue);
        }
        let main_loop = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            main_loop();
        }) as Box<dyn FnMut()>);
        run(main_loop.as_ref());
        main_loop.forget();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        while !geng.window.should_close() {
            if !main_loop() {
                break;
            }
        }
    }
}

pub fn run(geng: Rc<Geng>, state: impl State) {
    let mut state_manager = StateManager::new();
    state_manager.push(Box::new(state));
    let state = DebugOverlay::new(&geng, state_manager);
    run_impl(geng, state);
}
