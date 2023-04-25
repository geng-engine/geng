use super::*;

pub struct ColorBox {
    pub color: Rgba<f32>,
    pub size: vec2<f32>,
}

impl ColorBox {
    pub fn new(color: Rgba<f32>) -> Self {
        Self {
            color,
            size: vec2::ZERO,
        }
    }
    pub fn divider(color: Rgba<f32>, size: f32) -> Self {
        Self {
            color,
            size: vec2(size, size),
        }
    }
}

impl Widget for ColorBox {
    fn draw(&mut self, cx: &mut DrawContext) {
        cx.draw2d.draw2d(
            cx.framebuffer,
            &PixelPerfectCamera,
            &draw2d::Quad::new(cx.position.map(|x| x as f32), self.color),
        );
    }

    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: self.size.map(|x| x as f64),
            flex: vec2(0.0, 0.0),
        }
    }
}
