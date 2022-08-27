use geng::{prelude::*, Camera2d};

struct State {
    // geng and camera will be used for rendering
    geng: Geng,
    camera: Camera2d,

    // This timer will be updated every second
    timer: f64,
    // by this updater
    timer_updater: FixedUpdater,
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Draw current time to screen
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            // Use timer.contents to access the contents of the updater
            &self.timer.to_string(),
            Vec2::ZERO,
            geng::TextAlign::CENTER,
            5.0,
            Rgba::WHITE,
        );
    }

    fn update(&mut self, delta_time: f64) {
        // Update the updater, it will return the number of
        // fixed updates that should be called in this frame.
        // Sometimes this number can be 0, and sometimes
        // it can be more than 1.
        for _ in 0..self.timer_updater.update(delta_time) {
            self.timer += self.timer_updater.fixed_delta_time;
        }
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Blank");

    let state = State {
        geng: geng.clone(),
        camera: Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 50.0,
        },
        timer: 0.0,
        // To create a new updater we need to provide:
        //  - fixed_delta_time, which is the time period between update calls
        //  - delay, which is the delay before the first update call (leave 0.0 for immediate first update)
        timer_updater: FixedUpdater::new(1.0, 0.0),
    };

    geng::run(&geng, state)
}
