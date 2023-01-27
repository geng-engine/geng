use super::*;

pub struct FpsCounter {
    geng: Geng,
    next_fps_update: f64,
    frames: usize,
    fps: f64,
    text: String,
    timer: Timer,
}

impl FpsCounter {
    const FPS_UPDATE_INTERVAL: f64 = 1.0;

    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            next_fps_update: Self::FPS_UPDATE_INTERVAL,
            frames: 0,
            fps: 0.0,
            text: "FPS".to_owned(),
            timer: Timer::new(),
        }
    }

    pub fn update(&mut self) {
        let delta_time = self.timer.tick().as_secs_f64();
        self.next_fps_update -= delta_time;
        self.frames += 1;
        if self.next_fps_update < 0.0 {
            self.fps = self.frames as f64 / (Self::FPS_UPDATE_INTERVAL - self.next_fps_update);
            self.next_fps_update = Self::FPS_UPDATE_INTERVAL;
            self.frames = 0;
            self.text = format!("FPS: {}", self.fps.round() as i64);
        }
    }

    pub fn ui(&mut self) -> impl ui::Widget + '_ {
        use ui::*;
        ui::stack![
            ui::ColorBox::new(Rgba::BLACK).constraints_override(Constraints {
                min_size: vec2::ZERO,
                flex: vec2::ZERO
            }),
            ui::Text::new(&mut self.text, self.geng.default_font(), 16.0, Rgba::WHITE)
                .uniform_padding(2.0),
        ]
    }
}
