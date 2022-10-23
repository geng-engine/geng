use geng::prelude::*;

mod slider;

struct State {
    geng: Geng,
    counter: i32,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            counter: 0,
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }
    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;
        let minus_button = geng::ui::Button::new(cx, "-");
        let plus_button = geng::ui::Button::new(cx, "+");
        if minus_button.was_clicked() {
            self.counter -= 1;
        }
        if plus_button.was_clicked() {
            self.counter += 1;
        }
        let slider = slider::Slider::new(cx, (self.counter as f64).clamp(0.0, 100.0), 0.0..=100.0);
        if let Some(change) = slider.get_change() {
            self.counter = change as _;
        }
        (
            "counter example".center(),
            (
                minus_button,
                self.counter.to_string().padding_horizontal(32.0),
                plus_button,
            )
                .row()
                .center(),
            slider
                .fixed_size(vec2(100.0, cx.theme().text_size as f64))
                .center(),
        )
            .column()
            .center()
            .boxed()
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Geng UI Demo!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
