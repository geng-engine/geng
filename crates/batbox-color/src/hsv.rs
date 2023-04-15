use super::*;

/// HSVA Color
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hsva<T: ColorComponent> {
    /// Hue
    pub h: T,
    /// Saturation
    pub s: T,
    /// Value
    pub v: T,
    /// Alpha (opacity)
    pub a: T,
}

impl<T: ColorComponent> Hsva<T> {
    /// Construct a new Hsva value
    pub fn new(h: T, s: T, v: T, a: T) -> Self {
        Self { h, s, v, a }
    }

    /// Map all component values using same function
    pub fn map<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Hsva<U> {
        Hsva {
            h: f(self.h),
            s: f(self.s),
            v: f(self.v),
            a: f(self.a),
        }
    }

    /// Convert color component type
    pub fn convert<U: ColorComponent>(self) -> Hsva<U> {
        self.map(|x| x.convert())
    }
}

impl<C1: ColorComponent, C2: ColorComponent> From<Hsva<C1>> for Rgba<C2> {
    fn from(hsv: Hsva<C1>) -> Self {
        let Hsva { h, s, v, a } = hsv.convert::<f32>();
        let h = h - h.floor();
        let r;
        let g;
        let b;
        let f = h * 6.0 - (h * 6.0).floor();
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        if h * 6.0 < 1.0 {
            r = v;
            g = t;
            b = p;
        } else if h * 6.0 < 2.0 {
            r = q;
            g = v;
            b = p;
        } else if h * 6.0 < 3.0 {
            r = p;
            g = v;
            b = t;
        } else if h * 6.0 < 4.0 {
            r = p;
            g = q;
            b = v;
        } else if h * 6.0 < 5.0 {
            r = t;
            g = p;
            b = v;
        } else {
            r = v;
            g = p;
            b = q;
        }
        Rgba::new(r, g, b, a).convert()
    }
}

impl<C1: ColorComponent, C2: ColorComponent> From<Rgba<C1>> for Hsva<C2> {
    fn from(rgb: Rgba<C1>) -> Self {
        let Rgba { r, g, b, a } = rgb.convert::<f32>();
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let h = if max == min {
            0.0 // Undefined really
        } else if max == r && g >= b {
            (g - b) / (max - min) / 6.0
        } else if max == r && g < b {
            (g - b) / (max - min) / 6.0 + 1.0
        } else if max == g {
            (b - r) / (max - min) / 6.0 + 1.0 / 3.0
        } else {
            // if max = b {
            (r - g) / (max - min) / 6.0 + 2.0 / 3.0
        };
        let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };
        let v = max;
        Hsva::new(h, s, v, a).convert()
    }
}
