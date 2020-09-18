use super::*;

pub struct Padding<T> {
    core: WidgetCore,
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
                core: WidgetCore::void(),
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
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget> Widget for Padding<T> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core_mut().constraints = self.child.core().constraints;
        let extra_size = vec2(self.left + self.right, self.bottom + self.top);
        self.core_mut().constraints.min_size += extra_size;
    }
    fn layout_children(&mut self) {
        self.child.core_mut().position = AABB {
            x_min: self.core().position.x_min + self.left,
            x_max: self.core().position.x_max - self.right,
            y_min: self.core().position.y_min + self.bottom,
            y_max: self.core().position.y_max - self.top,
        }
    }
    fn walk_children_mut<'a>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        f(&mut self.child);
    }
}
