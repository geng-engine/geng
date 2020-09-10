use super::*;

pub struct Theme {
    pub usable_color: Color<f32>,
    pub hover_color: Color<f32>,
    pub text_color: Color<f32>,
    pub text_size: f32,
    pub press_ratio: f32,
    pub font: Rc<Font>,
}

impl Theme {
    pub fn default(geng: &Rc<Geng>) -> Self {
        Self {
            usable_color: Color::WHITE,
            hover_color: Color::rgb(0.3, 0.3, 1.0),
            text_color: Color::GRAY,
            text_size: 32.0,
            press_ratio: 0.75,
            font: geng.default_font().clone(),
        }
    }
}
