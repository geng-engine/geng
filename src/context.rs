use super::*;

pub(crate) struct GengImpl {
    window: Window,
    default_font: Rc<Font>,
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
        window::run(
            &geng_window::Options::new(title),
            move |window| async move {
                let default_font = Rc::new(Font::default(window.ugli()));
                let geng = Geng {
                    inner: Rc::new(GengImpl {
                        window: window.clone(),
                        default_font,
                    }),
                };
                while let Some(_) = window.events().next().await {
                    if geng.ugli().try_check().is_err() {
                        panic!("WTF");
                    }
                }
                // f(geng).await;
            },
        );
    }

    /// Initialize with custom [ContextOptions].
    pub fn run_with<Fut>(options: &ContextOptions, f: impl 'static + FnOnce(Geng) -> Fut)
    where
        Fut: 'static + Future<Output = ()>,
    {
        let options = options.clone();
        setup_panic_handler();
        window::run(&options.window.clone(), move |window| async move {
            let default_font = Rc::new(Font::default(window.ugli()));
            let geng = Geng {
                inner: Rc::new(GengImpl {
                    window: window.clone(),
                    default_font,
                }),
            };
            f(geng).await;
        });
    }

    pub fn window(&self) -> &Window {
        &self.inner.window
    }

    // #[cfg(feature = "audio")]
    // pub fn audio(&self) -> &Audio {
    //     &self.inner.audio
    // }

    pub fn ugli(&self) -> &Ugli {
        self.inner.window.ugli()
    }

    // pub fn asset_manager(&self) -> &asset::Manager {
    //     &self.inner.asset_manager
    // }

    // pub fn gilrs(&self) -> Option<Ref<Gilrs>> {
    //     self.inner.gilrs.as_ref().map(|gilrs| gilrs.borrow())
    // }

    // pub fn shader_lib(&self) -> &shader::Library {
    //     &self.inner.shader_lib
    // }

    pub fn default_font(&self) -> &Rc<Font> {
        &self.inner.default_font
    }

    // pub fn ui_theme(&self) -> ui::Theme {
    //     match &mut *self.inner.ui_theme.borrow_mut() {
    //         Some(theme) => theme.clone(),
    //         theme @ None => {
    //             *theme = Some(ui::Theme::dark(self.ugli()));
    //             theme.clone().unwrap()
    //         }
    //     }
    // }

    // pub fn set_ui_theme(&self, theme: ui::Theme) {
    //     *self.inner.ui_theme.borrow_mut() = Some(theme);
    // }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_icon(&self, path: &std::path::Path) -> anyhow::Result<()> {
        self.window().set_icon(path)
    }

    // pub fn draw2d(&self) -> &draw2d::Helper {
    //     &self.inner.draw2d
    // }
}

impl Geng {}
