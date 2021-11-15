use super::*;

pub(crate) struct GengImpl {
    window: Window,
    #[cfg(feature = "audio")]
    #[allow(dead_code)]
    pub(crate) audio: AudioContext,
    shader_lib: ShaderLib,
    draw_2d: Rc<Draw2D>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) asset_manager: AssetManager,
    default_font: Rc<Font>,
    fixed_delta_time: Cell<f64>,
    max_delta_time: Cell<f64>,
}

#[derive(Clone)]
pub struct Geng {
    pub(crate) inner: Rc<GengImpl>,
}

pub struct ContextOptions {
    pub title: String,
    pub vsync: bool,
    pub fixed_delta_time: f64,
    pub max_delta_time: f64,
    pub antialias: bool,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            title: "Geng Application".to_string(),
            vsync: true,
            fixed_delta_time: 0.05,
            max_delta_time: 0.1,
            antialias: false,
        }
    }
}

impl Geng {
    /// Initialize with default [ContextOptions] except for the title.
    /// To initialize with different options see [`Geng::new_with()`].
    pub fn new(title: &str) -> Self {
        Self::new_with(ContextOptions {
            title: title.to_owned(),
            ..default()
        })
    }

    /// Initialize with custom [ContextOptions].
    pub fn new_with(options: ContextOptions) -> Self {
        setup_panic_handler();
        let window = Window::new(&options);
        let ugli = window.ugli().clone();
        let shader_lib = ShaderLib::new_impl(&ugli, &options);
        let draw_2d = Rc::new(Draw2D::new(&shader_lib, &ugli));
        let default_font = Rc::new({
            let data = include_bytes!("font/default.ttf") as &[u8];
            Font::new_with(window.ugli(), &shader_lib, data.to_owned()).unwrap()
        });
        Self {
            inner: Rc::new(GengImpl {
                window,
                #[cfg(feature = "audio")]
                audio: AudioContext::new(),
                shader_lib,
                draw_2d,
                #[cfg(not(target_arch = "wasm32"))]
                asset_manager: AssetManager::new(),
                default_font,
                fixed_delta_time: Cell::new(options.fixed_delta_time),
                max_delta_time: Cell::new(options.max_delta_time),
            }),
        }
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    pub fn ugli(&self) -> &Ugli {
        self.inner.window.ugli()
    }

    pub fn shader_lib(&self) -> &ShaderLib {
        &self.inner.shader_lib
    }

    pub fn draw_2d(&self) -> &Rc<Draw2D> {
        &self.inner.draw_2d
    }

    pub fn default_font(&self) -> &Rc<Font> {
        &self.inner.default_font
    }
}

fn run_impl(geng: &Geng, state: impl State) {
    let state = Rc::new(RefCell::new(state));
    let ui_controller = Rc::new(RefCell::new(ui::Controller::new()));
    geng.inner.window.set_event_handler(Box::new({
        let state = state.clone();
        let ui_controller = ui_controller.clone();
        move |event| {
            if !ui_controller
                .borrow_mut()
                .handle_event(&mut state.borrow_mut().ui(), event.clone())
            {
                state.borrow_mut().handle_event(event);
            }
        }
    }));

    let mut timer = Timer::new();
    let mut fixed_updater = FixedUpdater::new(geng.inner.fixed_delta_time.get(), 0.0);
    let mut main_loop = {
        let geng = geng.clone();
        // TODO: remove the busy loop to not use any resources?
        move || {
            let delta_time = timer.tick();
            let delta_time = delta_time.min(geng.inner.max_delta_time.get());
            state.borrow_mut().update(delta_time);
            ui_controller
                .borrow_mut()
                .update(&mut state.borrow_mut().ui(), delta_time);

            for _ in 0..fixed_updater.update(delta_time) {
                state
                    .borrow_mut()
                    .fixed_update(fixed_updater.fixed_delta_time);
            }

            let window_size = geng.inner.window.real_size();
            // This means window is minimized?
            if window_size.x != 0 && window_size.y != 0 {
                let mut framebuffer = ugli::Framebuffer::default(geng.ugli());
                state.borrow_mut().draw(&mut framebuffer);
                ui_controller
                    .borrow_mut()
                    .draw(&mut state.borrow_mut().ui(), &mut framebuffer);
            }
            geng.inner.window.swap_buffers();

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
        while !geng.inner.window.should_close() {
            if !main_loop() {
                break;
            }
        }
    }
}

/// Run the application
pub fn run(geng: &Geng, state: impl State) {
    let mut state_manager = StateManager::new();
    state_manager.push(Box::new(state));
    let state = CombinedState(state_manager, DebugOverlay::new(geng));
    run_impl(geng, state);
}
