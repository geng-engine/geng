use geng::prelude::*;

struct State {
    geng: Geng,
    font: geng::font::Ttf,
}

impl State {
    fn new(geng: &Geng) -> Self {
        let font = geng::font::Ttf::new(
            geng,
            include_bytes!("../src/font/default.ttf"),
            geng::font::ttf::Options {
                size: 64.0,
                max_distance: 8.0,
            },
        )
        .unwrap();
        Self {
            geng: geng.clone(),
            font,
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None);
        self.geng.draw_2d(
            framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::TexturedQuad::new(
                AABB::point(vec2(0, 0))
                    .extend_positive(self.geng.window().size())
                    .map(|x| x as f32),
                &self.font.atlas,
            ),
        )
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Font");
    let state = State::new(&geng);
    geng::run(&geng, state);
}
