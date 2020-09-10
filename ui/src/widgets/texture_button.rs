use super::*;

pub struct TextureButton {
    geng: Rc<Geng>,
    theme: Rc<Theme>,
    core: WidgetCore,
    current: Rc<ugli::Texture>,
    next: Rc<ugli::Texture>,
    time: f64,
}

impl TextureButton {
    pub fn new(geng: &Rc<Geng>, theme: &Rc<Theme>, texture: &Rc<ugli::Texture>) -> Self {
        Self {
            geng: geng.clone(),
            theme: theme.clone(),
            core: WidgetCore::new(),
            current: texture.clone(),
            next: texture.clone(),
            time: 1.0,
        }
    }
    pub fn swap(&mut self, texture: &Rc<ugli::Texture>) {
        self.current = self.next.clone();
        self.next = texture.clone();
        self.time = 0.0;
    }
    pub fn rotate(&mut self) {
        self.current = self.next.clone();
        self.time = 0.0;
    }
    pub fn ui<'a>(&'a mut self, action: Box<dyn FnMut() + 'a>) -> impl Widget + 'a {
        TextureButtonUI {
            geng: self.geng.clone(),
            theme: &self.theme,
            core: &mut self.core,
            texture: if self.time < 0.5 {
                &self.current
            } else {
                &self.next
            },
            time: &mut self.time,
            action,
        }
    }
}

pub struct TextureButtonUI<'a> {
    geng: Rc<Geng>,
    theme: &'a Theme,
    core: &'a mut WidgetCore,
    action: Box<dyn FnMut() + 'a>,
    texture: &'a ugli::Texture,
    time: &'a mut f64,
}

impl<'a> AsMut<WidgetCore> for TextureButtonUI<'a> {
    fn as_mut(&mut self) -> &mut WidgetCore {
        self.core
    }
}

impl<'a> Widget for TextureButtonUI<'a> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn update(&mut self, delta_time: f64) {
        *self.time = partial_min(*self.time + delta_time * 4.0, 1.0);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let angle = std::f32::consts::PI * ((*self.time as f32 * std::f32::consts::PI).cos() - 1.0);
        let mut v1 = Vec2::rotated(
            vec2(self.core().position().width() as f32 / 2.0, 0.0),
            angle,
        );
        let mut v2 = Vec2::rotated(
            vec2(0.0, self.core().position().height() as f32 / 2.0),
            angle,
        );
        if self.core().captured() {
            v1 *= self.theme.press_ratio;
            v2 *= self.theme.press_ratio;
        }
        let center = self.core().position().center().map(|x| x as f32);

        self.geng.draw_2d().textured(
            framebuffer,
            &[
                draw_2d::TexturedVertex {
                    a_pos: center - v1 - v2,
                    a_vt: vec2(0.0, 0.0),
                    a_color: Color::WHITE,
                },
                geng::draw_2d::TexturedVertex {
                    a_pos: center + v1 - v2,
                    a_vt: vec2(1.0, 0.0),
                    a_color: Color::WHITE,
                },
                geng::draw_2d::TexturedVertex {
                    a_pos: center + v1 + v2,
                    a_vt: vec2(1.0, 1.0),
                    a_color: Color::WHITE,
                },
                geng::draw_2d::TexturedVertex {
                    a_pos: center - v1 + v2,
                    a_vt: vec2(0.0, 1.0),
                    a_color: Color::WHITE,
                },
            ],
            self.texture,
            if self.core().hovered() {
                self.theme.hover_color
            } else {
                self.theme.usable_color
            },
            ugli::DrawMode::TriangleFan,
        )
    }
    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            (self.action)();
        }
    }
}
