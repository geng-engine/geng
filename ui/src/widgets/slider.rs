use super::*;

pub struct Slider {
    theme: Rc<Theme>,
    core: WidgetCore,
    tick_radius: f32,
}

impl Clone for Slider {
    fn clone(&self) -> Self {
        Self {
            theme: self.theme.clone(),
            core: WidgetCore::new(),
            tick_radius: 0.0,
        }
    }
}

impl Deref for Slider {
    type Target = WidgetCore;
    fn deref(&self) -> &WidgetCore {
        &self.core
    }
}

impl Slider {
    pub fn new(theme: &Rc<Theme>) -> Self {
        Self {
            theme: theme.clone(),
            core: WidgetCore::new(),
            tick_radius: 0.0,
        }
    }
    pub fn ui<'a>(
        &'a mut self,
        value: f64,
        range: RangeInclusive<f64>,
        f: Box<dyn FnMut(f64) + 'a>,
    ) -> impl Widget + 'a {
        SliderUI {
            theme: &self.theme,
            tick_radius: &mut self.tick_radius,
            core: &mut self.core,
            value,
            range,
            f,
        }
    }
}

pub struct SliderUI<'a> {
    theme: &'a Theme,
    core: &'a mut WidgetCore,
    tick_radius: &'a mut f32,
    value: f64,
    range: RangeInclusive<f64>,
    f: Box<dyn FnMut(f64) + 'a>,
}

impl SliderUI<'_> {
    const ANIMATION_SPEED: f32 = 5.0;
}

impl<'a> Widget for SliderUI<'a> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn update(&mut self, delta_time: f64) {
        let height = self.core.position().height() as f32;
        let target_tick_radius = if self.core.hovered() || self.core.captured() {
            height / 2.0
        } else {
            height / 6.0
        };
        *self.tick_radius += clamp_abs(
            target_tick_radius - *self.tick_radius,
            Self::ANIMATION_SPEED * delta_time as f32 * height,
        );
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let draw_2d = self.theme.geng().draw_2d();
        let position = self.core.position().map(|x| x as f32);
        let line_width = position.height() / 3.0;
        let value_position = *self.tick_radius
            + ((self.value - *self.range.start()) / (*self.range.end() - *self.range.start()))
                as f32
                * (position.width() - *self.tick_radius * 2.0);
        draw_2d.quad(
            framebuffer,
            AABB::from_corners(
                position.bottom_left()
                    + vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                position.bottom_left()
                    + vec2(value_position, (position.height() + line_width) / 2.0),
            ),
            self.theme.hover_color,
        );
        draw_2d.quad(
            framebuffer,
            AABB::from_corners(
                position.bottom_left()
                    + vec2(value_position, (position.height() - line_width) / 2.0),
                position.top_right()
                    - vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
            ),
            self.theme.usable_color,
        );
        draw_2d.circle(
            framebuffer,
            position.bottom_left() + vec2(line_width / 2.0, position.height() / 2.0),
            line_width / 2.0,
            self.theme.hover_color,
        );
        draw_2d.circle(
            framebuffer,
            position.top_right() - vec2(line_width / 2.0, position.height() / 2.0),
            line_width / 2.0,
            self.theme.usable_color,
        );
        draw_2d.circle(
            framebuffer,
            position.bottom_left() + vec2(value_position, position.height() / 2.0),
            *self.tick_radius,
            self.theme.hover_color,
        );
    }
    fn handle_event(&mut self, event: &Event) {
        if self.core.captured() {
            if let Event::MouseDown { position, .. } | Event::MouseMove { position } = &event {
                let position = position.x - self.core.position().x_min;
                let new_value = *self.range.start()
                    + clamp(
                        (position - self.core.position().height() / 2.0)
                            / (self.core.position().width() - self.core.position().height()),
                        0.0..=1.0,
                    ) * (*self.range.end() - *self.range.start());
                (self.f)(new_value);
            }
        }
    }
}
