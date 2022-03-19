use super::*;

pub struct ColorBox {
    pub color: Color<f32>,
}

impl ColorBox {
    pub fn new(color: Color<f32>) -> Self {
        Self { color }
    }
}

impl Widget for ColorBox {
    fn draw(&mut self, cx: &mut DrawContext) {
        cx.geng.draw_2d(
            cx.framebuffer,
            &PixelPerfectCamera,
            &draw_2d::Quad::new(cx.position.map(|x| x as f32), self.color),
        );
    }

    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints::default()
    }
}
