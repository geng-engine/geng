use crate::*;

mod component;
mod consts;

pub use component::*;
pub use consts::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Trans, Schematic)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
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

impl<T: ColorComponent + Eq> PartialEq for Color<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl<T: ColorComponent + Eq> Eq for Color<T> {}

impl<T: ColorComponent> Color<T> {
    pub fn rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::MAX }
    }
    pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
    pub fn convert<U: ColorComponent>(self) -> Color<U> {
        Color {
            r: self.r.convert(),
            g: self.g.convert(),
            b: self.b.convert(),
            a: self.a.convert(),
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
