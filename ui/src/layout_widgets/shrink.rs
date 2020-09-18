use super::*;

pub struct Shrink<T> {
    core: WidgetCore,
    ratio: f64,
    child: T,
}

impl<T, R: AsRef<T>> AsRef<T> for Shrink<R> {
    fn as_ref(&self) -> &T {
        self.child.as_ref()
    }
}

impl<T, R: AsMut<T>> AsMut<T> for Shrink<R> {
    fn as_mut(&mut self) -> &mut T {
        self.child.as_mut()
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn shrink(self, ratio: f64) -> Shrink<Self> {
            Shrink {
                core: WidgetCore::void(),
                ratio,
                child: self,
            }
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget> Widget for Shrink<T> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core_mut().constraints = self.child.core().constraints;
    }
    fn layout_children(&mut self) {
        let pos = self.core().position;
        let ratio = self.ratio / 2.0;
        self.child.core_mut().position = AABB {
            x_min: pos.x_min + pos.width() * ratio,
            x_max: pos.x_max - pos.width() * ratio,
            y_min: pos.y_min + pos.height() * ratio,
            y_max: pos.y_max - pos.height() * ratio,
        }
    }
    fn walk_children_mut<'a>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        f(&mut self.child);
    }
}
