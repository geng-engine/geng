use super::*;

pub struct Circle {
    pub center: Vec2<f32>,
    pub radius: f32,
    pub color: Color<f32>,
}

impl Circle {
    pub fn new(center: Vec2<f32>, radius: f32, color: Color<f32>) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Drawable2d for Circle {
    fn draw_2d(
        self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
    ) {
        geng.inner
            .draw_2d
            .circle(framebuffer, camera, self.center, self.radius, self.color);
    }
}
