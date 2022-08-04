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
    pub fn new(h: T, s: T, v: T, a: T) -> Self {
        Self { h, s, v, a }
    }
    pub fn map<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Hsva<U> {
        Hsva {
            h: f(self.h),
            s: f(self.s),
            v: f(self.v),
            a: f(self.a),
        }
    }
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
