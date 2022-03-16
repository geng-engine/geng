use super::*;

mod console;
mod fps_counter;

use console::*;
use fps_counter::*;

pub struct DebugOverlay<T> {
    fps_counter: FpsCounter,
    console: Console,
    enabled: bool,
    inner: T,
}

impl<T> DebugOverlay<T> {
    pub fn new(geng: &Geng, inner: T) -> Self {
        Self {
            fps_counter: FpsCounter::new(geng),
            console: Console::new(geng),
            enabled: false,
            inner,
        }
    }
}

impl<T: State> State for DebugOverlay<T> {
    fn update(&mut self, delta_time: f64) {
        self.fps_counter.update();
        self.console.update();
        self.inner.update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.inner.draw(framebuffer);
    }
    fn handle_event(&mut self, event: Event) {
        if let Event::KeyDown { key: Key::F3 } = event {
            self.enabled = !self.enabled;
        } else {
            self.inner.handle_event(event);
        }
    }
    fn ui(&mut self) -> Box<dyn ui::Widget + '_> {
        if self.enabled {
            use ui::*;
            let ui = ui::column![
                self.fps_counter.ui().align(vec2(0.0, 1.0)),
                self.console.ui(),
            ];
            Box::new(ui::stack![ui, self.inner.ui()])
        } else {
            self.inner.ui()
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {
        self.inner.fixed_update(delta_time);
    }
    fn transition(&mut self) -> Option<Transition> {
        self.inner.transition()
    }
}
