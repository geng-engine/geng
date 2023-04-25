use super::*;

pub struct Text<T: AsRef<str>, F: AsRef<Font>> {
    text: T,
    font: F,
    size: f32,
    color: Rgba<f32>,
}

impl<T: AsRef<str>, F: AsRef<Font>> Text<T, F> {
    pub fn new(text: T, font: F, size: f32, color: Rgba<f32>) -> Self {
        Self {
            text,
            font,
            size,
            color,
        }
    }
}

impl<T: AsRef<str>, F: AsRef<Font>> Widget for Text<T, F> {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        calc_text_constraints(self.text.as_ref(), self.font.as_ref(), self.size, cx)
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        draw_text(self.text.as_ref(), self.font.as_ref(), self.color, cx);
    }
}
