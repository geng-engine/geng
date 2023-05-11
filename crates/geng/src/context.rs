use super::*;

pub(crate) struct GengImpl {
    window: Window,
    #[cfg(feature = "audio")]
    audio: Audio,
    shader_lib: shader::Library,
    pub(crate) draw2d: Rc<draw2d::Helper>,
    asset_manager: asset::Manager,
    default_font: Rc<Font>,
    fixed_delta_time: Cell<f64>,
    max_delta_time: Cell<f64>,
    ui_theme: RefCell<Option<ui::Theme>>,
    pub(crate) options: ContextOptions,
    pub(crate) load_progress: RefCell<LoadProgress>,
    pub(crate) gilrs: RefCell<gilrs::Gilrs>,
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
    pub hot_reload: bool,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            title: "Geng Application".to_string(),
            vsync: true,
            fixed_delta_time: 0.05,
            max_delta_time: 0.1,
            antialias: false,
            transparency: false,
            shader_prefix: None,
            window_size: None,
            fullscreen: !cfg!(debug_assertions),
            target_ui_resolution: None,
            hot_reload: cfg!(debug_assertions),
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
        let window = Window::new(&window::Options {
            fullscreen: options.fullscreen,
            vsync: options.vsync,
            title: options.title.clone(),
            antialias: options.antialias,
            transparency: options.transparency,
            size: options.window_size,
        });
        let ugli = window.ugli().clone();
        let shader_lib =
            shader::Library::new(&ugli, options.antialias, options.shader_prefix.clone());
        let draw2d = Rc::new(draw2d::Helper::new(&ugli, options.antialias));
        let default_font = Rc::new(Font::default(window.ugli()));
        #[cfg(feature = "audio")]
        let audio = Audio::new();
        Self {
            inner: Rc::new(GengImpl {
                window,
                #[cfg(feature = "audio")]
                audio: audio.clone(),
                shader_lib,
                draw2d,
                asset_manager: asset::Manager::new(
                    &ugli,
                    #[cfg(feature = "audio")]
                    &audio,
                    options.hot_reload,
                ),
                default_font,
                fixed_delta_time: Cell::new(options.fixed_delta_time),
                max_delta_time: Cell::new(options.max_delta_time),
                ui_theme: RefCell::new(None),
                options,
                load_progress: RefCell::new(asset::LoadProgress::new()),
                gilrs: RefCell::new(gilrs::Gilrs::new().unwrap()),
            }),
        }
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    pub fn audio(&self) -> &Audio {
        &self.inner.audio
    }

    pub fn ugli(&self) -> &Ugli {
        self.inner.window.ugli()
    }

    pub fn asset_manager(&self) -> &asset::Manager {
        &self.inner.asset_manager
    }

    pub fn gilrs(&self) -> Ref<Gilrs> {
        self.inner.gilrs.borrow()
    }

    pub fn shader_lib(&self) -> &shader::Library {
        &self.inner.shader_lib
    }

    pub fn default_font(&self) -> &Rc<Font> {
        &self.inner.default_font
    }

    pub fn ui_theme(&self) -> ui::Theme {
        match &mut *self.inner.ui_theme.borrow_mut() {
            Some(theme) => theme.clone(),
            theme @ None => {
                *theme = Some(ui::Theme::dark(self.ugli()));
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

    pub fn draw2d(&self) -> &draw2d::Helper {
        &self.inner.draw2d
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
        struct StateWrapper {
            state_manager: state::Manager,
            debug_overlay: geng_debug_overlay::DebugOverlay,
        }
        impl StateWrapper {
            fn update(&mut self, delta_time: f64) {
                self.state_manager.update(delta_time);
                self.debug_overlay.update(delta_time);
            }
            fn fixed_update(&mut self, delta_time: f64) {
                self.state_manager.fixed_update(delta_time);
                self.debug_overlay.fixed_update(delta_time);
            }
            fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
                self.state_manager.draw(framebuffer);
                self.debug_overlay.draw(framebuffer);
            }
            fn handle_event(&mut self, event: Event) {
                self.debug_overlay
                    .handle_event(event, |event| self.state_manager.handle_event(event));
            }
            fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> impl ui::Widget + 'a {
                ui::stack![self.state_manager.ui(cx), self.debug_overlay.ui(cx)]
            }
            fn transition(&mut self) -> Option<state::Transition> {
                self.state_manager.transition()
            }
        }
        struct Runner {
            geng: Geng,
            state: StateWrapper,
            ui_controller: ui::Controller,
            timer: Timer,
            next_fixed_update: f64,
        }
        let runner = Rc::new(RefCell::new(Runner {
            geng: geng.clone(),
            state: StateWrapper {
                state_manager: {
                    let mut state_manager = state::Manager::new();
                    state_manager.push(Box::new(state));
                    state_manager
                },
                debug_overlay: geng_debug_overlay::DebugOverlay::new(geng.window()),
            },
            ui_controller: ui::Controller::new(
                geng.ugli(),
                geng.ui_theme(),
                geng.inner.options.target_ui_resolution,
            ),
            timer: Timer::new(),
            next_fixed_update: geng.inner.fixed_delta_time.get(),
        }));

        impl Runner {
            fn update(&mut self) {
                let delta_time = self.timer.tick().as_secs_f64();
                let delta_time = delta_time.min(self.geng.inner.max_delta_time.get());
                self.state.update(delta_time);
                self.ui_controller
                    .update(&mut self.state.ui(&self.ui_controller), delta_time);
                self.next_fixed_update -= delta_time;
                while self.next_fixed_update <= 0.0 {
                    let delta_time = self.geng.inner.fixed_delta_time.get();
                    self.next_fixed_update += delta_time;
                    self.state.fixed_update(delta_time);
                }
            }

            fn handle_event(&mut self, event: Event) {
                if self
                    .ui_controller
                    .handle_event(&mut self.state.ui(&self.ui_controller), event.clone())
                {
                    return;
                }
                self.state.handle_event(event);
            }

            fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
                self.state.draw(framebuffer);
                self.ui_controller
                    .draw(&mut self.state.ui(&self.ui_controller), framebuffer);
            }

            fn need_to_quit(&mut self) -> bool {
                match self.state.transition() {
                    None => false,
                    Some(state::Transition::Pop) => true,
                    _ => unreachable!(),
                }
            }
        }
        geng.inner.window.set_event_handler(Box::new({
            let runner = runner.clone();
            move |event| {
                runner.borrow_mut().handle_event(event);
            }
        }));
        let main_loop = {
            let geng = geng.clone();
            // TODO: remove the busy loop to not use any resources?
            move || {
                {
                    let mut gilrs = geng.inner.gilrs.borrow_mut();
                    while let Some(gamepad_event) = gilrs.next_event() {
                        geng.inner.window.send_event(Event::Gamepad(gamepad_event));
                    }
                }

                runner.borrow_mut().update();
                let window_size = geng.inner.window.real_size();
                // This means window is minimized?
                if window_size.x != 0 && window_size.y != 0 {
                    let mut framebuffer = ugli::Framebuffer::default(geng.ugli());
                    runner.borrow_mut().draw(&mut framebuffer);
                }
                geng.inner.window.swap_buffers();

                !runner.borrow_mut().need_to_quit()
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
