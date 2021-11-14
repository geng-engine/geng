use super::*;

mod console;
mod fps_counter;

use console::*;
use fps_counter::*;

pub struct DebugOverlay {
    fps_counter: FpsCounter,
    console: Console,
    state: Box<dyn State>,
    enabled: bool,
}

impl DebugOverlay {
    pub fn new(geng: &Geng, state: impl State) -> Self {
        Self {
            fps_counter: FpsCounter::new(geng),
            console: Console::new(geng),
            state: Box::new(state),
            enabled: false,
        }
    }
}

impl State for DebugOverlay {
    fn update(&mut self, delta_time: f64) {
        self.state.update(delta_time);
    }
    fn fixed_update(&mut self, delta_time: f64) {
        self.state.fixed_update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.state.draw(framebuffer);
    }
    fn handle_event(&mut self, event: Event) {
        if let Event::KeyDown { key: Key::F3 } = event {
            self.enabled = !self.enabled;
        } else {
            self.state.handle_event(event);
        }
    }
    fn transition(&mut self) -> Option<Transition> {
        match self.state.transition() {
            Some(Transition::Pop) => Some(Transition::Pop),
            None => None,
            _ => unreachable!(),
        }
    }
    fn ui(&mut self) -> Box<dyn ui::Widget + '_> {
        let overlay_ui: Box<dyn ui::Widget> = if self.enabled {
            use ui::*;
            let ui = ui::column![
                self.fps_counter.ui().align(vec2(0.0, 1.0)),
                self.console.ui(),
            ];
            Box::new(ui)
        } else {
            Box::new(ui::void())
        };
        Box::new(ui::stack![self.state.ui(), overlay_ui])
    }
}
