use geng::prelude::*;

struct State {
    geng: Geng,
    text: String,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            text: "Click to start editing".to_owned(),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 15.0,
            },
            &self.text,
            vec2::splat(geng::TextAlign::CENTER),
            mat3::identity(),
            Rgba::WHITE,
        );
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::TouchStart(_) | geng::Event::MouseDown { .. } => {
                self.geng.window().start_text_edit(&self.text);
            }
            geng::Event::EditText(text) => {
                self.text = text;
            }
            geng::Event::KeyDown {
                key: geng::Key::Backspace,
            } => {
                self.text.pop();
            }
            _ => {}
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("Moving");
    let state = State::new(&geng);
    geng.run_loading(async move { state });
}
