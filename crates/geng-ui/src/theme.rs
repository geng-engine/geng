use super::*;

#[derive(Clone)]
pub struct Theme {
    ugli: Ugli,
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
    pub fn dark(ugli: &Ugli) -> Self {
        Self {
            ugli: ugli.clone(),
            background_color: Rgba::BLACK,
            warn_color: Rgba::YELLOW,
            error_color: Rgba::RED,
            success_color: Rgba::GREEN,
            usable_color: Rgba::WHITE,
            hover_color: Rgba::opaque(0.3, 0.3, 1.0),
            text_color: Rgba::GRAY,
            text_size: 32.0,
            press_ratio: 0.25,
            font: Rc::new(Font::default(ugli)),
        }
    }
    pub fn light(ugli: &Ugli) -> Self {
        Self {
            ugli: ugli.clone(),
            background_color: Rgba::WHITE,
            warn_color: Rgba::opaque(0.5, 0.5, 0.0),
            error_color: Rgba::RED,
            success_color: Rgba::GREEN,
            usable_color: Rgba::opaque(0.3, 0.3, 1.0),
            hover_color: Rgba::opaque(0.0, 0.0, 0.5),
            text_color: Rgba::BLACK,
            text_size: 32.0,
            press_ratio: 0.25,
            font: Rc::new(Font::default(ugli)),
        }
    }
    pub fn ugli(&self) -> &Ugli {
        &self.ugli
    }
}
