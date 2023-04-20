use super::*;

pub struct Padding<T> {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
    child: T,
}

impl<T, R: AsRef<T>> AsRef<T> for Padding<R> {
    fn as_ref(&self) -> &T {
        self.child.as_ref()
    }
}

impl<T, R: AsMut<T>> AsMut<T> for Padding<R> {
    fn as_mut(&mut self) -> &mut T {
        self.child.as_mut()
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn padding(self, top: f64, right: f64, bottom: f64, left: f64) -> Padding<Self> {
            Padding {
                top,
                right,
                bottom,
                left,
                child: self,
            }
        }
        fn uniform_padding(self, padding: f64) -> Padding<Self> {
            self.padding(padding, padding, padding, padding)
        }
        fn padding_top(self, padding: f64) -> Padding<Self> {
            self.padding(padding, 0.0, 0.0, 0.0)
        }
        fn padding_right(self, padding: f64) -> Padding<Self> {
            self.padding(0.0, padding, 0.0, 0.0)
        }
        fn padding_bottom(self, padding: f64) -> Padding<Self> {
            self.padding(0.0, 0.0, padding, 0.0)
        }
        fn padding_left(self, padding: f64) -> Padding<Self> {
            self.padding(0.0, 0.0, 0.0, padding)
        }
        fn padding_horizontal(self, padding: f64) -> Padding<Self> {
            self.padding(0.0, padding, 0.0, padding)
        }
        fn padding_vertical(self, padding: f64) -> Padding<Self> {
            self.padding(padding, 0.0, padding, 0.0)
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget> Widget for Padding<T> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        let mut result = children.get_constraints(&self.child);
        result.min_size += vec2(self.left + self.right, self.bottom + self.top);
        result
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        cx.set_position(
            &self.child,
            cx.position
                .extend_left(-self.left)
                .extend_right(-self.right)
                .extend_down(-self.bottom)
                .extend_up(-self.top),
        )
    }
    fn walk_children_mut<'a>(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
    }
}
