use super::*;

pub struct Text<'a, T: AsRef<str>> {
    core: WidgetCore,
    text: T,
    font: &'a Font,
    size: f32,
    color: Color<f32>,
}

pub fn text<'a, T: AsRef<str>>(
    text: T,
    font: &'a Font,
    size: f32,
    color: Color<f32>,
) -> Text<'a, T> {
    Text {
        core: WidgetCore::void(),
        text,
        font,
        size,
        color,
    }
}

impl<'a, T: AsRef<str>> Widget for Text<'a, T> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core_mut().constraints = widget::Constraints {
            min_size: vec2(
                self.font.measure(self.text.as_ref(), self.size).width() as f64,
                self.size as f64,
            ),
            flex: vec2(0.0, 0.0),
        };
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if self.text.as_ref().is_empty() {
            return;
        }
        let size = partial_min(
            self.core().position.height() as f32,
            self.size * self.core().position.width() as f32
                / self.font.measure(self.text.as_ref(), self.size).width(),
        );
        self.font.draw(
            framebuffer,
            self.text.as_ref(),
            self.core().position.bottom_left().map(|x| x as f32),
            size,
            self.color,
        );
    }
}
