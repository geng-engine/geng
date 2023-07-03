use geng::prelude::*;

struct State {
    geng: Geng,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::TRANSPARENT_BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Hello, World!",
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(framebuffer.size().map(|x| x as f32 / 2.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
        );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run_with(
        &geng::ContextOptions {
            window: {
                let mut options = geng::window::Options::new("Transparent");
                options.transparency = true;
                options
            },
            ..default()
        },
        |geng| async move {
            geng.run_state(State::new(&geng)).await;
        },
    );
}
