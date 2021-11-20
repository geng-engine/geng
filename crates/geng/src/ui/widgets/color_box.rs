use super::*;

pub struct ColorBox {
    geng: Geng,
    core: WidgetCore,
    pub color: Color<f32>,
}

impl ColorBox {
    pub fn new(geng: &Geng, color: Color<f32>) -> Self {
        Self {
            geng: geng.clone(),
            core: WidgetCore::void(),
            color,
        }
    }
}

impl Widget for ColorBox {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.geng.draw_2d(
            framebuffer,
            &PixelPerfectCamera,
            draw_2d::ColoredQuad::new(self.core().position.map(|x| x as f32), self.color),
        );
    }
}
