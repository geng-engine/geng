use super::*;

mod console;
mod fps_counter;

use console::*;
use fps_counter::*;

pub struct DebugOverlay {
    fps_counter: FpsCounter,
    console: Console,
    enabled: bool,
}

impl DebugOverlay {
    pub fn new(geng: &Geng) -> Self {
        Self {
            fps_counter: FpsCounter::new(geng),
            console: Console::new(geng),
            enabled: false,
        }
    }
}

impl State for DebugOverlay {
    fn update(&mut self, delta_time: f64) {
        self.fps_counter.update();
        self.console.update();
    }
    fn draw(&mut self, _framebuffer: &mut ugli::Framebuffer) {}
    fn handle_event(&mut self, event: Event) {
        if let Event::KeyDown { key: Key::F3 } = event {
            self.enabled = !self.enabled;
        }
    }
    fn ui(&mut self) -> Box<dyn ui::Widget + '_> {
        if self.enabled {
            use ui::*;
            let ui = ui::column![
                self.fps_counter.ui().align(vec2(0.0, 1.0)),
                self.console.ui(),
            ];
            Box::new(ui)
        } else {
            Box::new(ui::void())
        }
    }
}
