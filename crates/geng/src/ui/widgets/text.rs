use super::*;

pub struct Text<T: AsRef<str>, F: AsRef<Font>> {
    text: T,
    font: F,
    size: f32,
    color: Color<f32>,
}

impl<T: AsRef<str>, F: AsRef<Font>> Text<T, F> {
    pub fn new(text: T, font: F, size: f32, color: Color<f32>) -> Self {
        Self {
            text,
            font,
            size,
            color,
        }
    }
}

impl<T: AsRef<str>, F: AsRef<Font>> Widget for Text<T, F> {
    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: vec2(
                self.font
                    .as_ref()
                    .measure(self.text.as_ref(), self.size)
                    .map_or(0.0, |aabb| aabb.width() as f64),
                self.size as f64,
            ),
            flex: vec2(0.0, 0.0),
        }
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        if self.text.as_ref().is_empty() {
            return;
        }
        let size = partial_min(
            cx.position.height() as f32,
            self.size * cx.position.width() as f32
                / self
                    .font
                    .as_ref()
                    .measure(self.text.as_ref(), self.size)
                    .map_or(0.0, |aabb| aabb.width()),
        );
        self.font.as_ref().draw(
            cx.framebuffer,
            &PixelPerfectCamera,
            self.text.as_ref(),
            cx.position.bottom_left().map(|x| x as f32),
            TextAlign::LEFT,
            size,
            self.color,
        );
    }
}
