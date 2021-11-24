use geng::{prelude::*, Camera2d};
use geng_egui::*;

struct State {
    geng: Geng,
    camera: Camera2d,
    egui: EguiGeng,
}

impl State {
    /// Here we will do ui stuff
    fn ui(&mut self) {
        egui::Window::new("Egui Window").show(self.egui.get_context(), |ui| {
            ui.heading("Hello World!");
        });
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        // Begin frame
        self.egui.begin_frame();

        // UI logic here
        self.ui();

        // End frame
        self.egui.end_frame();
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Render ui
        self.egui.draw(framebuffer);
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Simple UI Example");
    let state = State {
        geng: geng.clone(),
        camera: Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 100.0,
        },
        egui: EguiGeng::new(&geng),
    };

    geng::run(&geng, state);
}
