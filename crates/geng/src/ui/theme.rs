use super::*;

pub struct Theme {
    geng: Geng,
    pub background_color: Color<f32>,
    pub warn_color: Color<f32>,
    pub error_color: Color<f32>,
    pub success_color: Color<f32>,
    pub usable_color: Color<f32>,
    pub hover_color: Color<f32>,
    pub text_color: Color<f32>,
    pub text_size: f32,
    pub press_ratio: f32,
    pub font: Rc<Font>,
}

impl Theme {
    pub fn dark(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            background_color: Color::BLACK,
            warn_color: Color::YELLOW,
            error_color: Color::RED,
            success_color: Color::GREEN,
            usable_color: Color::WHITE,
            hover_color: Color::rgb(0.3, 0.3, 1.0),
            text_color: Color::GRAY,
            text_size: 32.0,
            press_ratio: 0.25,
            font: geng.default_font().clone(),
        }
    }
    pub fn light(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            background_color: Color::WHITE,
            warn_color: Color::rgb(0.5, 0.5, 0.0),
            error_color: Color::RED,
            success_color: Color::GREEN,
            usable_color: Color::rgb(0.3, 0.3, 1.0),
            hover_color: Color::rgb(0.0, 0.0, 0.5),
            text_color: Color::BLACK,
            text_size: 32.0,
            press_ratio: 0.25,
            font: geng.default_font().clone(),
        }
    }
    pub fn geng(&self) -> &Geng {
        &self.geng
    }
}
