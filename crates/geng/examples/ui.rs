use geng::prelude::*;

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
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }
    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;
        let counter = Rc::new(RefCell::new(&mut self.counter));
        let result = (
            "counter example".center(),
            (
                geng::ui::Button::new(cx, "-", {
                    let counter = counter.clone();
                    move || **counter.borrow_mut() -= 1
                }),
                counter
                    .borrow()
                    .to_string()
                    .padding_left(32.0)
                    .padding_right(32.0),
                geng::ui::Button::new(cx, "+", {
                    let counter = counter.clone();
                    move || **counter.borrow_mut() += 1
                }),
            )
                .row()
                .center(),
        )
            .column()
            .center();
        result.boxed()
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Geng UI Demo!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}
