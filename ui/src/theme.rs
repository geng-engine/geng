use crate::*;

pub struct Theme {
    pub color: Color<f32>,
    pub hover_color: Color<f32>,
    pub press_ratio: f32,
    pub font: Rc<Font>,
}

impl Theme {
    pub fn default(geng: &Rc<Geng>) -> Self {
        Self {
            color: Color::WHITE,
            hover_color: Color::rgb(0.3, 0.3, 1.0),
            press_ratio: 0.75,
            font: geng.default_font().clone(),
        }
    }
}
