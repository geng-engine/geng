use geng_core::*;
use geng_ui as ui;
use prelude::*;

mod console;
mod fps_counter;

use console::*;
use fps_counter::*;

struct Data {
    fps_counter: FpsCounter,
    console: Console,
}

impl Data {
    fn new(geng: &Rc<Geng>) -> Self {
        Self {
            fps_counter: FpsCounter::new(geng),
            console: Console::new(geng),
        }
    }
    fn before_draw(&mut self) {
        self.fps_counter.before_draw();
        self.console.before_draw();
    }
    fn ui(&mut self) -> impl ui::Widget + '_ {
        use ui::*;
        ui::column![
            self.fps_counter.ui().align(vec2(0.0, 1.0)),
            self.console.ui(),
        ]
    }
}

pub struct DebugOverlay {
    geng: Rc<Geng>,
    ui_controller: ui::Controller,
    data: Data,
    state: Box<dyn State>,
    enabled: bool,
}

impl DebugOverlay {
    pub fn new(geng: &Rc<Geng>, state: impl State) -> Self {
        Self {
            geng: geng.clone(),
            ui_controller: ui::Controller::new(),
            data: Data::new(geng),
            state: Box::new(state),
            enabled: false,
        }
    }
}

impl State for DebugOverlay {
    fn update(&mut self, delta_time: f64) {
        if self.enabled {
            self.ui_controller.update(self.data.ui(), delta_time);
        }
        self.state.update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.state.draw(framebuffer);
        if self.enabled {
            self.data.before_draw();
            self.ui_controller.draw(self.data.ui(), framebuffer);
        }
    }
    fn handle_event(&mut self, event: Event) {
        if let Event::KeyDown { key: Key::F3 } = event {
            self.enabled = !self.enabled;
        }
        if !self.enabled
            || !self
                .ui_controller
                .handle_event(self.data.ui(), event.clone())
        {
            self.state.handle_event(event);
        }
    }
}
