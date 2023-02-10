use super::*;

/// [HSLA color](https://en.wikipedia.org/wiki/HSL_and_HSV).
/// Convert into/from [Rgba] via the [From] and [Into] traits.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hsla<T: ColorComponent> {
    /// Hue
    pub h: T,
    /// Saturation
    pub s: T,
    /// Lightness
    pub l: T,
    /// Alpha (opacity)
    pub a: T,
}

impl<T: ColorComponent> Hsla<T> {
    /// Construct a new Hsva value
    pub fn new(h: T, s: T, l: T, a: T) -> Self {
        Self { h, s, l, a }
    }

    /// Map all component values using same function
    pub fn map<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Hsla<U> {
        Hsla {
            h: f(self.h),
            s: f(self.s),
            l: f(self.l),
            a: f(self.a),
        }
    }

    /// Convert color component type
    pub fn convert<U: ColorComponent>(self) -> Hsla<U> {
        self.map(|x| x.convert())
    }
}

impl<C1: ColorComponent, C2: ColorComponent> From<Hsla<C1>> for Rgba<C2> {
    fn from(hsl: Hsla<C1>) -> Self {
        let Hsla { h, s, l, a } = hsl.convert::<f32>();
        let alpha = s * l.min(1.0 - l);
        let f = |n: f32| {
            let k = n + h * 12.0;
            let k = k - (k / 12.0).floor() * 12.0;
            l - alpha * (-1.0f32).max((k - 3.0).min(9.0 - k).min(1.0))
        };
        Rgba::new(f(0.0), f(8.0), f(4.0), a).convert()
    }
}

impl<C1: ColorComponent, C2: ColorComponent> From<Rgba<C1>> for Hsla<C2> {
    fn from(rgb: Rgba<C1>) -> Self {
        let Rgba { r, g, b, a } = rgb.convert::<f32>();
        let v = r.max(g).max(b); // max = v
        let min = r.min(g).min(b); // = v - c
        let c = v - min; // = 2 * (v - l)
        let l = v - c / 2.0; // = mid(r, g, b)

        let h = if c == 0.0 {
            0.0
        } else if v == r {
            (g - b) / c / 6.0
        } else if v == g {
            (b - r) / c / 6.0 + 1.0 / 3.0
        } else {
            // if v == b {
            (r - g) / c / 6.0 + 2.0 / 3.0
        };

        // let s = if v == 0.0 { 0.0 } else { c / v };
        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (v - l) / l.min(1.0 - l)
        };

        Hsla::new(h, s, l, a).convert()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EPS: f32 = 0.01;

    fn check_rgb(a: Rgba<f32>, b: Rgba<f32>) {
        let d =
            ((a.r - b.r).sqr() + (a.g - b.g).sqr() + (a.b - b.b).sqr() + (a.a - b.a).sqr()).sqrt();
        assert!(d.abs() < EPS)
    }

    fn check_hsl(a: Hsla<f32>, b: Hsla<f32>) {
        let d =
            ((a.h - b.h).sqr() + (a.s - b.s).sqr() + (a.l - b.l).sqr() + (a.a - b.a).sqr()).sqrt();
        assert!(d.abs() < EPS)
    }

    #[test]
    fn test_conversion() {
        let tests = [
            ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0]),
            ([0.75, 1.0, 1.0], [0.5, 1.0, 7.0 / 8.0]),
            ([0.438, 0.438, 0.812], [2.0 / 3.0, 0.5, 5.0 / 8.0]),
        ];

        for ([r, g, b], [h, s, l]) in tests {
            let rgb = Rgba::new(r, g, b, 1.0);
            let hsl = Hsla::new(h, s, l, 1.0);
            check_hsl(Hsla::from(rgb), hsl);
            check_rgb(Rgba::from(hsl), rgb);
        }
    }
}
