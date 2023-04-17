use batbox_color::*;
use batbox_la::*;
use batbox_time::Timer;
use geng_draw2d as draw2d;
use geng_ui as ui;
use geng_window::*;
use std::rc::Rc;

mod console;
mod fps_counter;
mod touch_simulator;

use console::*;
use fps_counter::*;
use touch_simulator::*;

pub struct DebugOverlay {
    show: bool,
    window: Window,
    draw2d: Rc<draw2d::Helper>,
    fps_counter: FpsCounter,
    console: Console,
    touch_simulator: Option<TouchSimulator>,
}

impl DebugOverlay {
    pub fn new(window: &Window) -> Self {
        Self {
            show: false,
            window: window.clone(),
            draw2d: Rc::new(draw2d::Helper::new(window.ugli(), true)),
            fps_counter: FpsCounter::new(),
            console: Console::new(),
            touch_simulator: None,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.fps_counter.update(delta_time);
        self.console.update(delta_time);
        if let Some(simulator) = &mut self.touch_simulator {
            simulator.update(delta_time);
        }
    }

    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if self.show {
            self.fps_counter.draw(framebuffer);
            self.console.draw(framebuffer);
        }
        if let Some(touch_simulator) = &self.touch_simulator {
            touch_simulator.draw(framebuffer);
        }
    }

    pub fn handle_event(&mut self, event: Event, mut inner_handler: impl FnMut(Event)) {
        if let Event::KeyDown { key } = event {
            match key {
                Key::F3 => {
                    self.show = !self.show;
                    return;
                }
                Key::M if self.window.is_key_pressed(Key::F3) => {
                    self.show = !self.show;
                    self.touch_simulator = match self.touch_simulator {
                        Some(_) => None,
                        None => Some(TouchSimulator::new(&self.draw2d)),
                    };
                    return;
                }
                _ => {}
            }
        }
        if let Some(touch_simulator) = &mut self.touch_simulator {
            if let Some(events) = touch_simulator.handle_event(&event) {
                for event in events {
                    inner_handler(event);
                }
                return;
            }
        }
        inner_handler(event);
    }
    pub fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        use ui::*;
        if self.show {
            ui::column![
                self.fps_counter.ui(cx).align(vec2(0.0, 1.0)),
                // self.console.ui(),
            ]
            .boxed()
        } else {
            Void.boxed()
        }
    }
    pub fn fixed_update(&mut self, _delta_time: f64) {}
}
