use super::*;

/// RGBA Color
#[repr(C)]
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Rgba<T: ColorComponent> {
    /// Red component
    pub r: T,
    /// Green component
    pub g: T,
    /// Blue component
    pub b: T,
    /// Alpha (opacity) component
    pub a: T,
}

impl<T: ColorComponent> From<Rgba<T>> for String {
    fn from(color: Rgba<T>) -> String {
        format!("{color}")
    }
}

impl<T: ColorComponent> TryFrom<String> for Rgba<T> {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl<T: ColorComponent> TryFrom<&str> for Rgba<T> {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(hex) = value.strip_prefix('#') {
            fn d(x: u8) -> u8 {
                x * 16 + x
            }
            return Ok(match hex.len() {
                3 => Rgba::<u8>::opaque(
                    d(u8::from_str_radix(&hex[0..1], 16)?),
                    d(u8::from_str_radix(&hex[1..2], 16)?),
                    d(u8::from_str_radix(&hex[2..3], 16)?),
                ),
                4 => Rgba::<u8>::new(
                    d(u8::from_str_radix(&hex[0..1], 16)?),
                    d(u8::from_str_radix(&hex[1..2], 16)?),
                    d(u8::from_str_radix(&hex[2..3], 16)?),
                    d(u8::from_str_radix(&hex[3..4], 16)?),
                ),
                6 => Rgba::<u8>::opaque(
                    u8::from_str_radix(&hex[0..2], 16)?,
                    u8::from_str_radix(&hex[2..4], 16)?,
                    u8::from_str_radix(&hex[4..6], 16)?,
                ),
                8 => Rgba::<u8>::new(
                    u8::from_str_radix(&hex[0..2], 16)?,
                    u8::from_str_radix(&hex[2..4], 16)?,
                    u8::from_str_radix(&hex[4..6], 16)?,
                    u8::from_str_radix(&hex[6..8], 16)?,
                ),
                _ => anyhow::bail!("Expected 3, 4, 6 or 8 hex digits"),
            }
            .convert());
        }
        Ok(match value {
            "white" => Self::WHITE,
            "black" => Self::BLACK,
            "gray" => Self::GRAY,
            "red" => Self::RED,
            "green" => Self::GREEN,
            "blue" => Self::BLUE,
            "cyan" => Self::CYAN,
            "magenta" => Self::MAGENTA,
            "yellow" => Self::YELLOW,
            _ => anyhow::bail!("Incorrect color format"),
        })
    }
}

impl<T: ColorComponent> std::fmt::Display for Rgba<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let color: Rgba<u8> = self.convert();
        write!(fmt, "#{:02x}{:02x}{:02x}", color.r, color.g, color.b)?;
        if color.a != u8::MAX {
            write!(fmt, "{:02x}", color.a)?;
        }
        Ok(())
    }
}

impl<T: ColorComponent + Approx> Approx for Rgba<T> {
    fn approx_distance_to(&self, other: &Self) -> f32 {
        (self.r.approx_distance_to(&other.r)
            + self.g.approx_distance_to(&other.g)
            + self.b.approx_distance_to(&other.b)
            + self.a.approx_distance_to(&other.a))
            / 4.0
    }
}

#[test]
fn test_display() {
    assert_eq!(
        Rgba::<f32>::new(0.1, 0.2, 0.3, 0.4).to_string(),
        "#19334c66"
    );
    assert_eq!(Rgba::<f32>::opaque(0.1, 0.2, 0.3).to_string(), "#19334c");
}

impl<T: ColorComponent + PartialEq> PartialEq for Rgba<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl<T: ColorComponent + Eq> Eq for Rgba<T> {}

impl<T: ColorComponent> Rgba<T> {
    /// Construct `Rgba` from red, green, and blue components, fully opaque (max alpha).
    pub fn opaque(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::MAX }
    }

    /// Construct `Rgba` from red, green, blue, and alpha components.
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    /// Convert `Rgba<T>` to `Rgba<U>` by applying a function to every color component excluding alpha.
    /// The resulting alpha is calculated by applying ColorComponent::convert() method.
    /// # Examples
    /// ```
    /// # use batbox_color::*;
    /// let initial = Rgba::new(0.7, 0.4, 1.0, 1.0);
    /// let f = |component: f32| component / 2.0;
    /// assert_eq!(initial.map_rgb(f), Rgba::new(0.35, 0.2, 0.5, 1.0));
    /// ```
    pub fn map_rgb<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Rgba<U> {
        Rgba {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: self.a.convert(),
        }
    }

    /// Convert `Rgba<T>` to `Rgba<U>` by applying a function to every color component.
    /// # Examples
    /// ```
    /// # use batbox_color::*;
    /// let initial = Rgba::new(0.7, 0.4, 1.0, 1.0);
    /// let f = |component: f32| component / 2.0;
    /// assert_eq!(initial.map(f), Rgba::new(0.35, 0.2, 0.5, 0.5));
    /// ```
    pub fn map<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Rgba<U> {
        Rgba {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: f(self.a),
        }
    }

    /// Applies a function to every component of two colors and produces a new color.
    /// # Examples
    /// ```
    /// # use batbox_color::*;
    /// let a = Rgba::new(0.2, 0.1, 0.3, 0.6);
    /// let b = Rgba::new(0.5, 0.3, 0.2, 0.2);
    /// let f = |a: f32, b: f32| a + b;
    /// assert_eq!(a.zip_map(b, f), Rgba::new(0.7, 0.4, 0.5, 0.8));
    /// ```
    pub fn zip_map<F: Fn(T, U) -> V, U: ColorComponent, V: ColorComponent>(
        self,
        other: Rgba<U>,
        f: F,
    ) -> Rgba<V> {
        Rgba {
            r: f(self.r, other.r),
            g: f(self.g, other.g),
            b: f(self.b, other.b),
            a: f(self.a, other.a),
        }
    }

    /// Convert `Rgba<T>` to `Rgba<U>` by applying `ColorComponent::convert()` method.
    /// # Examples
    /// ```
    /// # use batbox_color::*;
    /// assert_eq!(Rgba::opaque(0, 255, 0).convert(), Rgba::opaque(0.0, 1.0, 0.0));
    /// ```
    pub fn convert<U: ColorComponent>(self) -> Rgba<U> {
        self.map(|component| component.convert())
    }

    /// Linearly interpolate between `start` and `end` values.
    ///
    /// # Examples
    /// ```
    /// # use batbox_color::*;
    /// let start = Rgba::opaque(0.0, 0.0, 0.0);
    /// let end = Rgba::opaque(1.0, 1.0, 1.0);
    /// let interpolated = Rgba::lerp(start, end, 0.3);
    /// assert!(interpolated.r - 0.3 < 1e-5);
    /// assert!(interpolated.g - 0.3 < 1e-5);
    /// assert!(interpolated.b - 0.3 < 1e-5);
    /// assert_eq!(interpolated.a, 1.0);
    /// ```
    pub fn lerp(start: Self, end: Self, t: f32) -> Self {
        start.zip_map(end, |start, end| T::lerp(start, end, t))
    }
}

#[test]
fn test_convert() {
    assert_eq!(
        Rgba::opaque(1.0, 0.0, 0.5).convert::<u8>(),
        Rgba::opaque(0xff, 0, 0x7f)
    );
}

impl<T: ColorComponent> std::ops::Deref for Rgba<T> {
    type Target = [T; 4];
    fn deref(&self) -> &[T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: ColorComponent> std::ops::DerefMut for Rgba<T> {
    fn deref_mut(&mut self) -> &mut [T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

#[test]
fn test_deref() {
    let color = Rgba::opaque(1, 2, 3);
    assert_eq!(color[0], 1);
    assert_eq!(color[1], 2);
    assert_eq!(color[2], 3);
    assert_eq!(color[3], 0xff);
}

#[test]
fn test_deref_mut() {
    let mut color = Rgba::<f32>::opaque(0.0, 0.5, 1.0);
    color[0] = 1.0;
    color[1] = 0.3;
    color[2] = 0.7;
    color[3] = 0.1;
    assert!(color.r.approx_eq(&1.0));
    assert!(color.g.approx_eq(&0.3));
    assert!(color.b.approx_eq(&0.7));
    assert!(color.a.approx_eq(&0.1));
}
