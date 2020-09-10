use super::*;

pub struct TextButton {
    core: WidgetCore,
    theme: Rc<Theme>,
    pub text: String,
    size: f32,
}

impl TextButton {
    pub fn new(
        #[allow(unused_variables)] geng: &Rc<Geng>,
        theme: &Rc<Theme>,
        text: String,
        size: f32,
    ) -> Self {
        Self {
            theme: theme.clone(),
            core: WidgetCore::new(),
            text,
            size,
        }
    }
    pub fn ui<'a>(&'a mut self, action: Box<dyn FnOnce() + 'a>) -> impl Widget + 'a {
        TextButtonUI {
            theme: &self.theme,
            core: &mut self.core,
            text: &self.text,
            size: self.size,
            action: Some(action),
        }
    }
}

pub struct TextButtonUI<'a> {
    theme: &'a Theme,
    core: &'a mut WidgetCore,
    action: Option<Box<dyn FnOnce() + 'a>>,
    text: &'a str,
    size: f32,
}

impl<'a> Widget for TextButtonUI<'a> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core.constraints = widget::Constraints {
            min_size: vec2(
                self.theme.font.measure(self.text, self.size).width() as f64,
                self.size as f64,
            ),
            flex: vec2(0.0, 0.0),
        };
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if self.text.is_empty() {
            return;
        }
        let mut size = partial_min(
            self.core().position.height() as f32,
            self.size * self.core().position.width() as f32
                / self.theme.font.measure(self.text, self.size).width(),
        );
        let color = if self.core().hovered() {
            self.theme.hover_color
        } else {
            self.theme.usable_color
        };
        let offset;
        if self.core().captured() {
            size *= self.theme.press_ratio;
            offset = self.core().position.size().map(|x| x as f32) * (1.0 - self.theme.press_ratio)
                / 2.0;
        } else {
            offset = vec2(0.0, 0.0);
        }
        self.theme.font.draw(
            framebuffer,
            self.text,
            self.core().position.bottom_left().map(|x| x as f32) + offset,
            size,
            color,
        );
    }
    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            if let Some(action) = self.action.take() {
                action();
            }
        }
    }
}
