use super::*;

mod component;
mod consts;

pub use component::*;
pub use consts::*;

/// RGBA Color
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Color<T> {
    /// Red component
    pub r: T,
    /// Green component
    pub g: T,
    /// Blue component
    pub b: T,
    /// Alpha (opacity) component
    pub a: T,
}

impl<T: ColorComponent> Display for Color<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let color: Color<u8> = self.convert();
        write!(fmt, "#{:02x}{:02x}{:02x}", color.r, color.g, color.b)?;
        if color.a != u8::MAX {
            write!(fmt, "{:02x}", color.a)?;
        }
        Ok(())
    }
}

impl<T: ColorComponent + ApproxEq> ApproxEq for Color<T> {
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
        Color::<f32>::rgba(0.1, 0.2, 0.3, 0.4).to_string(),
        "#19334c66"
    );
    assert_eq!(Color::<f32>::rgb(0.1, 0.2, 0.3).to_string(), "#19334c");
}

impl<T: ColorComponent + PartialEq> PartialEq for Color<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl<T: ColorComponent + Eq> Eq for Color<T> {}

impl<T: ColorComponent> Color<T> {
    /// Construct `Color` from red, green, and blue components.
    pub fn rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::MAX }
    }

    /// Construct `Color` from red, green, blue, and alpha components.
    pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    /// Convert `Color<T>` to `Color<U>` by applying a function to every color component excluding alpha.
    /// The resulting alpha is calculated by applying ColorComponent::convert() method.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let initial = Color::rgba(0.7, 0.4, 1.0, 1.0);
    /// let f = |component: f32| component / 2.0;
    /// assert_eq!(initial.map_color(f), Color::rgba(0.35, 0.2, 0.5, 1.0));
    /// ```
    pub fn map_color<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Color<U> {
        Color {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: self.a.convert(),
        }
    }

    /// Convert `Color<T>` to `Color<U>` by applying a function to every color component.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let initial = Color::rgba(0.7, 0.4, 1.0, 1.0);
    /// let f = |component: f32| component / 2.0;
    /// assert_eq!(initial.map(f), Color::rgba(0.35, 0.2, 0.5, 0.5));
    /// ```
    pub fn map<F: Fn(T) -> U, U: ColorComponent>(self, f: F) -> Color<U> {
        Color {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
            a: f(self.a),
        }
    }

    /// Convert `Color<T>` to `Color<U>` by applying `ColorComponent::convert()` method.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(Color::rgb(0, 255, 0).convert(), Color::rgb(0.0, 1.0, 0.0));
    /// ```
    pub fn convert<U: ColorComponent>(self) -> Color<U> {
        self.map(|component| component.convert())
    }

    /// Linearly interpolate between `start` and `end` values.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let start = Color::rgb(0.0, 0.0, 0.0);
    /// let end = Color::rgb(1.0, 1.0, 1.0);
    /// let interpolated = Color::lerp(start, end, 0.3);
    /// assert!(interpolated.r - 0.3 < 1e-5);
    /// assert!(interpolated.g - 0.3 < 1e-5);
    /// assert!(interpolated.b - 0.3 < 1e-5);
    /// assert_eq!(interpolated.a, 1.0);
    /// ```
    pub fn lerp(start: Self, end: Self, t: f32) -> Self {
        Self {
            r: T::lerp(start.r, end.r, t),
            g: T::lerp(start.g, end.g, t),
            b: T::lerp(start.b, end.b, t),
            a: T::lerp(start.a, end.a, t),
        }
    }
}

#[test]
fn test_convert() {
    assert_eq!(
        Color::rgb(1.0, 0.0, 0.5).convert::<u8>(),
        Color::rgb(0xff, 0, 0x7f)
    );
}

impl<T: ColorComponent> Deref for Color<T> {
    type Target = [T; 4];
    fn deref(&self) -> &[T; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<T: ColorComponent> DerefMut for Color<T> {
    fn deref_mut(&mut self) -> &mut [T; 4] {
        unsafe { mem::transmute(self) }
    }
}

#[test]
fn test_deref() {
    let color = Color::rgb(1, 2, 3);
    assert_eq!(color[0], 1);
    assert_eq!(color[1], 2);
    assert_eq!(color[2], 3);
    assert_eq!(color[3], 0xff);
}

#[test]
fn test_deref_mut() {
    let mut color = Color::<f32>::rgb(0.0, 0.5, 1.0);
    color[0] = 1.0;
    color[1] = 0.3;
    color[2] = 0.7;
    color[3] = 0.1;
    assert!(color.r.approx_eq(&1.0));
    assert!(color.g.approx_eq(&0.3));
    assert!(color.b.approx_eq(&0.7));
    assert!(color.a.approx_eq(&0.1));
}
