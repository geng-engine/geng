use geng::prelude::*;

struct Test {
    t: f32,
}

impl geng::State for Test {
    fn update(&mut self, delta_time: f64) {
        self.t += delta_time as f32;
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(Rgba::new(self.t.fract(), 0.0, 0.0, 1.0)),
            None,
            None,
        );
    }
}

#[no_mangle]
fn android_main(app: android::App) {
    android::init(app);
    let geng = Geng::new("Android Test");
    geng.run(Test { t: 0.0 });
}
