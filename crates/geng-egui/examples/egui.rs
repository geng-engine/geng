use geng::prelude::*;
use geng_egui::*;

struct State {
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
    fn update(&mut self, _: f64) {
        // Begin frame
        self.egui.begin_frame();

        // UI logic here
        self.ui();

        // End frame
        self.egui.end_frame();
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.egui.handle_event(event);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::GRAY), None);

        // Render GUI
        self.egui.draw(framebuffer);
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Simple UI Example");
    let state = State {
        egui: EguiGeng::new(&geng),
    };

    geng::run(&geng, state);
}
