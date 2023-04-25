use super::*;

impl<T: ColorComponent> Rgba<T> {
    /// <span style="background-color:black;color:white">#FFF</span>
    pub const WHITE: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::MAX,
        a: T::MAX,
    };
    /// <span style="background-color:white;color:black">#000</span>
    pub const BLACK: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::ZERO,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#7F7F7F">#7F7F7F</span>
    pub const GRAY: Self = Self {
        r: T::HALF,
        g: T::HALF,
        b: T::HALF,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:white">rgba(255, 255, 255, 0)</span>
    pub const TRANSPARENT_WHITE: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::MAX,
        a: T::ZERO,
    };
    /// <span style="background-color:white;color:black">rgba(0, 0, 0, 0)</span>
    pub const TRANSPARENT_BLACK: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::ZERO,
        a: T::ZERO,
    };
    /// <span style="background-color:black;color:#F00">#F00</span>
    pub const RED: Self = Self {
        r: T::MAX,
        g: T::ZERO,
        b: T::ZERO,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#0F0">#0F0</span>
    pub const GREEN: Self = Self {
        r: T::ZERO,
        g: T::MAX,
        b: T::ZERO,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#00F">#00F</span>
    pub const BLUE: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::MAX,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#0FF">#0FF</span>
    pub const CYAN: Self = Self {
        r: T::ZERO,
        g: T::MAX,
        b: T::MAX,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#F0F">#F0F</span>
    pub const MAGENTA: Self = Self {
        r: T::MAX,
        g: T::ZERO,
        b: T::MAX,
        a: T::MAX,
    };
    /// <span style="background-color:black;color:#FF0">#FF0</span>
    pub const YELLOW: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::ZERO,
        a: T::MAX,
    };
}

#[test]
fn test_consts_stable() {
    macro_rules! test_stable {
        ($($name:ident,)*) => {
            $(
                assert_eq!(Rgba::<f32>::$name.convert::<u8>(), Rgba::<u8>::$name);
                assert!(Rgba::<f32>::$name.convert::<f64>().approx_eq(&Rgba::<f64>::$name));
                assert_eq!(Rgba::<f64>::$name.convert::<u8>(), Rgba::<u8>::$name);
                assert!(Rgba::<f64>::$name.convert::<f32>().approx_eq(&Rgba::<f32>::$name));
                assert!(Rgba::<u8>::$name.convert::<f32>().approx_eq_eps(&Rgba::<f32>::$name, 1.0 / 255.0));
                assert!(Rgba::<u8>::$name.convert::<f64>().approx_eq_eps(&Rgba::<f64>::$name, 1.0 / 255.0));
            )*
        };
    }
    test_stable!(
        WHITE,
        BLACK,
        GRAY,
        TRANSPARENT_WHITE,
        TRANSPARENT_BLACK,
        RED,
        GREEN,
        BLUE,
        CYAN,
        MAGENTA,
        YELLOW,
    );
}
