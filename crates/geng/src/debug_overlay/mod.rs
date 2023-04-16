use super::*;

mod console;
mod fps_counter;
mod touch_simulator;

use console::*;
use fps_counter::*;
use touch_simulator::*;

pub struct DebugOverlay<T> {
    geng: Geng,
    fps_counter: FpsCounter,
    console: Console,
    enabled: bool,
    touch_simulator: Option<TouchSimulator>,
    inner: T,
}

impl<T> DebugOverlay<T> {
    pub fn new(geng: &Geng, inner: T) -> Self {
        Self {
            geng: geng.clone(),
            fps_counter: FpsCounter::new(geng),
            console: Console::new(geng),
            enabled: false,
            touch_simulator: None,
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
        if let Some(touch_simulator) = &self.touch_simulator {
            touch_simulator.draw(framebuffer);
        }
    }
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { key } => match key {
                Key::F3 => {
                    self.enabled = !self.enabled;
                    return;
                }
                Key::M if self.geng.window().is_key_pressed(Key::F3) => {
                    self.enabled = !self.enabled;
                    self.touch_simulator = match self.touch_simulator {
                        Some(_) => None,
                        None => Some(TouchSimulator::new(&self.geng)),
                    };
                    return;
                }
                _ => {}
            },
            _ => {}
        }
        if let Some(touch_simulator) = &mut self.touch_simulator {
            if let Some(events) = touch_simulator.handle_event(&event) {
                for event in events {
                    self.inner.handle_event(event);
                }
            }
        }
        self.inner.handle_event(event);
    }
    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        if self.enabled {
            use ui::*;
            let ui = ui::column![
                self.fps_counter.ui().align(vec2(0.0, 1.0)),
                // self.console.ui(),
            ];
            Box::new(ui::stack![ui, self.inner.ui(cx)])
        } else {
            self.inner.ui(cx)
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {
        self.inner.fixed_update(delta_time);
    }
    fn transition(&mut self) -> Option<state::Transition> {
        self.inner.transition()
    }
}
