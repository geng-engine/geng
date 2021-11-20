use super::*;

pub struct ColoredQuad {
    pub aabb: AABB<f32>,
    pub color: Color<f32>,
}

impl ColoredQuad {
    pub fn new(aabb: AABB<f32>, color: Color<f32>) -> Self {
        Self { aabb, color }
    }
}

impl Drawable2d for ColoredQuad {
    fn draw_2d(
        self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl AbstractCamera2d,
    ) {
        geng.inner
            .draw_2d
            .quad(framebuffer, camera, self.aabb, self.color);
    }
}
