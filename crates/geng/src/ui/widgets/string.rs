use super::*;

fn calc_constraints(text: &str, cx: &ConstraintsContext) -> Constraints {
    let font_size = cx.theme.text_size;
    Constraints {
        min_size: vec2(
            cx.theme
                .font
                .measure(text, font_size)
                .map_or(0.0, |aabb| aabb.width() as f64),
            font_size as f64,
        ),
        flex: vec2(0.0, 0.0),
    }
}

fn draw(text: &str, cx: &mut DrawContext) {
    if text.is_empty() {
        return;
    }
    let size = partial_min(
        cx.position.height() as f32,
        cx.theme.text_size * cx.position.width() as f32
            / cx.theme
                .font
                .measure(text, cx.theme.text_size)
                .map_or(0.0, |aabb| aabb.width()),
    );
    cx.theme.font.draw(
        cx.framebuffer,
        &PixelPerfectCamera,
        text,
        cx.position.bottom_left().map(|x| x as f32),
        TextAlign::LEFT,
        size,
        cx.theme.text_color,
    );
}

impl Widget for String {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        calc_constraints(self.as_str(), cx)
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        draw(self.as_str(), cx);
    }
}

impl Widget for &'_ str {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        calc_constraints(self, cx)
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        draw(self, cx);
    }
}
