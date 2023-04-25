use super::*;

const TEXT_ALIGN: vec2<TextAlign> = vec2(TextAlign::LEFT, TextAlign::LEFT);

pub(crate) fn calc_text_constraints(
    text: &str,
    font: &Font,
    size: f32,
    _cx: &ConstraintsContext,
) -> Constraints {
    Constraints {
        min_size: vec2(
            font.measure(text, TEXT_ALIGN)
                .map_or(0.0, |aabb| aabb.width() as f64),
            size as f64,
        ),
        flex: vec2(0.0, 0.0),
    }
}

pub(crate) fn draw_text(text: &str, font: &Font, color: Rgba<f32>, cx: &mut DrawContext) {
    if text.is_empty() {
        return;
    }
    let _size = partial_min(
        cx.position.height() as f32,
        cx.position.width() as f32
            / font
                .measure(text, TEXT_ALIGN)
                .map_or(0.0, |aabb| aabb.width()),
    );
    let size = cx.position.height() as f32;
    font.draw(
        cx.framebuffer,
        &PixelPerfectCamera,
        text,
        TEXT_ALIGN,
        mat3::translate(
            cx.position.bottom_left().map(|x| x as f32) + vec2(0.0, -font.descender() * size),
        ) * mat3::scale_uniform(size),
        color,
    );
}

impl Widget for String {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        calc_text_constraints(self.as_str(), &cx.theme.font, cx.theme.text_size, cx)
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        draw_text(self.as_str(), &cx.theme.font, cx.theme.text_color, cx);
    }
}

impl Widget for &'_ str {
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        calc_text_constraints(self, &cx.theme.font, cx.theme.text_size, cx)
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        draw_text(self, &cx.theme.font, cx.theme.text_color, cx);
    }
}
