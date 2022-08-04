use super::*;

#[derive(Clone)]
pub struct Theme {
    geng: Geng,
    pub background_color: Rgba<f32>,
    pub warn_color: Rgba<f32>,
    pub error_color: Rgba<f32>,
    pub success_color: Rgba<f32>,
    pub usable_color: Rgba<f32>,
    pub hover_color: Rgba<f32>,
    pub text_color: Rgba<f32>,
    pub text_size: f32,
    pub press_ratio: f32,
    pub font: Rc<Font>,
}

impl Theme {
    pub fn dark(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            background_color: Rgba::BLACK,
            warn_color: Rgba::YELLOW,
            error_color: Rgba::RED,
            success_color: Rgba::GREEN,
            usable_color: Rgba::WHITE,
            hover_color: Rgba::from_rgb(0.3, 0.3, 1.0),
            text_color: Rgba::GRAY,
            text_size: 32.0,
            press_ratio: 0.25,
            font: geng.default_font().clone(),
        }
    }
    pub fn light(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            background_color: Rgba::WHITE,
            warn_color: Rgba::from_rgb(0.5, 0.5, 0.0),
            error_color: Rgba::RED,
            success_color: Rgba::GREEN,
            usable_color: Rgba::from_rgb(0.3, 0.3, 1.0),
            hover_color: Rgba::from_rgb(0.0, 0.0, 0.5),
            text_color: Rgba::BLACK,
            text_size: 32.0,
            press_ratio: 0.25,
            font: geng.default_font().clone(),
        }
    }
    pub fn geng(&self) -> &Geng {
        &self.geng
    }
}
