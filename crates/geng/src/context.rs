use super::*;

pub(crate) struct GengImpl {
    window: Window,
    #[cfg(feature = "audio")]
    #[allow(dead_code)]
    pub(crate) audio: AudioContext,
    shader_lib: ShaderLib,
    pub(crate) draw_2d: Rc<draw_2d::Helper>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) asset_manager: AssetManager,
    default_font: Rc<Font>,
    fixed_delta_time: Cell<f64>,
    max_delta_time: Cell<f64>,
    ui_theme: RefCell<Option<ui::Theme>>,
    pub(crate) options: ContextOptions,
    pub(crate) load_progress: RefCell<LoadProgress>,
}

#[derive(Clone)]
pub struct Geng {
    pub(crate) inner: Rc<GengImpl>,
}

#[derive(Debug, Clone)]
pub struct ContextOptions {
    pub title: String,
    pub vsync: bool,
    pub fixed_delta_time: f64,
    pub max_delta_time: f64,
    pub antialias: bool,
    pub transparency: bool,
    pub shader_prefix: Option<(String, String)>,
    pub window_size: Option<vec2<usize>>,
    pub fullscreen: bool,
    pub target_ui_resolution: Option<vec2<f64>>,
}

impl Default for ContextOptions {
    fn default() -> Self {
        let common_glsl = "#extension GL_OES_standard_derivatives : enable\nprecision highp int;\nprecision highp float;\n";
        Self {
            title: "Geng Application".to_string(),
            vsync: true,
            fixed_delta_time: 0.05,
            max_delta_time: 0.1,
            antialias: false,
            transparency: false,
            #[cfg(target_arch = "wasm32")]
            shader_prefix: Some((
                format!("{common_glsl}#define VERTEX_SHADER\n"),
                format!("{common_glsl}#define FRAGMENT_SHADER\n"),
            )),
            #[cfg(not(target_arch = "wasm32"))]
            shader_prefix: Some((
                format!("#version 100\n{common_glsl}#define VERTEX_SHADER\n"),
                format!("#version 100\n{common_glsl}#define FRAGMENT_SHADER\n"),
            )),
            window_size: None,
            fullscreen: false,
            target_ui_resolution: None,
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
        let draw_2d = Rc::new(draw_2d::Helper::new(&shader_lib, &ugli));
        let default_font = Rc::new({
            let data = include_bytes!("font/default.ttf") as &[u8];
            Font::new_with(window.ugli(), &shader_lib, data, default()).unwrap()
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
                ui_theme: RefCell::new(None),
                options,
                load_progress: RefCell::new(LoadProgress::new()),
            }),
        }
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    pub fn audio(&self) -> &AudioContext {
        &self.inner.audio
    }

    pub fn ugli(&self) -> &Ugli {
        self.inner.window.ugli()
    }

    pub fn shader_lib(&self) -> &ShaderLib {
        &self.inner.shader_lib
    }

    pub fn default_font(&self) -> &Rc<Font> {
        &self.inner.default_font
    }

    pub fn ui_theme(&self) -> ui::Theme {
        match &mut *self.inner.ui_theme.borrow_mut() {
            Some(theme) => theme.clone(),
            theme @ None => {
                *theme = Some(ui::Theme::dark(self));
                theme.clone().unwrap()
            }
        }
    }

    pub fn set_ui_theme(&self, theme: ui::Theme) {
        *self.inner.ui_theme.borrow_mut() = Some(theme);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_icon(&self, path: &std::path::Path) -> anyhow::Result<()> {
        self.window().set_icon(path)
    }
}

impl Geng {
    pub fn run_loading<S: State>(self, state: impl Future<Output = S> + 'static) {
        self.clone().run(LoadingScreen::new(
            &self,
            EmptyLoadingScreen::new(&self),
            state,
        ));
    }

    /// Run the application
    pub fn run(self, state: impl State) {
        let geng = &self;
        let mut state_manager = StateManager::new();
        state_manager.push(Box::new(state));
        let state = DebugOverlay::new(geng, state_manager);
        struct RunState<T> {
            geng: Geng,
            state: T,
            ui_controller: ui::Controller,
            timer: Timer,
            next_fixed_update: f64,
        }
        let state = Rc::new(RefCell::new(RunState {
            geng: geng.clone(),
            state,
            ui_controller: ui::Controller::new(geng),
            timer: Timer::new(),
            next_fixed_update: geng.inner.fixed_delta_time.get(),
        }));

        impl<T: State> RunState<T> {
            fn update(&mut self) {
                let delta_time = self.timer.tick().as_secs_f64();
                let delta_time = delta_time.min(self.geng.inner.max_delta_time.get());
                self.state.update(delta_time);
                self.ui_controller
                    .update(self.state.ui(&self.ui_controller).deref_mut(), delta_time);
                self.next_fixed_update -= delta_time;
                while self.next_fixed_update <= 0.0 {
                    let delta_time = self.geng.inner.fixed_delta_time.get();
                    self.next_fixed_update += delta_time;
                    self.state.fixed_update(delta_time);
                }
            }

            fn handle_event(&mut self, event: Event) {
                if self.ui_controller.handle_event(
                    self.state.ui(&self.ui_controller).deref_mut(),
                    event.clone(),
                ) {
                    return;
                }
                self.state.handle_event(event);
            }

            fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
                self.state.draw(framebuffer);
                self.ui_controller
                    .draw(self.state.ui(&self.ui_controller).deref_mut(), framebuffer);
            }

            fn need_to_quit(&mut self) -> bool {
                match self.state.transition() {
                    None => false,
                    Some(Transition::Pop) => true,
                    _ => unreachable!(),
                }
            }
        }
        geng.inner.window.set_event_handler(Box::new({
            let state = state.clone();
            move |event| {
                state.borrow_mut().handle_event(event);
            }
        }));
        let main_loop = {
            let geng = geng.clone();
            // TODO: remove the busy loop to not use any resources?
            move || {
                state.borrow_mut().update();
                let window_size = geng.inner.window.real_size();
                // This means window is minimized?
                if window_size.x != 0 && window_size.y != 0 {
                    let mut framebuffer = ugli::Framebuffer::default(geng.ugli());
                    state.borrow_mut().draw(&mut framebuffer);
                }
                geng.inner.window.swap_buffers();

                !state.borrow_mut().need_to_quit()
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
            })
                as Box<dyn FnMut()>);
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

            // Needed to drop state
            geng.inner.window.clear_event_handler();
        }
    }
}
