use super::*;

#[derive(ugli::Vertex)]
pub struct Vertex {
    a_pos: vec2<f32>,
}

pub struct EmptyLoadingScreen {
    geng: Geng,
}

impl EmptyLoadingScreen {
    pub fn new(geng: &Geng) -> Self {
        geng.set_loading_progress_title("Loading assets"); // TODO
        geng.set_loading_progress(0.0, None); // TODO
        Self { geng: geng.clone() }
    }
}

impl ProgressScreen for EmptyLoadingScreen {}

impl Drop for EmptyLoadingScreen {
    fn drop(&mut self) {
        self.geng.finish_loading();
    }
}

impl State for EmptyLoadingScreen {
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
        let progress = self.geng.inner.load_progress.borrow();
        self.geng
            .set_loading_progress(progress.progress as f64, Some(progress.total as f64));
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        #![allow(unused_variables)]
    }
}
