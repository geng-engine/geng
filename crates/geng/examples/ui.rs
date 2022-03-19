use geng::prelude::*;

struct State {
    geng: Geng,
    counter: i32,
    minus_button: geng::ui::Button,
    plus_button: geng::ui::Button,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            counter: 0,
            minus_button: geng::ui::Button::new(),
            plus_button: geng::ui::Button::new(),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
    fn ui(&mut self) -> Box<dyn geng::ui::Widget + '_> {
        use geng::ui;
        use geng::ui::*;
        let theme = Rc::new(ui::Theme::dark(&self.geng));
        if self.minus_button.clicked() {
            self.counter -= 1;
        }
        if self.plus_button.clicked() {
            self.counter += 1;
        }
        Box::new(
            ui::column![
                ui::Text::new(
                    "Counter example",
                    self.geng.default_font(),
                    32.0,
                    Color::WHITE
                )
                .center(),
                row![
                    ui::Button::text(&mut self.minus_button, "-", &theme).padding_right(32.0),
                    ui::Text::new(
                        self.counter.to_string(),
                        self.geng.default_font(),
                        32.0,
                        Color::WHITE
                    ),
                    ui::Button::text(&mut self.plus_button, "+", &theme).padding_left(32.0),
                ]
                .center(),
            ]
            .center(),
        )
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Geng UI Demo!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
