use super::*;

pub struct Button {
    core: WidgetCore,
    clicked: bool,
    hover_time: f64,
    capture_time: f64,
    click_time: f64,
}

impl AsRef<Button> for Button {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            core: WidgetCore::new(),
            clicked: false,
            hover_time: 0.0,
            capture_time: 0.0,
            click_time: 0.0,
        }
    }
    pub fn clicked(&mut self) -> bool {
        mem::replace(&mut self.clicked, false)
    }
}

impl Widget for Button {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn update(&mut self, delta_time: f64) {
        if self.core.hovered() {
            self.hover_time += delta_time;
        } else {
            self.hover_time = 0.0;
        }
        if self.core.captured() {
            self.capture_time += delta_time;
        } else {
            self.capture_time = 0.0;
        }
        self.click_time += delta_time;
    }
    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            self.clicked = true;
            self.click_time = 0.0;
        }
    }
}

impl Button {
    pub fn text<'a, B: Widget + AsRef<Button> + 'a, T: AsRef<str> + 'a>(
        button: B,
        text: T,
        theme: &Rc<Theme>,
    ) -> impl Widget + 'a {
        let text = Text::new(
            text,
            theme.font.clone(),
            theme.text_size,
            if button.as_ref().core.hovered {
                theme.hover_color
            } else {
                theme.usable_color
            },
        )
        .shrink(if button.as_ref().core().captured() {
            theme.press_ratio as f64
        } else {
            0.0
        });
        ui::stack![button, text]
    }
    pub fn texture<'a, B: Widget + AsRef<Button> + 'a>(
        button: B,
        texture: &'a ugli::Texture,
        theme: &Rc<Theme>,
    ) -> impl Widget + 'a {
        let texture = Texture::colored(
            theme.geng(),
            texture,
            if button.as_ref().core.hovered() {
                theme.hover_color
            } else {
                theme.usable_color
            },
        )
        .shrink(if button.as_ref().core().captured() {
            theme.press_ratio as f64
        } else {
            0.0
        });
        ui::stack![button, texture]
    }
}
