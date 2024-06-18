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
    pub(crate) load_progress: RefCell<asset::LoadProgress>,
    // pub(crate) gilrs: Option<RefCell<gilrs::Gilrs>>,
}

#[derive(Clone)]
pub struct Geng {
    pub(crate) inner: Rc<GengImpl>,
}

#[derive(Debug, Clone)]
pub struct ContextOptions {
    pub window: window::Options,
    pub fixed_delta_time: f64,
    pub max_delta_time: f64,
    pub shader_prefix: Option<(String, String)>,
    pub target_ui_resolution: Option<vec2<f64>>,
    pub hot_reload: bool,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            window: window::Options {
                title: "Geng Application".to_string(),
                vsync: true,
                antialias: false,
                transparency: false,
                mouse_passthrough: false,
                size: None,
                fullscreen: !cfg!(debug_assertions),
                auto_close: true,
                start_hidden: false,
            },
            fixed_delta_time: 0.05,
            max_delta_time: 0.1,
            shader_prefix: None,
            target_ui_resolution: None,
            hot_reload: cfg!(debug_assertions),
        }
    }
}

impl Geng {
    /// Initialize with default [ContextOptions] except for the title.
    /// To initialize with different options see [`Geng::new_with()`].
    pub fn run<Fut>(title: &str, f: impl 'static + FnOnce(Geng) -> Fut)
    where
        Fut: 'static + Future<Output = ()>,
    {
        let mut options = ContextOptions::default();
        options.window.title = title.to_owned();
        Self::run_with(&options, f);
    }

    /// Initialize with custom [ContextOptions].
    pub fn run_with<Fut>(options: &ContextOptions, f: impl 'static + FnOnce(Geng) -> Fut)
    where
        Fut: 'static + Future<Output = ()>,
    {
        let options = options.clone();
        setup_panic_handler();
        window::run(&options.window.clone(), |window| async move {
            let ugli = window.ugli().clone();
            let shader_lib = shader::Library::new(
                &ugli,
                options.window.antialias,
                options.shader_prefix.clone(),
            );
            let draw2d = Rc::new(draw2d::Helper::new(&ugli, options.window.antialias));
            let default_font = Rc::new(Font::default(window.ugli()));
            #[cfg(feature = "audio")]
            let audio = Audio::new().unwrap();
            let geng = Geng {
                inner: Rc::new(GengImpl {
                    window: window.clone(),
                    #[cfg(feature = "audio")]
                    audio: audio.clone(),
                    shader_lib,
                    draw2d,
                    asset_manager: asset::Manager::new(
                        &window,
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
                    // gilrs: if cfg!(target_os = "android") {
                    //     None
                    // } else {
                    //     Some(RefCell::new(gilrs::Gilrs::new().unwrap()))
                    // },
                }),
            };
            f(geng).await;
        });
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    #[cfg(feature = "audio")]
    pub fn audio(&self) -> &Audio {
        &self.inner.audio
    }

    pub fn ugli(&self) -> &Ugli {
        self.inner.window.ugli()
    }

    pub fn asset_manager(&self) -> &asset::Manager {
        &self.inner.asset_manager
    }

    // pub fn gilrs(&self) -> Option<Ref<Gilrs>> {
    //     self.inner.gilrs.as_ref().map(|gilrs| gilrs.borrow())
    // }

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
    /// Run the application
    pub async fn run_state(&self, state: impl State) {
        self.finish_loading();
        let geng = self;
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
        let mut runner = Runner {
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
        };

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
        let mut events = geng.window().events();
        while let Some(event) = events.next().await {
            match event {
                Event::Draw => {
                    // if let Some(gilrs) = &geng.inner.gilrs {
                    //     let mut gilrs = gilrs.borrow_mut();
                    //     while let Some(gamepad_event) = gilrs.next_event() {
                    //         // TODO geng.inner.window.send_event(Event::Gamepad(gamepad_event));
                    //     }
                    // }

                    runner.update();
                    let window_size = geng.inner.window.real_size();
                    // This means window is minimized?
                    if window_size.x != 0 && window_size.y != 0 {
                        geng.window().with_framebuffer(|framebuffer| {
                            runner.draw(framebuffer);
                        });
                    }

                    if runner.need_to_quit() {
                        return;
                    }
                }
                _ => runner.handle_event(event),
            }
        }
    }
}
