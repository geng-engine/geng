use geng::{prelude::*, Camera2d, FixedUpdater, Updatable};

// Our timer
struct Timer {
    // Current elapsed time
    current_time: f32,
}

// This trait tells the updater how to update our timer
impl Updatable for Timer {
    fn update(&mut self, delta_time: f32) {
        // In this case we just increment the time by delta_time
        self.current_time += delta_time;
    }
}

struct State {
    // geng and camera will be used for rendering
    geng: Geng,
    camera: Camera2d,

    // This timer will be updated every second
    timer: FixedUpdater<Timer>,
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Draw current time to screen
        self.geng.default_font().draw(
            framebuffer,
            &self.camera,
            // Use timer.contents to access the contents of the updater
            &self.timer.contents.current_time.to_string(),
            Vec2::ZERO,
            geng::TextAlign::CENTER,
            5.0,
            Color::WHITE,
        );
    }

    fn update(&mut self, delta_time: f64) {
        // Update the updater, it will call our timer's update
        // method when
        self.timer.update(delta_time as f32);
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
        // To create a new updater we need to provide:
        //  - fixed_delta_time, which is the time period between update calls
        //  - delay, which is the delay before the first update call (leave 0.0 for immediate first update)
        //  - contents, which is anything that can be updated (implements the geng::Updatable trait)
        timer: FixedUpdater::new(1.0, 0.0, Timer { current_time: 0.0 }),
    };

    geng::run(&geng, state)
}
