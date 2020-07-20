use crate::*;

pub struct Geng {
    window: Window,
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

pub fn run(geng: Rc<Geng>, state: impl State) {
    let state = Rc::new(RefCell::new(state));
    geng.window.set_event_handler(Box::new({
        let state = state.clone();
        move |event| {
            state.borrow_mut().handle_event(event);
        }
    }));

    let mut timer = Timer::new();
    let main_loop = {
        let geng = geng.clone();
        move || {
            let delta_time = timer.tick();
            let delta_time = delta_time.min(geng.max_delta_time.get());
            state.borrow_mut().update(delta_time);

            let mut framebuffer = ugli::Framebuffer::default(geng.ugli());
            state.borrow_mut().draw(&mut framebuffer);

            geng.window.swap_buffers();
        }
    };

    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    js! {
        var main_loop = @{main_loop};
        function main_loop_wrapper() {
            main_loop();
            window.requestAnimationFrame(main_loop_wrapper);
        }
        main_loop_wrapper();
    }

    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    {
        let mut main_loop = main_loop;
        while !geng.window.should_close() {
            main_loop();
        }
    }
}
