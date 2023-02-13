use super::*;

pub mod ttf;

pub use ttf::Ttf;
pub use Ttf as Font;

#[derive(ugli::Vertex, Debug)]
pub struct Vertex {
    pub a_pos: vec2<f32>,
    pub a_vt: vec2<f32>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct TextAlign(pub f32);

impl TextAlign {
    pub const LEFT: Self = Self(0.0);
    pub const BOTTOM: Self = Self(0.0);
    pub const TOP: Self = Self(1.0);
    pub const CENTER: Self = Self(0.5);
    pub const RIGHT: Self = Self(1.0);
}
